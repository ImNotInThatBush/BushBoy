use minifb::Key;

pub struct Emulator;

impl Emulator {
    pub fn new() -> Self {
        Emulator
    }

    // Fase base: stub CPU cycle (da espandere in seguito)
    pub fn tick(&mut self) {
        // TODO: implement CPU cycle
        // Per ora stampa "tick"
        // println!("tick"); // Decommenta per debug, commenta per uso normale
    }

    pub fn render(&self, buffer: &mut [u32]) {
        // Disegna schermo tutto blu (BGR)
        for pixel in buffer.iter_mut() {
            *pixel = 0x00_00_FF;
        }
    }

    pub fn handle_keys(&mut self, _keys: &Option<Vec<Key>>) {
        // TODO: handle keyboard input
    }
}
