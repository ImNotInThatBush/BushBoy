use crate::memory::Memory;

pub struct CPU {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8, // flags
    pub h: u8,
    pub l: u8,
    pub pc: u16, // program counter
    pub sp: u16, // stack pointer
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    pub fn debug(&self) {
        println!("CPU Registers:");
        println!(
            "A:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} F:{:02X} H:{:02X} L:{:02X}",
            self.a, self.b, self.c, self.d, self.e, self.f, self.h, self.l
        );
        println!("PC:{:04X} SP:{:04X}", self.pc, self.sp);
    }

    pub fn step(&mut self, mem: &Memory) {
        let opcode = mem.read_byte(self.pc);
        println!("Executing opcode: {:02X} at PC: {:04X}", opcode, self.pc);

        match opcode {
            0x00 => { // NOP
                println!("NOP");
                self.pc += 1;
            }
            0x31 => { // LD SP, nn
                let lo = mem.read_byte(self.pc + 1);
                let hi = mem.read_byte(self.pc + 2);
                self.sp = ((hi as u16) << 8) | lo as u16;
                println!("LD SP, ${:04X}", self.sp);
                self.pc += 3;
            }
            _ => {
                println!("Unknown opcode: {:02X}", opcode);
                self.pc += 1;
            }
        }
    }

    pub fn run(&mut self, mem: &Memory, steps: usize) {
        for _ in 0..steps {
            self.step(mem);
        }
    }
}
