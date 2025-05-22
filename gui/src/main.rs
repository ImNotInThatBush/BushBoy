use minifb::{Key, Window, WindowOptions};
use emulator_core::Emulator;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    let mut window = Window::new(
        "Bush Boy Emulator",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut emulator = Emulator::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        emulator.tick();
        emulator.render(&mut buffer);

        // Passa &Option<Vec<Key>> come richiesto da handle_keys
        let keys = window.get_keys();
        emulator.handle_keys(&keys);

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
