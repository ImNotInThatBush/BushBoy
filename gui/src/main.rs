use emulator_core::Emulator;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut window = Window::new(
        "BushBoy Emulator",
        160,
        144,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));

    let mut buffer: Vec<u32> = vec![0; 160 * 144];
    let mut emulator = Emulator::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let keys = window.get_keys();
        emulator.handle_keys(&Some(keys));
        emulator.tick();
        emulator.render(&mut buffer);
        window.update_with_buffer(&buffer, 160, 144).unwrap();
    }
}
