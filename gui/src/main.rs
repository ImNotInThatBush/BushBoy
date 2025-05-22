use std::fs::File;
use std::io::Read;

use emulator_core::{cpu::Cpu, memory::Memory};

fn main() {
    let rom_path = "../snake.gb";

    let mut file = File::open(rom_path).expect("Failed to open ROM file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read ROM");

    let mut memory = Memory::new();
    memory.load_rom(&buffer);

    let mut cpu = Cpu::new();
    cpu.run(&mut memory);
}
