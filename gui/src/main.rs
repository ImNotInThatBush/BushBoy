use emulator_core::{CPU, Memory};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("snake.gb").expect("ROM non trovata");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Errore lettura ROM");

    let mut mem = Memory::new(buffer);
    let mut cpu = CPU::new();

    cpu.run(&mut mem, 500);
    cpu.debug();
}
