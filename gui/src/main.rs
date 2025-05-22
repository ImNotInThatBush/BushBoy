use minifb::{Key, Window, WindowOptions};
use emulator_core::Emulator;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    let mut window = Window::new(
        "BushBoy Emulator - Debug",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut emulator = Emulator::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let keys = window.get_keys();
        let keys_opt = if keys.is_empty() { None } else { Some(keys) };
        emulator.handle_keys(&keys_opt);

        // Debug print subito dopo handle_keys
        println!("DEBUG: Keys: {:?}", keys_opt);

        emulator.tick();
        emulator.render(&mut buffer);
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
