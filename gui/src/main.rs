use emulator_core::Emulator;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut emulator = Emulator::new();
    let mut window = Window::new("BushBoy Emulator", 160, 144, WindowOptions::default())
        .expect("Unable to open Window");

    while window.is_open() {
        let keys = window.get_keys();
        emulator.handle_keys(&keys);

        let buffer = emulator.get_frame_buffer();
        window
            .update_with_buffer(&buffer, 160, 144)
            .expect("Failed to update window");
    }
}
