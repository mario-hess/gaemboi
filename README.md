# gemboi
## ˈʤɛmˌbɔɪ

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

This is a Nintendo Game Boy emulator written in Rust. It aims to accurately emulate the functionality and behavior of the original Game Boy hardware.

## Features

- [x] Emulation of Game Boy CPU (Sharp LR35902)
- [x] Precise timing based on instruction cycles and timing registers 
- [ ] Support for Game Boy ROMs
- [ ] Accurate emulation of memory, registers, and interrupts
- [ ] Basic graphics
- [ ] Audio emulation (DMG sound channels)
- [ ] Input handling
- [ ] Save states and battery-backed save support

## Blargg's test ROMs

### CPU

- [x] 01-special.gb
- [x] 02-interrupts.gb
- [x] 03-op sp,hl.gb
- [x] 04-op r,imm.gb
- [x] 05-op rp.gb
- [x] 06-ld r,r.gb
- [x] 07-jr,jp,call,ret,rst.gb
- [x] 08-misc instrs.gb
- [x] 09-op r,r.gb
- [x] 10-bit ops.gb
- [x] 11-op a,(hl).gb

### Timing

- [x] instr_timing.gb

## Getting Started

### Prerequisites

- Rust (https://www.rust-lang.org/tools/install)

### Development

Clone the repository:
```
git clone https://github.com/mario-hess/gemboi
```

### Usage

Create a folder named 'roms' in the root directory, and place your rom in there.
Run the emulator with a specified ROM file:
```
cargo run --release -- <rom_file_name>
```
Replace <rom_file_name> with the name of your Game Boy ROM file (.gb).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Pan Docs](https://gbdev.io/pandocs/) - Comprehensive Game Boy technical reference
- [Awesome Game Boy Development](https://github.com/avivace/awesome-gbdev) - A curated list of Game Boy development resources
- [Rust Programming Language](https://www.rust-lang.org/) - Official website for the Rust programming language
