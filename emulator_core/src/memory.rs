pub struct Memory {
    pub rom: Vec<u8>,
    pub ram: [u8; 0x2000], // solo un esempio di RAM Work RAM
}

impl Memory {
    pub fn new(rom: Vec<u8>) -> Self {
        Memory {
            rom,
            ram: [0; 0x2000],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            0xC000..=0xDFFF => self.ram[(addr - 0xC000) as usize],
            _ => 0xFF, // default
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0xC000..=0xDFFF => self.ram[(addr - 0xC000) as usize] = val,
            _ => {}, // ROM non scrivibile
        }
    }
}
