// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Thu Jan 18 14:14:22 2018 (+0100)
// Last-Updated: Sun Jan 21 20:55:16 2018 (+0100)
//           By: Louise <louise>
// 
use byteorder::{ByteOrder, LittleEndian};

pub struct GPU {
    // Memory
    pram: [u8; 0x400],
    vram: [u8; 0x18000],
    oam:  [u8; 0x400],

    // State
    render_line: Option<u16>,
    vcount: u16,
    clock: u32,
    dots: u32,
    mode: GpuMode,

    dispcnt: u16,
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            pram: [0; 0x400],
            vram: [0; 0x18000],
            oam:  [0; 0x400],

            render_line: None,
            vcount: 0,
            clock: 0,
            dots: 0,
            mode: GpuMode::Visible,

            dispcnt: 0,
        }
    }

    #[inline]
    fn increment_lines(&mut self) {
        self.vcount = (self.vcount + 1) % 228;
    }
    
    pub fn spend_cycles(&mut self, nb_cycles: u32) {
        let total_cycles = self.clock + nb_cycles;

        let dots = total_cycles >> 2;
        let new_clock = total_cycles & 3;
        
        self.dots += dots;
        self.clock = new_clock;

        match self.mode {
            GpuMode::Visible => {
                if self.dots >= 240 {
                    self.mode = GpuMode::HBlank;
                }
            }
            GpuMode::HBlank => {
                while self.dots >= 308 {
                    self.dots -= 308;
                    self.increment_lines();

                    if self.vcount == 160 {
                        self.mode = GpuMode::VBlank;
                    } else {
                        self.mode = GpuMode::Visible;
                    }
                }
            }
            GpuMode::VBlank => {
                while self.dots >= 308 {
                    self.dots -= 308;
                    self.increment_lines();

                    if self.vcount == 0 {
                        self.mode = GpuMode::Visible;
                    }
                }
            }
        }
    }

    #[inline]
    pub fn io_write_u16(&mut self, address: usize, value: u16) {
        match address {
            DISPCNT => self.dispcnt = value,
            _ => warn!("Unmapped io_write_u16 at {:08x} (GPU, value={:04x})", address, value),
        }
    }

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
}

enum GpuMode {
    Visible,
    HBlank,
    VBlank,
}

const DISPCNT: usize = 0x04000000;
