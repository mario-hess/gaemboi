use std::{
    cell::RefCell,
    error::Error,
    rc::Rc,
    time::Instant,
};

use egui_sdl2_gl::{
    egui::Context,
    painter::Painter,
    sdl2::{video::Window, AudioSubsystem, EventPump},
    EguiStateHandler,
};

use crate::{
    apu::audio::create_audio_device,
    cpu::{
        clock::{Clock, CYCLES_PER_FRAME},
        Cpu,
    },
    event_handler::EventHandler,
    ppu::colors::Colors,
    sync_bridge::SyncBridge,
    ui::UIManager,
};

pub trait MemoryAccess {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
}

pub trait ComponentTick {
    fn tick(&mut self, m_cycles: u8);
}

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
        fast_forward: Rc<RefCell<u8>>,
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
        let mut sync_bridge = SyncBridge::new();

        let audio_device = create_audio_device(
            audio_subsystem,
            self.cpu.memory_bus.apu.master_volume.get_left_volume(),
            self.cpu.memory_bus.apu.master_volume.get_right_volume(),
            &event_handler.volume,
            self.cpu.memory_bus.apu.audio_buffer.clone(),
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

            if self.last_second.elapsed().as_secs() >= 1 {
                self.fps = self.frame_count as f32 / self.frame_times.iter().sum::<f32>();
                self.frame_times.clear();
                self.frame_count = 0;
                self.last_second = std::time::Instant::now();
            }

            sync_bridge.sync(
                &frame_start_time,
                &fast_forward,
                event_handler.performance_mode,
                self.cpu.memory_bus.apu.enabled,
                self.cpu.memory_bus.apu.audio_buffer.clone(),
            );

            let frame_time = frame_start_time.elapsed().as_secs_f32();
            self.frame_times.push(frame_time);
            self.frame_count += 1;

            if event_handler.file_path.is_some() {
                break;
            }
        }
    }
}
