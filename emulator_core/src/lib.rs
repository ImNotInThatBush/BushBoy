use minifb::Key;

pub struct Emulator;

impl Emulator {
    pub fn new() -> Self {
        Emulator
    }

    pub fn tick(&mut self) {
        // Stub ciclo CPU
        // println!("tick"); // Debug
    }

    pub fn render(&self, buffer: &mut [u32]) {
        for pixel in buffer.iter_mut() {
            *pixel = 0x00_00_FF;
        }
    }

    // Logica di base: stampa i tasti premuti (solo debug)
    pub fn handle_keys(&mut self, keys: &Option<Vec<Key>>) {
        if let Some(keys) = keys {
            for key in keys {
                println!("Key pressed: {:?}", key);
            }
        }
    }
}
