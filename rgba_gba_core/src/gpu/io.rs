// io.rs --- 
// 
// Filename: io.rs
// Author: Louise <louise>
// Created: Tue Jan 23 16:22:52 2018 (+0100)
// Last-Updated: Tue Jan 23 22:37:43 2018 (+0100)
//           By: Louise <louise>
// 
use gpu::GPU;

impl GPU {
    #[inline]
    pub fn io_read_u16(&self, address: usize) -> u16 {
        match address {
            VCOUNT => self.vcount,
            _ => { warn!("Unmapped read_u16 from {:08x} (GPU)", address); 0 },
        }
    }

    #[inline]
    pub fn io_write_u16(&mut self, address: usize, value: u16) {
        match address {
            DISPCNT => self.dispcnt = value,
            DISPSTAT => {
                self.irq_vblank_en = (value & 0x0008) != 0;
                self.irq_hblank_en = (value & 0x0010) != 0;
                self.irq_vcount_en = (value & 0x0020) != 0;

                self.vcount_match  =  value >> 8;
            }
            _ => warn!("Unmapped write_u16 to {:08x} (GPU, value={:04x})", address, value),
        }
    }
}

const DISPCNT:  usize = 0x04000000;
const DISPSTAT: usize = 0x04000004;
const VCOUNT:   usize = 0x04000006;
