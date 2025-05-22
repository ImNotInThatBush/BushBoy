pub struct Emulator;

impl Emulator {
    pub fn new() -> Self {
        Emulator
    }

    pub fn tick(&mut self) {
        // TODO: implement CPU cycle
    }

    pub fn render(&self, buffer: &mut [u32]) {
        // TODO: draw to screen buffer
        for pixel in buffer.iter_mut() {
            *pixel = 0x00_00_FF; // blu (BGR format)
        }
    }

    pub fn handle_keys(&mut self, _keys: &Option<Vec<minifb::Key>>) {
        // TODO: handle keyboard input
    }
}
