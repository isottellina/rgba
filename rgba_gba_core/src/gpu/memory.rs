// memory.rs --- 
// 
// Filename: memory.rs
// Author: Louise <louise>
// Created: Tue Jan 23 16:22:07 2018 (+0100)
// Last-Updated: Tue Jan 23 16:27:54 2018 (+0100)
//           By: Louise <louise>
// 
use byteorder::{ByteOrder, LittleEndian};
use gpu::GPU;

impl GPU {
    #[inline]
    pub fn pram_write_u32(&mut self, address: usize, value: u32) {
        LittleEndian::write_u32(
            &mut self.pram[(address & 0x3ff)..], value
        );
    }

    #[inline]
    pub fn vram_write_u32(&mut self, address: usize, value: u32) {
        LittleEndian::write_u32(
            &mut self.vram[(address & 0x17fff)..], value
        );
    }

    #[inline]
    pub fn oam_write_u32(&mut self, address: usize, value: u32) {
        LittleEndian::write_u32(
            &mut self.oam[(address & 0x3ff)..], value
        );
    }

    #[inline]
    pub fn pram_write_u16(&mut self, address: usize, value: u16) {
        LittleEndian::write_u16(
            &mut self.pram[(address & 0x3ff)..], value
        );
    }

    #[inline]
    pub fn vram_write_u16(&mut self, address: usize, value: u16) {
        LittleEndian::write_u16(
            &mut self.vram[(address & 0x17fff)..], value
        );
    }

    #[inline]
    pub fn oam_write_u16(&mut self, address: usize, value: u16) {
        LittleEndian::write_u16(
            &mut self.oam[(address & 0x3ff)..], value
        );
    }

    #[inline]
    pub fn pram_read_u32(&self, address: usize) -> u32 {
        LittleEndian::read_u32(
            &self.pram[(address & 0x3ff)..]
        )
    }

    #[inline]
    pub fn vram_read_u32(&self, address: usize) -> u32{
        LittleEndian::read_u32(
            &self.vram[(address & 0x17fff)..]
        )
    }

    #[inline]
    pub fn oam_read_u32(&self, address: usize) -> u32{
        LittleEndian::read_u32(
            &self.oam[(address & 0x3ff)..]
        )
    }
    
    #[inline]
    pub fn pram_read_u16(&self, address: usize) -> u16 {
        LittleEndian::read_u16(
            &self.pram[(address & 0x3ff)..]
        )
    }

    #[inline]
    pub fn vram_read_u16(&self, address: usize) -> u16{
        LittleEndian::read_u16(
            &self.vram[(address & 0x17fff)..]
        )
    }

    #[inline]
    pub fn oam_read_u16(&self, address: usize) -> u16{
        LittleEndian::read_u16(
            &self.oam[(address & 0x3ff)..]
        )
    }
}
