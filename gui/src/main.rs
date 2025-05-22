use emulator_core::{CPU, Memory};
use std::fs::File;
use std::io::Read;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    let mut file = File::open("snake.gb").expect("ROM non trovata");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Errore lettura ROM");

    let mut mem = Memory::new(buffer);
    let mut cpu = CPU::new();

    let mut window = Window::new("BushBoy - Snake", WIDTH, HEIGHT, WindowOptions::default())
        .expect("Impossibile creare finestra");

    let mut pixels = vec![0x00_00_00; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Input Game Boy
        let keys = window.get_keys().unwrap_or_else(Vec::new);

        let mut joypad = 0xFF;

        if keys.contains(&Key::Right) || keys.contains(&Key::D) {
            joypad &= !(1 << 0); // Right
        }
        if keys.contains(&Key::Left) || keys.contains(&Key::A) {
            joypad &= !(1 << 1); // Left
        }
        if keys.contains(&Key::Up) || keys.contains(&Key::W) {
            joypad &= !(1 << 2); // Up
        }
        if keys.contains(&Key::Down) || keys.contains(&Key::S) {
            joypad &= !(1 << 3); // Down
        }
        if keys.contains(&Key::Enter) {
            joypad &= !(1 << 4); // Start
        }
        if keys.contains(&Key::Space) {
            joypad &= !(1 << 5); // Select
        }
        if keys.contains(&Key::Z) {
            joypad &= !(1 << 6); // B
        }
        if keys.contains(&Key::X) {
            joypad &= !(1 << 7); // A
        }

        mem.write_byte(0xFF00, joypad);

        cpu.run(&mut mem, 10);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let addr = 0x8000 + (y * WIDTH + x) % 0x2000;
                let val = mem.read_byte(addr as u16);
                let color = match val {
                    0 => 0xFF_FF_FF,
                    1..=63 => 0xAA_AA_AA,
                    64..=127 => 0x55_55_55,
                    _ => 0x00_00_00,
                };
                pixels[y * WIDTH + x] = color;
            }
        }

        window.update_with_buffer(&pixels, WIDTH, HEIGHT).unwrap();
    }
}
