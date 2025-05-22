use std::fs::File;
use std::io::Read;

use emulator_core::{Emulator, memory::Memory};
use minifb::{Window, WindowOptions, Key};

fn main() {
    let rom_path = "../snake.gb";
    let mut file = File::open(rom_path).expect("Failed to open ROM file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read ROM");

    let memory = Memory::new(buffer);
    let mut emulator = Emulator::new();

    let width = 160;
    let height = 144;
    let mut window = Window::new(
        "BushBoy Emulator",
        width,
        height,
        WindowOptions::default(),
    ).expect("Failed to create window");

    let mut screen_buffer = vec![0u32; width * height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let keys = window.get_keys();
        emulator.handle_keys(&keys);
        emulator.tick();
        emulator.render(&mut screen_buffer);
        window
            .update_with_buffer(&screen_buffer, width, height)
            .expect("Failed to update window");
    }
}
