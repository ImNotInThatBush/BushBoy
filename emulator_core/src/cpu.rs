use crate::memory::Memory;

pub struct CPU {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
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

    fn set_flag(&mut self, bit: u8, value: bool) {
        if value {
            self.f |= 1 << bit;
        } else {
            self.f &= !(1 << bit);
        }
    }

    pub fn step(&mut self, mem: &mut Memory) {
        let opcode = mem.read_byte(self.pc);
        println!("Executing opcode: {:02X} at PC: {:04X}", opcode, self.pc);

        match opcode {
            0x00 => {
                println!("NOP");
                self.pc += 1;
            }
            0x01 => {
                let lo = mem.read_byte(self.pc + 1);
                let hi = mem.read_byte(self.pc + 2);
                self.c = lo;
                self.b = hi;
                println!("LD BC, ${:02X}{:02X}", self.b, self.c);
                self.pc += 3;
            }
            0x04 => {
                let prev = self.b;
                self.b = self.b.wrapping_add(1);
                self.set_flag(7, self.b == 0);
                self.set_flag(6, false);
                self.set_flag(5, (prev & 0x0F) + 1 > 0x0F);
                println!("INC B -> {:02X}", self.b);
                self.pc += 1;
            }
            0x11 => {
                let lo = mem.read_byte(self.pc + 1);
                let hi = mem.read_byte(self.pc + 2);
                self.e = lo;
                self.d = hi;
                println!("LD DE, ${:02X}{:02X}", self.d, self.e);
                self.pc += 3;
            }
            0x21 => {
                let lo = mem.read_byte(self.pc + 1);
                let hi = mem.read_byte(self.pc + 2);
                self.l = lo;
                self.h = hi;
                println!("LD HL, ${:02X}{:02X}", self.h, self.l);
                self.pc += 3;
            }
            0x31 => {
                let lo = mem.read_byte(self.pc + 1);
                let hi = mem.read_byte(self.pc + 2);
                self.sp = ((hi as u16) << 8) | lo as u16;
                println!("LD SP, ${:04X}", self.sp);
                self.pc += 3;
            }
            0x2A => {
                let addr = ((self.h as u16) << 8) | self.l as u16;
                self.a = mem.read_byte(addr);
                let hl = addr.wrapping_add(1);
                self.h = (hl >> 8) as u8;
                self.l = hl as u8;
                println!("LD A, (HL+) from ${:04X} = {:02X}", addr, self.a);
                self.pc += 1;
            }
            0x77 => {
                let addr = ((self.h as u16) << 8) | self.l as u16;
                mem.write_byte(addr, self.a);
                println!("LD (HL), A -> ${:04X} = {:02X}", addr, self.a);
                self.pc += 1;
            }
            0xEA => {
                let lo = mem.read_byte(self.pc + 1);
                let hi = mem.read_byte(self.pc + 2);
                let addr = ((hi as u16) << 8) | lo as u16;
                mem.write_byte(addr, self.a);
                println!("LD (${:04X}), A -> Wrote {:02X}", addr, self.a);
                self.pc += 3;
            }
            0xFA => {
                let lo = mem.read_byte(self.pc + 1);
                let hi = mem.read_byte(self.pc + 2);
                let addr = ((hi as u16) << 8) | lo as u16;
                self.a = mem.read_byte(addr);
                println!("LD A, (${:04X}) -> {:02X}", addr, self.a);
                self.pc += 3;
            }
            0xE0 => {
                let offset = mem.read_byte(self.pc + 1);
                let addr = 0xFF00 + offset as u16;
                mem.write_byte(addr, self.a);
                println!("LD (${:04X}), A -> {:02X}", addr, self.a);
                self.pc += 2;
            }
            0xF0 => {
                let offset = mem.read_byte(self.pc + 1);
                let addr = 0xFF00 + offset as u16;
                self.a = mem.read_byte(addr);
                println!("LD A, (${:04X}) -> {:02X}", addr, self.a);
                self.pc += 2;
            }
            0xC3 => {
                let lo = mem.read_byte(self.pc + 1);
                let hi = mem.read_byte(self.pc + 2);
                let addr = ((hi as u16) << 8) | lo as u16;
                println!("JP ${:04X}", addr);
                self.pc = addr;
            }
            0xCD => {
                let lo = mem.read_byte(self.pc + 1);
                let hi = mem.read_byte(self.pc + 2);
                let addr = ((hi as u16) << 8) | lo as u16;
                let return_addr = self.pc + 3;
                self.sp -= 2;
                mem.write_byte(self.sp, (return_addr & 0xFF) as u8);
                mem.write_byte(self.sp + 1, (return_addr >> 8) as u8);
                println!("CALL ${:04X} (return to ${:04X})", addr, return_addr);
                self.pc = addr;
            }
            0xC9 => {
                let lo = mem.read_byte(self.sp);
                let hi = mem.read_byte(self.sp + 1);
                self.sp += 2;
                let addr = ((hi as u16) << 8) | lo as u16;
                println!("RET to ${:04X}", addr);
                self.pc = addr;
            }
            _ => {
                println!("Unknown opcode: {:02X}", opcode);
                self.pc += 1;
            }
        }
    }

    pub fn run(&mut self, mem: &mut Memory, steps: usize) {
        for _ in 0..steps {
            self.step(mem);
        }
    }
}
