// mos6502.rs --- 
// 
// Filename: mos6502.rs
// Author: Louise <ludwigette>
// Created: Fri Aug  3 13:44:39 2018 (+0200)
// Last-Updated: Fri Aug  3 21:34:03 2018 (+0200)
//           By: Louise <ludwigette>
// 

pub struct MOS6502 {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    p: u8,
    pc: u16,
}

impl MOS6502 {
    pub fn new() -> MOS6502 {
        MOS6502 {
            a: 0,
            x: 0,
            y: 0,
            s: 0xfd,
            p: 0x34,
        }
    }
}
