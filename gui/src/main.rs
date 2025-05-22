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


        window.update_with_buffer(&pixels, WIDTH, HEIGHT).unwrap();
    }
}
