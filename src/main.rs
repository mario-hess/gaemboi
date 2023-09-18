use std::env;
use std::fs::File;
use std::io::Read;

mod cartridge;
mod clock;
mod cpu;
mod instruction;
mod interrupt;
mod machine;
mod memory_bus;
mod ppu;
mod registers;
mod timer;
mod keyboard;

use crate::machine::Machine;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Error: No file path provided.");
    }

    let rom_path = "roms/".to_owned() + &args[1];

    let mut file = File::open(rom_path).expect("Error: Can't open file.");
    let mut rom_data = Vec::new();
    file.read_to_end(&mut rom_data)
        .expect("Error: Can't read file.");

    let mut machine = Machine::new(rom_data);
    machine.run();
}
