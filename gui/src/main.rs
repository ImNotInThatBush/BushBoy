use std::fs::File;
use std::io::Read;

fn main() {
    // Carica la ROM
    let mut file = File::open("snake.gb").expect("ROM file not found");
    let mut rom_data = Vec::new();
    file.read_to_end(&mut rom_data).expect("Failed to read ROM");

    // Memoria deve essere mutabile
    let mut mem = emulator_core::memory::Memory::new(rom_data);
    println!("ROM loaded: {} bytes", mem.rom.len());

    let mut cpu = emulator_core::cpu::CPU::new();

    println!("--- Stato iniziale ---");
    cpu.debug();

    // Passiamo &mut mem
    println!("\n--- Inizio esecuzione ---");
    cpu.run(&mut mem, 10);

    println!("\n--- Stato finale ---");
    cpu.debug();
}
