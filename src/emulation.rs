use std::{
    cell::RefCell,
    collections::VecDeque,
    error::Error,
    rc::Rc,
    sync::{Arc, Mutex},
    time::Instant,
};

use egui_sdl2_gl::{
    egui::Context,
    painter::Painter,
    sdl2::{video::Window, AudioSubsystem, EventPump},
    EguiStateHandler,
};

use crate::{
    apu::{self, AUDIO_BUFFER_THRESHOLD},
    cpu::{
        clock::{Clock, CYCLES_PER_FRAME},
        Cpu,
    },
    event_handler::EventHandler,
    memory_bus::ComponentTick,
    ppu::colors::Colors,
    ui::UIManager,
    FRAME_DURATION, FRAME_DURATION_MICROS,
};

pub struct Emulation {
    pub cpu: Cpu,
    clock: Clock,
    frame_times: Vec<f32>,
    frame_count: u16,
    last_second: Instant,
    fps: f32,
}

impl Emulation {
    pub fn new(
        rom_data: Vec<u8>,
        colors: Rc<RefCell<Colors>>,
        fast_forward: Rc<RefCell<u32>>,
    ) -> Result<Self, Box<dyn Error>> {
        let frame_times = Vec::new();
        let frame_count = 0;
        let last_second = std::time::Instant::now();
        let fps = 0.0;

        Ok(Self {
            cpu: Cpu::new(rom_data, colors, fast_forward)?,
            clock: Clock::new(),
            frame_times,
            frame_count,
            last_second,
            fps,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn run(
        &mut self,
        event_handler: &mut EventHandler,
        start_time: Instant,
        egui_state: &mut EguiStateHandler,
        egui_ctx: &Context,
        event_pump: &mut EventPump,
        window: &mut Window,
        painter: &mut Painter,
        ui_manager: &mut UIManager,
        colors: Rc<RefCell<Colors>>,
        audio_subsystem: &AudioSubsystem,
    ) {
        let audio_device = apu::audio::create_audio_device(
            audio_subsystem,
            &self.cpu.memory_bus.apu.left_volume,
            &self.cpu.memory_bus.apu.right_volume,
            &event_handler.volume,
            &mut self.cpu.memory_bus.apu.audio_buffer,
        );
        audio_device.resume();

        // ---------------- EMULATION LOOP ------------------------
        while !event_handler.quit {
            let frame_start_time = std::time::Instant::now();
            let time = start_time.elapsed().as_secs_f64();
            egui_state.input.time = Some(time);
            egui_ctx.begin_frame(egui_state.input.take());

            event_handler.poll(event_pump, egui_state, window, painter);
            self.cpu.memory_bus.joypad.handle_input(event_handler);

            while self.clock.cycles_passed <= CYCLES_PER_FRAME {
                let m_cycles = self.cpu.step();
                self.cpu.memory_bus.tick(m_cycles);

                if self.cpu.memory_bus.ppu.should_draw {
                    ui_manager.draw(
                        egui_ctx,
                        egui_state,
                        painter,
                        window,
                        event_handler,
                        colors.clone(),
                        &mut self.cpu,
                        &self.fps,
                    );

                    self.cpu.memory_bus.ppu.should_draw = false;
                    self.cpu.memory_bus.ppu.clear_screen();
                }

                self.clock.tick(m_cycles);
            }

            self.clock.reset();

            let fast_forward = *event_handler.fast_forward.borrow();

            if self.cpu.memory_bus.apu.enabled {
                if event_handler.performance_mode {
                    if should_delay(
                        frame_start_time,
                        &self.cpu.memory_bus.apu.audio_buffer,
                        fast_forward,
                    ) {
                        std::thread::sleep(std::time::Duration::from_micros(
                            FRAME_DURATION_MICROS / fast_forward as u64
                                - frame_start_time.elapsed().as_micros() as u64,
                        ));
                    }
                } else {
                    while should_delay(
                        frame_start_time,
                        &self.cpu.memory_bus.apu.audio_buffer,
                        fast_forward,
                    ) {
                        std::hint::spin_loop();
                    }
                }
            } else {
                std::thread::sleep(std::time::Duration::from_micros(
                    FRAME_DURATION_MICROS / fast_forward as u64
                        - frame_start_time.elapsed().as_micros() as u64,
                ));
            }

            let frame_time = frame_start_time.elapsed().as_secs_f32();
            self.frame_times.push(frame_time);
            self.frame_count += 1;

            if self.last_second.elapsed().as_secs() >= 1 {
                self.fps = self.frame_count as f32 / self.frame_times.iter().sum::<f32>();
                self.frame_times.clear();
                self.frame_count = 0;
                self.last_second = std::time::Instant::now();
            }

            if event_handler.file_path.is_some() {
                break;
            }
        }
    }
}

fn should_delay(
    frame_start_time: std::time::Instant,
    audio_buffer: &Arc<Mutex<VecDeque<u8>>>,
    fast_forward: u32,
) -> bool {
    frame_start_time.elapsed().as_micros() < FRAME_DURATION.as_micros() / fast_forward as u128
        && audio_buffer.lock().unwrap().len() > AUDIO_BUFFER_THRESHOLD
}
