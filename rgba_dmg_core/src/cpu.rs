// cpu.rs --- 
// 
// Filename: cpu.rs
// Author: Louise <louise>
// Created: Wed Dec  6 14:46:30 2017 (+0100)
// Last-Updated: Mon Jul  1 13:14:48 2019 (+0200)
//           By: Louise <ludwigette>
// 
use ::Interconnect;
use std::fmt;

#[derive(Debug)]
pub struct LR35902 {
    a: u8,
    
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    zero: bool,
    sub: bool,
    half: bool,
    carry: bool,

    sp: u16,
    pc: u16,

    ime: bool,
    halt: bool,
}

impl LR35902 {
    pub fn new() -> LR35902 {
        LR35902 {
            a: 0,
        
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            
            zero: false,
            sub: false,
            half: false,
            carry: false,
            
            sp: 0,
            pc: 0,

            ime: false,
            halt: false
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.ime = false;
        self.halt = false;
    }

    #[inline]
    /// Returns the Program Counter
    pub fn pc(&self) -> usize { self.pc as usize }

    #[inline]
    /// Jumps to given address (and delay)
    fn load_pc(&mut self, io: &mut Interconnect, pc: u16) {
        io.delay(1);

        self.pc = pc;
    }

    #[inline]
    /// Jump
    fn jp(&mut self, io: &mut Interconnect) {
        let pc = self.next_u16(io);
        self.load_pc(io, pc);
    }

    #[inline]
    /// Jump if cond
    fn jp_cond(&mut self, io: &mut Interconnect, cond: bool) {
        let pc = self.next_u16(io);

        if cond {
            self.load_pc(io, pc);
        }
    }
    
    #[inline]
    /// Relative jump
    fn jr(&mut self, io: &mut Interconnect) {
        let value = self.next_u8(io) as i8;
        let pc = self.pc as i16;
        let new_pc = pc.wrapping_add(value as i16);
        
        self.load_pc(io, new_pc as u16);
    }

    #[inline]
    /// Relative jump if cond is true
    fn jr_cond(&mut self, io: &mut Interconnect, cond: bool) {
        let value = self.next_u8(io) as i8;
        
        if cond {
            let pc = self.pc as i16;
            let new_pc = pc.wrapping_add(value as i16);
            self.load_pc(io, new_pc as u16);
        }
    }

    #[inline]
    /// Call a subroutine
    fn call(&mut self, io: &mut Interconnect) {
        let new_pc = self.next_u16(io);
        let current_pc = self.pc;
        
        self.push(io, current_pc);
        self.load_pc(io, new_pc);
    }

    #[inline]
    /// Call a subroutine if cond is true
    fn call_cond(&mut self, io: &mut Interconnect, cond: bool) {
        let new_pc = self.next_u16(io);

        if cond {
            let current_pc = self.pc();

            self.push(io, current_pc as u16);
            self.load_pc(io, new_pc);
        }
    }
    
    #[inline]
    /// Returns from a subroutine
    fn ret(&mut self, io: &mut Interconnect) {
        let new_pc = self.pop(io);
        self.load_pc(io, new_pc);
    }

    #[inline]
    /// Returns if cond is true
    fn ret_cond(&mut self, io: &mut Interconnect, cond: bool) {
        io.delay(1);

        if cond {
            let new_pc = self.pop(io);
            self.load_pc(io, new_pc);
        }
    }

    #[inline]
    /// Calls a certain subroutine
    fn rst(&mut self, io: &mut Interconnect, addr: u16) {
        let pc = self.pc;
        
        self.push(io, pc);
        self.load_pc(io, addr);
    }
    
    // Memory
    #[inline]
    /// Reads a byte at given address
    fn read_u8(&self, io: &mut Interconnect, address: usize) -> u8 {
        io.delay(1);
        
        io.read_u8(address)
    }

    #[inline]
    /// Reads a byte at (HL)
    fn read_hl(&self, io: &mut Interconnect) -> u8 {
        let hl = self.hl();
        
        self.read_u8(io, hl as usize)
    }

    #[inline]
    /// Writes a byte at (HL)
    fn write_hl(&self, io: &mut Interconnect, value: u8) {
        let hl = self.hl();
        
        self.write_u8(io, hl as usize, value)
    }

    #[inline]
    /// Reads a word at given address
    fn read_u16(&self, io: &mut Interconnect, address: usize) -> u16 {
        io.delay(2);
        
        io.read_u16(address)
    }

    #[inline]
    /// Writes a byte
    fn write_u8(&self, io: &mut Interconnect, address: usize, value: u8) {
        io.delay(1);
        
        io.write_u8(address, value);
    }

    #[inline]
    /// Writes a word
    fn write_u16(&self, io: &mut Interconnect, address: usize, value: u16) {
        io.delay(2);
        
        io.write_u16(address, value);
    }
    
    /// Returns the byte at PC and increments PC
    fn next_u8(&mut self, io: &mut Interconnect) -> u8 {
        let pc = self.pc;
        let value = self.read_u8(io, pc as usize);

        self.pc = pc.wrapping_add(1);

        value
    }

    /// Returns the word at PC and add 2 to PC
    fn next_u16(&mut self, io: &mut Interconnect) -> u16 {
        let value = self.read_u16(io, self.pc as usize);

        self.pc = self.pc.wrapping_add(2);

        value
    }

    /// Push a value on the stack
    fn push(&mut self, io: &mut Interconnect, value: u16) {
        self.sp -= 1;
        self.write_u8(io, self.sp as usize, ((value & 0xFF00) >> 8) as u8);
        self.sp -= 1;
        self.write_u8(io, self.sp as usize, (value & 0x00FF) as u8);
    }

    /// Pop a value from the stack
    fn pop(&mut self, io: &mut Interconnect) -> u16 {
        let value1 = self.read_u8(io, self.sp as usize) as u16;
        self.sp += 1;
        let value2 = self.read_u8(io, self.sp as usize) as u16;
        self.sp += 1;

        (value2 << 8) | value1
    }
    
    // 8-bit registers
    fn f(&self) -> u8 {
        ((self.zero as u8) << 7) |
        ((self.sub as u8) << 6) |
        ((self.half as u8) << 5) |
        ((self.carry as u8) << 4)
    }
    fn set_f(&mut self, f: u8) {
        self.zero = (f & 0x80) != 0;
        self.sub = (f & 0x40) != 0;
        self.half = (f & 0x20) != 0;
        self.carry = (f & 0x10) != 0;
    }

    // 16-bit registers
    fn af(&self) -> u16 { ((self.a as u16) << 8) | self.f() as u16 }
    fn set_af(&mut self, af: u16) {
        self.a = (af >> 8) as u8;
        self.set_f(af as u8);
    }

    fn bc(&self) -> u16 { ((self.b as u16) << 8) | self.c as u16 }
    fn set_bc(&mut self, bc: u16) {
        self.b = (bc >> 8) as u8;
        self.c = bc as u8;
    }
    
    fn de(&self) -> u16 { ((self.d as u16) << 8) | self.e as u16 }
    fn set_de(&mut self, de: u16) {
        self.d = (de >> 8) as u8;
        self.e = de as u8;
    }

    fn hl(&self) -> u16 { ((self.h as u16) << 8) | self.l as u16 }
    fn set_hl(&mut self, hl: u16) {
        self.h = (hl >> 8) as u8;
        self.l = hl as u8;
    }

    // Arithmetic
    fn inc_u8(&mut self, value: u8) -> u8 {
        self.zero = value == 0xff;
        self.sub = false;
        self.half = (value & 0xf) == 0xf;

        value.wrapping_add(1)
    }

    fn dec_u8(&mut self, value: u8) -> u8 {
        self.zero = value == 1;
        self.sub = true;
        self.half = (value & 0xf) == 0;

        value.wrapping_sub(1)
    }

    fn inc_u16(&self, io: &mut Interconnect, value: u16) -> u16 {
        io.delay(1);

        value.wrapping_add(1)
    }

    fn dec_u16(&self, io: &mut Interconnect, value: u16) -> u16 {
        io.delay(1);

        value.wrapping_sub(1)
    }
    
    fn add_u8(&mut self, value: u8) {
        let a = self.a as u32;
        let v = value as u32;
       
	let res = a + v;

        self.zero = (res & 0xff) == 0;
        self.sub = false;
        self.half = (a ^ v ^ res) & 0x10 == 0x10;
        self.carry = (res & 0x100) == 0x100;

        self.a = res as u8;
    }

    fn add_u16(&mut self, io: &mut Interconnect, value: u16) {
        let hl = self.hl() as u32;
        let v = value as u32;

        let res = hl + v;

        self.sub = false;
        self.half = (hl ^ v ^ res) & 0x1000 == 0x1000;
        self.carry = (res & 0x1_0000) != 0;

        self.set_hl(res as u16);
        io.delay(1);
    }
    
    fn adc_u8(&mut self, value: u8) {
        let c = self.carry as u8;
        let res = (self.a as u16) + (value as u16) + (c as u16);

        self.zero = (res & 0xFF) == 0;
        self.sub = false;
        self.half = (((self.a & 0xf) + (value & 0xf) + c) & 0x10) != 0;
        self.carry = (res & 0x100) != 0;

        self.a = res as u8;
    }

    fn sub_u8(&mut self, value: u8) {
        self.zero = self.a == value;
        self.sub = true;
        self.half = (self.a & 0xf) < (value & 0xf);
        self.carry = self.a < value;

        self.a = self.a.wrapping_sub(value);
    }

    fn sbc_u8(&mut self, value: u8) {
        let a = self.a as u16;
        let v = value as u16;
        let c = self.carry as u16;
        
        let res = a.wrapping_sub(v).wrapping_sub(c);
        
        self.zero = (res & 0xff) == 0;
        self.sub = true;
        self.half = ((a ^ v ^ res) & 0x10) != 0;
        self.carry = (res & 0x100) != 0;

        self.a = res as u8;
    }
    
    fn cp_u8(&mut self, value: u8) {
        self.zero = self.a == value;
        self.sub = true;
        self.half = (self.a & 0xf) < (value & 0xf);
        self.carry = self.a < value;
    }

    fn daa(&mut self) {
        let mut carry = false;

        if !self.sub {
            if self.carry || (self.a > 0x99) {
                self.a = self.a.wrapping_add(0x60);
                carry = true;
            }

            if self.half || (self.a & 0xf) > 0x9 {
                self.a = self.a.wrapping_add(0x06);
            }
        } else if self.carry {
            carry = true;

            self.a = self.a.wrapping_add(
                if self.half { 0x9a } else { 0xa0 }
            );
        } else if self.half {
            self.a = self.a.wrapping_add(0xfa);
        }

        self.carry = carry;
        self.zero = self.a == 0;
        self.half = false;
    }
    
    // Bitwise
    fn and_u8(&mut self, value: u8) {
        self.a &= value;

        self.zero = self.a == 0;
        self.sub = false;
        self.half = true;
        self.carry = false;
    }
    
    fn xor_u8(&mut self, value: u8) {
        self.a ^= value;

        self.zero = self.a == 0;
        self.sub = false;
        self.half = false;
        self.carry = false;
    }

    fn or_u8(&mut self, value: u8) {
        self.a |= value;

        self.zero = self.a == 0;
        self.sub = false;
        self.half = false;
        self.carry = false;
    }

    fn rlc_u8(&mut self, value: u8) -> u8 {
        let res = value.rotate_left(1);
        
        self.zero = res == 0;
        self.sub = false;
        self.half = false;
        self.carry = (res & 0x01) != 0;
        
        res
    }

    fn rrc_u8(&mut self, value: u8) -> u8 {
        let res = value.rotate_right(1);
        
        self.zero = res == 0;
        self.sub = false;
        self.half = false;
        self.carry = (res & 0x80) != 0;
        
        res
    }
    
    fn rl_u8(&mut self, value: u8) -> u8 {
        let carry = self.carry as u8;
        self.carry = (value & 0x80) != 0;
        
        let new_value = (value << 1) | carry;

        self.zero = new_value == 0;
        self.sub = false;
        self.half = false;

        new_value
    }
    
    fn rr_u8(&mut self, value: u8) -> u8 {
        let carry = self.carry as u8;
        let new_value = (value >> 1) | (carry << 7);

        self.zero = new_value == 0;
        self.sub = false;
        self.half = false;
        self.carry = (value & 0x01) != 0;

        new_value
    }

    fn sla_u8(&mut self, value: u8) -> u8 {
        self.zero = (value & 0x7f) == 0;
        self.sub = false;
        self.half = false;
        self.carry = (value & 0x80) != 0;

        value << 1
    }

    fn sra_u8(&mut self, value: u8) -> u8 {
        self.zero = (value & 0xfe) == 0;
        self.sub = false;
        self.half = false;
        self.carry = (value & 0x01) != 0;

        (value & 0x80) | (value >> 1)
    }
    
    fn swap_u8(&mut self, value: u8) -> u8 {
        self.zero = value == 0;
        self.sub = false;
        self.half = false;
        self.carry = false;

        value.rotate_right(4)
    }
    
    fn srl_u8(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;

        self.zero  = new_value == 0;
        self.sub   = false;
        self.half  = false;
        self.carry = (value & 1) != 0;

        new_value
    }
    
    fn bit_u8(&mut self, bit: u8, value: u8) {
        self.zero = (value & (1 << bit)) == 0;
        self.sub = false;
        self.half = true;
    }

    pub fn step(&mut self, io: &mut Interconnect) {
        if !self.halt {
            self.next_instruction(io);
        } else {
            io.delay(1);
        }

        if let Some(it) = io.next_interrupt() {
            self.halt = false;
            
            if self.ime {
                let pc = self.pc;
                
                self.push(io, pc);
                self.load_pc(io, it);

                self.ime = false;
                io.next_interrupt_ack();
            }
        }
    }
    
    fn next_instruction(&mut self, io: &mut Interconnect) {
        let opcode = self.next_u8(io);
        
        match opcode {
            0x00 => { }
            0x01 => { let bc = self.next_u16(io); self.set_bc(bc); }
            0x02 => { self.write_u8(io, self.bc() as usize, self.a) }
            0x03 => { let bc = self.inc_u16(io, self.bc()); self.set_bc(bc) }
            0x04 => { let b = self.b; self.b = self.inc_u8(b) }
            0x05 => { let b = self.b; self.b = self.dec_u8(b) }
            0x06 => { self.b = self.next_u8(io) }
            0x07 => {
                self.a = self.a.rotate_left(1);

                self.zero = false;
                self.sub = false;
                self.half = false;
                self.carry = (self.a & 0x01) != 0;
            }
            0x08 => {
                let addr = self.next_u16(io) as usize;
                self.write_u16(io, addr, self.sp);
            }
            0x09 => { let bc = self.bc(); self.add_u16(io, bc) }
            0x0A => { self.a = self.read_u8(io, self.bc() as usize) }
            0x0B => { let bc = self.dec_u16(io, self.bc()); self.set_bc(bc) }
            0x0C => { let c = self.c; self.c = self.inc_u8(c) },
            0x0D => { let c = self.c; self.c = self.dec_u8(c) },
            0x0E => { self.c = self.next_u8(io) }
            0x0F => {
                self.a = self.a.rotate_right(1);

                self.zero = false;
                self.sub = false;
                self.half = false;
                self.carry = (self.a & 0x80) != 0;
            }

            0x10 => { }
            0x11 => { let de = self.next_u16(io); self.set_de(de); },
            0x12 => {
                let de = self.de();

                self.write_u8(io, de as usize, self.a);
                self.set_de(de);
            },
            0x13 => { let de = self.inc_u16(io, self.de()); self.set_de(de) }
            0x14 => { let d = self.d; self.d = self.inc_u8(d) }
            0x15 => { let d = self.d; self.d = self.dec_u8(d) }
            0x16 => { self.d = self.next_u8(io) }
            0x17 => {
                let carry = self.carry as u8;
                self.carry = (self.a & 0x80) != 0;
                
                self.a = (self.a << 1) | carry;
                
                self.zero = false;
                self.sub = false;
                self.half = false;
            },
            
            0x18 => { self.jr(io) }
            0x19 => { let de = self.de(); self.add_u16(io, de) }
            0x1A => { self.a = self.read_u8(io, self.de() as usize) }
            0x1C => { let e = self.e; self.e = self.inc_u8(e) }
            0x1B => { let de = self.dec_u16(io, self.de()); self.set_de(de) }
            0x1D => { let e = self.e; self.e = self.dec_u8(e) }
            0x1E => self.e = self.next_u8(io),
            0x1F => {
                let carry = self.carry as u8;

                self.zero = false;
                self.sub = false;
                self.half = false;
                self.carry = (self.a & 0x01) != 0;

                self.a = (self.a >> 1) | (carry << 7);
            },
            
            0x20 => { let c = !self.zero; self.jr_cond(io, c) }
            0x21 => { let hl = self.next_u16(io); self.set_hl(hl); }
            0x22 => {
                let hl = self.hl();

                self.write_hl(io, self.a);
                self.set_hl(hl + 1);
            },
            0x23 => { let hl = self.inc_u16(io, self.hl()); self.set_hl(hl) }
            0x24 => { let h = self.h; self.h = self.inc_u8(h) }
            0x25 => { let h = self.h; self.h = self.dec_u8(h) }
            0x26 => { self.h = self.next_u8(io) }
            0x27 => { self.daa() }
            0x28 => { let c = self.zero; self.jr_cond(io, c) }
            0x29 => { let hl = self.hl(); self.add_u16(io, hl) }
            0x2A => {
                let hl = self.hl();

                self.a = self.read_hl(io);
                self.set_hl(hl + 1);
            }
            0x2B => { let hl = self.dec_u16(io, self.hl()); self.set_hl(hl) }
            0x2C => { let l = self.l; self.l = self.inc_u8(l) }
            0x2D => { let l = self.l; self.l = self.dec_u8(l) }
            0x2E => { self.l = self.next_u8(io) }
            0x2F => { self.sub = true; self.half = true; self.a ^= 0xFF }

            0x30 => { let c = !self.carry; self.jr_cond(io, c) }
            0x31 => { self.sp = self.next_u16(io) }
            0x32 => {
                let hl = self.hl();

                self.write_hl(io, self.a);
                self.set_hl(hl - 1);
            }
            0x33 => { self.sp = self.inc_u16(io, self.sp) }
            0x34 => {
                let hl = self.read_hl(io);
                let v = self.inc_u8(hl);
                self.write_hl(io, v)
            }
            0x35 => {
                let hl = self.read_hl(io);
                let v = self.dec_u8(hl);
                self.write_hl(io, v)
            }
            0x36 => { let v = self.next_u8(io); self.write_hl(io, v); }
            0x37 => { self.sub = false; self.half = false; self.carry = true }
            0x38 => { let c = self.carry; self.jr_cond(io, c) }
            0x39 => { let sp = self.sp; self.add_u16(io, sp) }
            0x3A => {
                let hl = self.hl();

                self.a = self.read_hl(io);
                self.set_hl(hl - 1);
            }
            0x3B => { self.sp = self.dec_u16(io, self.sp) }
            0x3C => { let a = self.a; self.a = self.inc_u8(a) }
            0x3D => { let a = self.a; self.a = self.dec_u8(a) }
            0x3E => self.a = self.next_u8(io),
            0x3F => { self.sub = false; self.half = false; self.carry = !self.carry }

            0x40 => { }
            0x41 => { self.b = self.c }
            0x42 => { self.b = self.d }
            0x43 => { self.b = self.e }
            0x44 => { self.b = self.h }
            0x45 => { self.b = self.l }
            0x46 => { self.b = self.read_hl(io) }
            0x47 => { self.b = self.a }
            0x48 => { self.c = self.b }
            0x49 => { }
            0x4A => { self.c = self.d }
            0x4B => { self.c = self.e }
            0x4C => { self.c = self.h }
            0x4D => { self.c = self.l }
            0x4E => { self.c = self.read_hl(io) }
            0x4F => { self.c = self.a }

            0x50 => { self.d = self.b }
            0x51 => { self.d = self.c }
            0x52 => { }
            0x53 => { self.d = self.e }
            0x54 => { self.d = self.h }
            0x55 => { self.d = self.l }
            0x56 => { self.d = self.read_hl(io) }
            0x57 => { self.d = self.a }
            0x58 => { self.e = self.b }
            0x59 => { self.e = self.c }
            0x5A => { self.e = self.d }
            0x5B => { }
            0x5C => { self.e = self.h }
            0x5D => { self.e = self.l }
            0x5E => { self.e = self.read_hl(io) }
            0x5F => { self.e = self.a }

            0x60 => { self.h = self.b }
            0x61 => { self.h = self.c }
            0x62 => { self.h = self.d}
            0x63 => { self.h = self.e }
            0x64 => { }
            0x65 => { self.h = self.l }
            0x66 => { self.h = self.read_hl(io) }
            0x67 => { self.h = self.a }
            0x68 => { self.l = self.b }
            0x69 => { self.l = self.c }
            0x6A => { self.l = self.d }
            0x6B => { self.l = self.e }
            0x6C => { self.l = self.h }
            0x6D => { }
            0x6E => { self.l = self.read_hl(io) }
            0x6F => { self.l = self.a }
            
            0x70 => { self.write_hl(io, self.b) }
            0x71 => { self.write_hl(io, self.c) }
            0x72 => { self.write_hl(io, self.d) }
            0x73 => { self.write_hl(io, self.e) }
            0x74 => { self.write_hl(io, self.h) }
            0x75 => { self.write_hl(io, self.l) }
            0x76 => { self.halt = true; }
            0x77 => { self.write_hl(io, self.a) }
            0x78 => { self.a = self.b }
            0x79 => { self.a = self.c }
            0x7A => { self.a = self.d }
            0x7B => { self.a = self.e }
            0x7C => { self.a = self.h }
            0x7D => { self.a = self.l }
            0x7E => { self.a = self.read_hl(io) },
            0x7F => { },
            
            0x80 => { let b = self.b; self.add_u8(b) }
            0x81 => { let c = self.c; self.add_u8(c) }
            0x82 => { let d = self.d; self.add_u8(d) }
            0x83 => { let e = self.e; self.add_u8(e) }
            0x84 => { let h = self.h; self.add_u8(h) }
            0x85 => { let l = self.l; self.add_u8(l) }
            0x86 => { let hl = self.read_hl(io); self.add_u8(hl) }
            0x87 => { let a = self.a; self.add_u8(a) }
            0x88 => { let b = self.b; self.adc_u8(b) }
            0x89 => { let c = self.c; self.adc_u8(c) }
            0x8A => { let d = self.d; self.adc_u8(d) }
            0x8B => { let e = self.e; self.adc_u8(e) }
            0x8C => { let h = self.h; self.adc_u8(h) }
            0x8D => { let l = self.l; self.adc_u8(l) }
            0x8E => { let hl = self.read_hl(io); self.adc_u8(hl) }
            0x8F => { let a = self.a; self.adc_u8(a) }

            0x90 => { let b = self.b; self.sub_u8(b) }
            0x91 => { let c = self.c; self.sub_u8(c) }
            0x92 => { let d = self.d; self.sub_u8(d) }
            0x93 => { let e = self.e; self.sub_u8(e) }
            0x94 => { let h = self.h; self.sub_u8(h) }
            0x95 => { let l = self.l; self.sub_u8(l) }
            0x96 => { let hl = self.read_hl(io); self.sub_u8(hl) }
            0x97 => { let a = self.a; self.sub_u8(a) }
            0x98 => { let b = self.b; self.sbc_u8(b) }
            0x99 => { let c = self.c; self.sbc_u8(c) }
            0x9A => { let d = self.d; self.sbc_u8(d) }
            0x9B => { let e = self.e; self.sbc_u8(e) }
            0x9C => { let h = self.h; self.sbc_u8(h) }
            0x9D => { let l = self.l; self.sbc_u8(l) }
            0x9E => { let hl = self.read_hl(io); self.sbc_u8(hl) }
            0x9F => { let a = self.a; self.sbc_u8(a) }

            0xA0 => { let b = self.b; self.and_u8(b) }
            0xA1 => { let c = self.c; self.and_u8(c) }
            0xA2 => { let d = self.d; self.and_u8(d) }
            0xA3 => { let e = self.e; self.and_u8(e) }
            0xA4 => { let h = self.h; self.and_u8(h) }
            0xA5 => { let l = self.l; self.and_u8(l) }
            0xA6 => { let hl = self.read_hl(io); self.and_u8(hl) }
            0xA7 => { let a = self.a; self.and_u8(a) }
            0xA8 => { let b = self.b; self.xor_u8(b) }
            0xA9 => { let c = self.c; self.xor_u8(c) }
            0xAA => { let d = self.d; self.xor_u8(d) }
            0xAB => { let e = self.e; self.xor_u8(e) }
            0xAC => { let h = self.h; self.xor_u8(h) }
            0xAD => { let l = self.l; self.xor_u8(l) }
            0xAE => { let hl = self.read_hl(io); self.xor_u8(hl) }
            0xAF => { let a = self.a; self.xor_u8(a) }

            0xB0 => { let b = self.b; self.or_u8(b) }
            0xB1 => { let c = self.c; self.or_u8(c) }
            0xB2 => { let d = self.d; self.or_u8(d) }
            0xB3 => { let e = self.e; self.or_u8(e) }
            0xB4 => { let h = self.h; self.or_u8(h) }
            0xB5 => { let l = self.l; self.or_u8(l) }
            0xB6 => { let hl = self.read_hl(io); self.or_u8(hl) }
            0xB7 => { let a = self.a; self.or_u8(a) }
            0xB8 => { let b = self.b; self.cp_u8(b) }
            0xB9 => { let c = self.c; self.cp_u8(c) }
            0xBA => { let d = self.d; self.cp_u8(d) }
            0xBB => { let e = self.e; self.cp_u8(e) }
            0xBC => { let h = self.h; self.cp_u8(h) }
            0xBD => { let l = self.l; self.cp_u8(l) }
            0xBE => { let hl = self.read_hl(io); self.cp_u8(hl) }
            0xBF => { let a = self.a; self.cp_u8(a) }

            0xC0 => { let c = !self.zero; self.ret_cond(io, c) }
            0xC1 => { let bc = self.pop(io); self.set_bc(bc) }
            0xC2 => { let c = !self.zero; self.jp_cond(io, c) }
            0xC3 => { self.jp(io) }
            0xC4 => { let c = !self.zero; self.call_cond(io, c) }
            0xC5 => { let bc = self.bc(); io.delay(1); self.push(io, bc) }
            0xC6 => { let v = self.next_u8(io); self.add_u8(v) }
            0xC7 => { self.rst(io, 0x00) }
            0xC8 => { let c = self.zero; self.ret_cond(io, c) }
            0xC9 => { self.ret(io) }
            0xCA => { let c = self.zero; self.jp_cond(io, c) }
            0xCB => { self.next_instruction_cb(io) }
            0xCC => { let c = self.zero; self.call_cond(io, c) }
            0xCD => { self.call(io) }
            0xCE => { let v = self.next_u8(io); self.adc_u8(v) }
            0xCF => { self.rst(io, 0x08) }

            0xD0 => { let c = !self.carry; self.ret_cond(io, c) }
            0xD1 => { let de = self.pop(io); self.set_de(de) }
            0xD2 => { let c = !self.carry; self.jp_cond(io, c) }
            0xD4 => { let c = !self.carry; self.call_cond(io, c) }
            0xD5 => { let de = self.de(); io.delay(1); self.push(io, de) }
            0xD6 => { let v = self.next_u8(io); self.sub_u8(v) }
            0xD7 => { self.rst(io, 0x10) }
            0xD8 => { let c = self.carry; self.ret_cond(io, c) }
            0xD9 => { self.ret(io); self.ime = true }
            0xDA => { let c = self.carry; self.jp_cond(io, c) }
            0xDC => { let c = self.carry; self.call_cond(io, c) }
            0xDE => { let v = self.next_u8(io); self.sbc_u8(v) }
            0xDF => { self.rst(io, 0x18) }
            
            0xE0 => {
                let address = 0xFF00 + (self.next_u8(io) as u16);
                self.write_u8(io, address as usize, self.a);
            },
            0xE1 => { let hl = self.pop(io); self.set_hl(hl); },
            0xE2 => { self.write_u8(io, 0xFF00 + self.c as usize, self.a) }
            0xE5 => { let hl = self.hl(); io.delay(1); self.push(io, hl); }
            0xE6 => { let v = self.next_u8(io); self.and_u8(v) }
            0xE7 => { self.rst(io, 0x20) }
            0xE8 => {
                let sp = self.sp as i32;
                let v = (self.next_u8(io) as i8) as i32;
                
                let res = sp + v;

                self.zero = false;
                self.sub = false;
                self.half = (sp ^ v ^ res) & 0x10 == 0x10;
                self.carry = (sp ^ v ^ res) & 0x100 == 0x100;
                
                self.sp = res as u16;
                io.delay(2);
            }
            0xE9 => { self.pc = self.hl() }
            0xEA => {
                let addr = self.next_u16(io);
                self.write_u8(io, addr as usize, self.a);
            }
            0xEE => { let v = self.next_u8(io); self.xor_u8(v) }
            0xEF => { self.rst(io, 0x28) }
            
            0xF0 => {
                let v = self.next_u8(io) as usize;
                self.a = self.read_u8(io, 0xff00 + v);
            },
            0xF1 => { let af = self.pop(io); self.set_af(af); }
            0xF2 => { self.a = self.read_u8(io, 0xFF00 + self.c as usize) }
            0xF3 => { self.ime = false }
            0xF5 => { let af = self.af(); io.delay(1); self.push(io, af); }
            0xF6 => { let v = self.next_u8(io); self.or_u8(v) }
            0xF7 => { self.rst(io, 0x30) }
            0xF8 => {
                let sp = self.sp as i32;
                let v = (self.next_u8(io) as i8) as i32;
                
                let res = sp + v;

                self.zero = false;
                self.sub = false;
                self.half = (sp ^ v ^ res) & 0x10 == 0x10;
                self.carry = (sp ^ v ^ res) & 0x100 == 0x100;
                
                self.set_hl(res as u16);
                io.delay(1);
            }
            0xF9 => { io.delay(1); self.sp = self.hl() }
            0xFA => {
                let addr = self.next_u16(io);
                self.a = self.read_u8(io, addr as usize);
            }
            0xFB => { self.ime = true }
            0xFE => { let v = self.next_u8(io); self.cp_u8(v); }
            0xFF => { self.rst(io, 0x38) }
                
            _ => {
                println!("{}", self);
                unimplemented!("Opcode 0x{:02X} ({:04x}) does not exist.",
                               opcode, self.pc - 1);
            }
        }
    }

    fn next_instruction_cb(&mut self, io: &mut Interconnect) {
        let opcode = self.next_u8(io);

        match opcode {
            0x00 => { let b = self.b; self.b = self.rlc_u8(b) }
            0x01 => { let c = self.c; self.c = self.rlc_u8(c) }
            0x02 => { let d = self.d; self.d = self.rlc_u8(d) }
            0x03 => { let e = self.e; self.e = self.rlc_u8(e) }
            0x04 => { let h = self.h; self.h = self.rlc_u8(h) }
            0x05 => { let l = self.l; self.l = self.rlc_u8(l) }
            0x06 => {
                let hl = self.read_hl(io);
                let new_hl = self.rlc_u8(hl);
                self.write_hl(io, new_hl);
            }
            0x07 => { let a = self.a; self.a = self.rlc_u8(a) }
            0x08 => { let b = self.b; self.b = self.rrc_u8(b) }
            0x09 => { let c = self.c; self.c = self.rrc_u8(c) }
            0x0A => { let d = self.d; self.d = self.rrc_u8(d) }
            0x0B => { let e = self.e; self.e = self.rrc_u8(e) }
            0x0C => { let h = self.h; self.h = self.rrc_u8(h) }
            0x0D => { let l = self.l; self.l = self.rrc_u8(l) }
            0x0E => {
                let hl = self.read_hl(io);
                let new_hl = self.rrc_u8(hl);
                self.write_hl(io, new_hl);
            }
            0x0F => { let a = self.a; self.a = self.rrc_u8(a) }
            
            0x10 => { let b = self.b; self.b = self.rl_u8(b) }
            0x11 => { let c = self.c; self.c = self.rl_u8(c) }
            0x12 => { let d = self.d; self.d = self.rl_u8(d) }
            0x13 => { let e = self.e; self.e = self.rl_u8(e) }
            0x14 => { let h = self.h; self.h = self.rl_u8(h) }
            0x15 => { let l = self.l; self.l = self.rl_u8(l) }
            0x16 => {
                let hl = self.read_hl(io);
                let new_hl = self.rl_u8(hl);
                self.write_hl(io, new_hl);
            }
            0x17 => { let a = self.a; self.a = self.rl_u8(a) }
            0x18 => { let b = self.b; self.b = self.rr_u8(b) }
            0x19 => { let c = self.c; self.c = self.rr_u8(c) }
            0x1A => { let d = self.d; self.d = self.rr_u8(d) }
            0x1B => { let e = self.e; self.e = self.rr_u8(e) }
            0x1C => { let h = self.h; self.h = self.rr_u8(h) }
            0x1D => { let l = self.l; self.l = self.rr_u8(l) }
            0x1E => {
                let hl = self.read_hl(io);
                let new_hl = self.rr_u8(hl);
                self.write_hl(io, new_hl);
            }
            0x1F => { let a = self.a; self.a = self.rr_u8(a) }
            
            0x20 => { let b = self.b; self.b = self.sla_u8(b) }
            0x21 => { let c = self.c; self.c = self.sla_u8(c) }
            0x22 => { let d = self.d; self.d = self.sla_u8(d) }
            0x23 => { let e = self.e; self.e = self.sla_u8(e) }
            0x24 => { let h = self.h; self.h = self.sla_u8(h) }
            0x25 => { let l = self.l; self.l = self.sla_u8(l) }
            0x26 => {
                let hl = self.read_hl(io);
                let new_hl = self.sla_u8(hl);
                self.write_hl(io, new_hl);
            }
            0x27 => { let a = self.a; self.a = self.sla_u8(a) }
            0x28 => { let b = self.b; self.b = self.sra_u8(b) }
            0x29 => { let c = self.c; self.c = self.sra_u8(c) }
            0x2A => { let d = self.d; self.d = self.sra_u8(d) }
            0x2B => { let e = self.e; self.e = self.sra_u8(e) }
            0x2C => { let h = self.h; self.h = self.sra_u8(h) }
            0x2D => { let l = self.l; self.l = self.sra_u8(l) }
            0x2E => {
                let hl = self.read_hl(io);
                let new_hl = self.sra_u8(hl);
                self.write_hl(io, new_hl);
            }
            0x2F => { let a = self.a; self.a = self.sra_u8(a) }

            0x30 => { let b = self.b; self.b = self.swap_u8(b) }
            0x31 => { let c = self.c; self.c = self.swap_u8(c) }
            0x32 => { let d = self.d; self.d = self.swap_u8(d) }
            0x33 => { let e = self.e; self.e = self.swap_u8(e) }
            0x34 => { let h = self.h; self.h = self.swap_u8(h) }
            0x35 => { let l = self.l; self.l = self.swap_u8(l) }
            0x36 => {
                let hl = self.read_hl(io);
                let new_hl = self.swap_u8(hl);
                self.write_hl(io, new_hl);
            }
            0x37 => { let a = self.a; self.a = self.swap_u8(a) }
            0x38 => { let b = self.b; self.b = self.srl_u8(b) }
            0x39 => { let c = self.c; self.c = self.srl_u8(c) }
            0x3A => { let d = self.d; self.d = self.srl_u8(d) }
            0x3B => { let e = self.e; self.e = self.srl_u8(e) }
            0x3C => { let h = self.h; self.h = self.srl_u8(h) }
            0x3D => { let l = self.l; self.l = self.srl_u8(l) }
            0x3E => {
                let hl = self.read_hl(io);
                let new_hl = self.srl_u8(hl);
                self.write_hl(io, new_hl);
            }
            0x3F => { let a = self.a; self.a = self.srl_u8(a) }

            0x40 => { let b = self.b; self.bit_u8(0, b) }
            0x41 => { let c = self.c; self.bit_u8(0, c) }
            0x42 => { let d = self.d; self.bit_u8(0, d) }
            0x43 => { let e = self.e; self.bit_u8(0, e) }
            0x44 => { let h = self.h; self.bit_u8(0, h) }
            0x45 => { let l = self.l; self.bit_u8(0, l) }
            0x46 => { let hl = self.read_hl(io); self.bit_u8(0, hl) }
            0x47 => { let a = self.a; self.bit_u8(0, a) }
            0x48 => { let b = self.b; self.bit_u8(1, b) }
            0x49 => { let c = self.c; self.bit_u8(1, c) }
            0x4A => { let d = self.d; self.bit_u8(1, d) }
            0x4B => { let e = self.e; self.bit_u8(1, e) }
            0x4C => { let h = self.h; self.bit_u8(1, h) }
            0x4D => { let l = self.l; self.bit_u8(1, l) }
            0x4E => { let hl = self.read_hl(io); self.bit_u8(1, hl) }
            0x4F => { let a = self.a; self.bit_u8(1, a) }
            
            0x50 => { let b = self.b; self.bit_u8(2, b) }
            0x51 => { let c = self.c; self.bit_u8(2, c) }
            0x52 => { let d = self.d; self.bit_u8(2, d) }
            0x53 => { let e = self.e; self.bit_u8(2, e) }
            0x54 => { let h = self.h; self.bit_u8(2, h) }
            0x55 => { let l = self.l; self.bit_u8(2, l) }
            0x56 => { let hl = self.read_hl(io); self.bit_u8(2, hl) }
            0x57 => { let a = self.a; self.bit_u8(2, a) }
            0x58 => { let b = self.b; self.bit_u8(3, b) }
            0x59 => { let c = self.c; self.bit_u8(3, c) }
            0x5A => { let d = self.d; self.bit_u8(3, d) }
            0x5B => { let e = self.e; self.bit_u8(3, e) }
            0x5C => { let h = self.h; self.bit_u8(3, h) }
            0x5D => { let l = self.l; self.bit_u8(3, l) }
            0x5E => { let hl = self.read_hl(io); self.bit_u8(3, hl) }
            0x5F => { let a = self.a; self.bit_u8(3, a) }
            
            0x60 => { let b = self.b; self.bit_u8(4, b) }
            0x61 => { let c = self.c; self.bit_u8(4, c) }
            0x62 => { let d = self.d; self.bit_u8(4, d) }
            0x63 => { let e = self.e; self.bit_u8(4, e) }
            0x64 => { let h = self.h; self.bit_u8(4, h) }
            0x65 => { let l = self.l; self.bit_u8(4, l) }
            0x66 => { let hl = self.read_hl(io); self.bit_u8(4, hl) }
            0x67 => { let a = self.a; self.bit_u8(4, a) }
            0x68 => { let b = self.b; self.bit_u8(5, b) }
            0x69 => { let c = self.c; self.bit_u8(5, c) }
            0x6A => { let d = self.d; self.bit_u8(5, d) }
            0x6B => { let e = self.e; self.bit_u8(5, e) }
            0x6C => { let h = self.h; self.bit_u8(5, h) }
            0x6D => { let l = self.l; self.bit_u8(5, l) }
            0x6E => { let hl = self.read_hl(io); self.bit_u8(5, hl) }
            0x6F => { let a = self.a; self.bit_u8(5, a) }
            
            0x70 => { let b = self.b; self.bit_u8(6, b) }
            0x71 => { let c = self.c; self.bit_u8(6, c) }
            0x72 => { let d = self.d; self.bit_u8(6, d) }
            0x73 => { let e = self.e; self.bit_u8(6, e) }
            0x74 => { let h = self.h; self.bit_u8(6, h) }
            0x75 => { let l = self.l; self.bit_u8(6, l) }
            0x76 => { let hl = self.read_hl(io); self.bit_u8(6, hl) }
            0x77 => { let a = self.a; self.bit_u8(6, a) }
            0x78 => { let b = self.b; self.bit_u8(7, b) }
            0x79 => { let c = self.c; self.bit_u8(7, c) }
            0x7A => { let d = self.d; self.bit_u8(7, d) }
            0x7B => { let e = self.e; self.bit_u8(7, e) }
            0x7C => { let h = self.h; self.bit_u8(7, h) }
            0x7D => { let l = self.l; self.bit_u8(7, l) }
            0x7E => { let hl = self.read_hl(io); self.bit_u8(7, hl) }
            0x7F => { let a = self.a; self.bit_u8(7, a) }

            0x80 => { self.b &= 0xfe }
            0x81 => { self.c &= 0xfe }
            0x82 => { self.d &= 0xfe }
            0x83 => { self.e &= 0xfe }
            0x84 => { self.h &= 0xfe }
            0x85 => { self.l &= 0xfe }
            0x86 => { let hl = self.read_hl(io) & 0xfe; self.write_hl(io, hl) }
            0x87 => { self.a &= 0xfe }
            0x88 => { self.b &= 0xfd }
            0x89 => { self.c &= 0xfd }
            0x8A => { self.d &= 0xfd }
            0x8B => { self.e &= 0xfd }
            0x8C => { self.h &= 0xfd }
            0x8D => { self.l &= 0xfd }
            0x8E => { let hl = self.read_hl(io) & 0xfd; self.write_hl(io, hl) }
            0x8F => { self.a &= 0xfd }

            0x90 => { self.b &= 0xfb }
            0x91 => { self.c &= 0xfb }
            0x92 => { self.d &= 0xfb }
            0x93 => { self.e &= 0xfb }
            0x94 => { self.h &= 0xfb }
            0x95 => { self.l &= 0xfb }
            0x96 => { let hl = self.read_hl(io) & 0xfb; self.write_hl(io, hl) }
            0x97 => { self.a &= 0xfb }
            0x98 => { self.b &= 0xf7 }
            0x99 => { self.c &= 0xf7 }
            0x9A => { self.d &= 0xf7 }
            0x9B => { self.e &= 0xf7 }
            0x9C => { self.h &= 0xf7 }
            0x9D => { self.l &= 0xf7 }
            0x9E => { let hl = self.read_hl(io) & 0xf7; self.write_hl(io, hl) }
            0x9F => { self.a &= 0xf7 }

            0xA0 => { self.b &= 0xef }
            0xA1 => { self.c &= 0xef }
            0xA2 => { self.d &= 0xef }
            0xA3 => { self.e &= 0xef }
            0xA4 => { self.h &= 0xef }
            0xA5 => { self.l &= 0xef }
            0xA6 => { let hl = self.read_hl(io) & 0xef; self.write_hl(io, hl) }
            0xA7 => { self.a &= 0xef }
            0xA8 => { self.b &= 0xdf }
            0xA9 => { self.c &= 0xdf }
            0xAA => { self.d &= 0xdf }
            0xAB => { self.e &= 0xdf }
            0xAC => { self.h &= 0xdf }
            0xAD => { self.l &= 0xdf }
            0xAE => { let hl = self.read_hl(io) & 0xdf; self.write_hl(io, hl) }
            0xAF => { self.a &= 0xdf }

            0xB0 => { self.b &= 0xbf }
            0xB1 => { self.c &= 0xbf }
            0xB2 => { self.d &= 0xbf }
            0xB3 => { self.e &= 0xbf }
            0xB4 => { self.h &= 0xbf }
            0xB5 => { self.l &= 0xbf }
            0xB6 => { let hl = self.read_hl(io) & 0xbf; self.write_hl(io, hl) }
            0xB7 => { self.a &= 0xbf }
            0xB8 => { self.b &= 0x7f }
            0xB9 => { self.c &= 0x7f }
            0xBA => { self.d &= 0x7f }
            0xBB => { self.e &= 0x7f }
            0xBC => { self.h &= 0x7f }
            0xBD => { self.l &= 0x7f }
            0xBE => { let hl = self.read_hl(io) & 0x7f; self.write_hl(io, hl) }
            0xBF => { self.a &= 0x7f }

            0xC0 => { self.b |= 0x01 }
            0xC1 => { self.c |= 0x01 }
            0xC2 => { self.d |= 0x01 }
            0xC3 => { self.e |= 0x01 }
            0xC4 => { self.h |= 0x01 }
            0xC5 => { self.l |= 0x01 }
            0xC6 => { let hl = self.read_hl(io) | 0x01; self.write_hl(io, hl) }
            0xC7 => { self.a |= 0x01 }
            0xC8 => { self.b |= 0x02 }
            0xC9 => { self.c |= 0x02 }
            0xCA => { self.d |= 0x02 }
            0xCB => { self.e |= 0x02 }
            0xCC => { self.h |= 0x02 }
            0xCD => { self.l |= 0x02 }
            0xCE => { let hl = self.read_hl(io) | 0x02; self.write_hl(io, hl) }
            0xCF => { self.a |= 0x02 }

            0xD0 => { self.b |= 0x04 }
            0xD1 => { self.c |= 0x04 }
            0xD2 => { self.d |= 0x04 }
            0xD3 => { self.e |= 0x04 }
            0xD4 => { self.h |= 0x04 }
            0xD5 => { self.l |= 0x04 }
            0xD6 => { let hl = self.read_hl(io) | 0x04; self.write_hl(io, hl) }
            0xD7 => { self.a |= 0x04 }
            0xD8 => { self.b |= 0x08 }
            0xD9 => { self.c |= 0x08 }
            0xDA => { self.d |= 0x08 }
            0xDB => { self.e |= 0x08 }
            0xDC => { self.h |= 0x08 }
            0xDD => { self.l |= 0x08 }
            0xDE => { let hl = self.read_hl(io) | 0x08; self.write_hl(io, hl) }
            0xDF => { self.a |= 0x08 }

            0xE0 => { self.b |= 0x10 }
            0xE1 => { self.c |= 0x10 }
            0xE2 => { self.d |= 0x10 }
            0xE3 => { self.e |= 0x10 }
            0xE4 => { self.h |= 0x10 }
            0xE5 => { self.l |= 0x10 }
            0xE6 => { let hl = self.read_hl(io) | 0x10; self.write_hl(io, hl) }
            0xE7 => { self.a |= 0x10 }
            0xE8 => { self.b |= 0x20 }
            0xE9 => { self.c |= 0x20 }
            0xEA => { self.d |= 0x20 }
            0xEB => { self.e |= 0x20 }
            0xEC => { self.h |= 0x20 }
            0xED => { self.l |= 0x20 }
            0xEE => { let hl = self.read_hl(io) | 0x20; self.write_hl(io, hl) }
            0xEF => { self.a |= 0x20 }

            0xF0 => { self.b |= 0x40 }
            0xF1 => { self.c |= 0x40 }
            0xF2 => { self.d |= 0x40 }
            0xF3 => { self.e |= 0x40 }
            0xF4 => { self.h |= 0x40 }
            0xF5 => { self.l |= 0x40 }
            0xF6 => { let hl = self.read_hl(io) | 0x40; self.write_hl(io, hl) }
            0xF7 => { self.a |= 0x40 }
            0xF8 => { self.b |= 0x80 }
            0xF9 => { self.c |= 0x80 }
            0xFA => { self.d |= 0x80 }
            0xFB => { self.e |= 0x80 }
            0xFC => { self.h |= 0x80 }
            0xFD => { self.l |= 0x80 }
            0xFE => { let hl = self.read_hl(io) | 0x80; self.write_hl(io, hl) }
            0xFF => { self.a |= 0x80 }
        }
    }
}

impl fmt::Display for LR35902 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=====LR35902=====\n\
                   AF: {:02x}{:02x} HL: {:02x}{:02x} [{}{}{}{}]\n\
                   BC: {:02x}{:02x} SP: {:04x}\n\
                   DE: {:02x}{:02x} PC: {:04x}",
               self.a, self.f(), self.h, self.l,
               if self.zero  { 'Z' } else { '-' },
               if self.sub   { 'N' } else { '-' },
               if self.half  { 'H' } else { '-' },
               if self.carry { 'C' } else { '-' },
               self.b, self.c, self.sp,
               self.d, self.e, self.pc
        )
    }
}
