use crate::timer::{DIV, TAC, Timer};

const JOYPAD_INPUT: u16 = 0xFF00;
const SERIAL_SB: u16 = 0xFF01;
const SERIAL_SC: u16 = 0xFF02;
const INTERRUPT_FLAG: u16 = 0xFF0F;
const AUDIO_START: u16 = 0xFF10;
const AUDIO_END: u16 = 0xFF26;
const WAVE_PATTERN_START: u16 = 0xFF30;
const WAVE_PATTERN_END: u16 = 0xFF3F;
const LCD_CONTROL: u16 = 0xFF40;
const LCD_STATUS: u16 = 0xFF41;
const SCROLL_Y: u16 = 0xFF42;
const SCROLL_X: u16 = 0xFF43;
const LINE_Y: u16 = 0xFF44;
const LINE_Y_COMPARE: u16 = 0xFF45;
const DMA: u16 = 0xFF46;
const BACKGROUND_PALETTE: u16 = 0xFF47;
const OBJECT_PALETTE_0: u16 = 0xFF48;
const OBJECT_PALETTE_1: u16 = 0xFF49;
const WINDOW_Y: u16 = 0xFF4A;
const WINDOW_X: u16 = 0xFF4B;
const SPEED_SWITCH: u16 = 0xFF4D;

pub struct IO {
    joypad_input: u8,
    serial_sb: u8,
    serial_sc: u8,
    pub timer: Timer,
    pub interrupt_flag: u8,
    audio: [u8; 23],
    wave_pattern: [u8; 16],
    lcd_control: u8,
    lcd_status: u8,
    scroll_y: u8,
    scroll_x: u8,
    line_y: u8,
    line_y_compare: u8,
    dma: u8,
    background_palette: u8,
    object_palette_0: u8,
    object_palette_1: u8,
    window_y: u8,
    window_x: u8,
    speed_switch: u8,
}

impl IO {
    pub fn new() -> Self {
        Self {
            joypad_input: 0,
            serial_sb: 0,
            serial_sc: 0,
            timer: Timer::new(),
            interrupt_flag: 0,
            audio: [0; 23],
            wave_pattern: [0; 16],
            lcd_control: 0,
            lcd_status: 0,
            scroll_y: 0,
            scroll_x: 0,
            line_y: 0,
            line_y_compare: 0,
            dma: 0,
            background_palette: 0,
            object_palette_0: 0,
            object_palette_1: 0,
            window_y: 0,
            window_x: 0,
            speed_switch: 0,
        }
    }


    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            JOYPAD_INPUT => self.joypad_input,
            SERIAL_SB => self.serial_sb,
            SERIAL_SC => self.serial_sc,
            DIV..= TAC => self.timer.read_byte(address),
            INTERRUPT_FLAG => self.interrupt_flag,
            AUDIO_START..=AUDIO_END => self.audio[address as usize - AUDIO_START as usize],
            WAVE_PATTERN_START..=WAVE_PATTERN_END => {
                self.wave_pattern[address as usize - WAVE_PATTERN_START as usize]
            }
            LCD_CONTROL => self.lcd_control,
            LCD_STATUS => self.lcd_status,
            SCROLL_Y => self.scroll_y,
            SCROLL_X => self.scroll_x,
            LINE_Y => self.line_y,
            LINE_Y_COMPARE => self.line_y_compare,
            DMA => self.dma,
            BACKGROUND_PALETTE => self.background_palette,
            OBJECT_PALETTE_0 => self.object_palette_0,
            OBJECT_PALETTE_1 => self.object_palette_1,
            WINDOW_Y => self.window_y,
            WINDOW_X => self.window_x,
            SPEED_SWITCH => self.speed_switch,
            _ => {
                println!("Unknown serial address: {:#X} Can't read byte.", address);
                0x00
            }
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            JOYPAD_INPUT => self.joypad_input = value,
            SERIAL_SB => self.serial_sb = value,
            SERIAL_SC => self.serial_sc = value,
            DIV..=TAC => self.timer.write_byte(address, value),
            INTERRUPT_FLAG => self.interrupt_flag = value,
            AUDIO_START..=AUDIO_END => self.audio[address as usize - AUDIO_START as usize] = value,
            WAVE_PATTERN_START..=WAVE_PATTERN_END => {
                self.wave_pattern[address as usize - WAVE_PATTERN_START as usize] = value;
            }
            LCD_CONTROL => self.lcd_control = value,
            LCD_STATUS => self.lcd_status = value,
            SCROLL_Y => self.scroll_y = value,
            SCROLL_X => self.scroll_x = value,
            LINE_Y => self.line_y = value,
            LINE_Y_COMPARE => self.line_y_compare = value,
            DMA => self.dma = value,
            BACKGROUND_PALETTE => self.background_palette = value,
            OBJECT_PALETTE_0 => self.object_palette_0 = value,
            OBJECT_PALETTE_1 => self.object_palette_1 = value,
            WINDOW_Y => self.window_y = value,
            WINDOW_X => self.window_x = value,
            SPEED_SWITCH => self.speed_switch = value,
            _ => println!("Unknown serial address: {:#X} Can't write byte: {:#X}.", address, value),
        };
    }
}
