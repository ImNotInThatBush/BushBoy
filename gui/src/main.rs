use std::fs::File;
use std::io::Read;

fn main() {
    // Carica la ROM snake.gb dalla root del repo
    let mut file = File::open("snake.gb").expect("ROM file not found");
    let mut rom_data = Vec::new();
    file.read_to_end(&mut rom_data).expect("Failed to read ROM");

    let mem = emulator_core::memory::Memory::new(rom_data);
    println!("ROM loaded: {} bytes", mem.rom.len());

    let mut cpu = emulator_core::cpu::CPU::new();
    cpu.debug();         // Stato iniziale
    cpu.step(&mem);      // Esegui la prima istruzione (NOP)
    cpu.debug();         // Stato dopo 1 istruzione
}
