use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "BushBoy - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let keys = window.get_keys().unwrap_or_else(|| Vec::new());

        for key in &keys {
            match key {
                Key::Z => println!("B"),
                Key::X => println!("A"),
                Key::Enter => println!("Start"),
                Key::RightShift => println!("Select"),
                Key::Up => println!("↑"),
                Key::Down => println!("↓"),
                Key::Left => println!("←"),
                Key::Right => println!("→"),
                _ => {}
            }
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
