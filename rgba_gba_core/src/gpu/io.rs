// io.rs --- 
// 
// Filename: io.rs
// Author: Louise <louise>
// Created: Tue Jan 23 16:22:52 2018 (+0100)
// Last-Updated: Tue Apr 17 19:45:47 2018 (+0200)
//           By: Louise <louise>
// 
use crate::gpu::GPU;
use crate::gpu::GpuMode;

impl GPU {
    #[inline]
    pub fn io_read_u16(&self, address: usize) -> u16 {
        match address {
            DISPCNT => self.dispcnt,
            DISPSTAT => {
                ((self.mode == GpuMode::VBlank) as u16) |
                (((self.mode == GpuMode::HBlank) as u16) << 1) |
                (((self.vcount == self.vcount_match) as u16) << 2) |
                ((self.irq_vblank_en as u16) << 3) |
                ((self.irq_hblank_en as u16) << 4) |
                ((self.irq_vcount_en as u16) << 5) |
                (self.vcount_match << 8)
            },
            VCOUNT => self.vcount,

            // Background control
            BG0CNT  => self.bg[0].cnt,
            BG1CNT  => self.bg[1].cnt,
            BG2CNT  => self.bg[2].cnt,
            BG3CNT  => self.bg[3].cnt,
            
            // Unused
            0x04000002 => 0,
            
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
            VCOUNT => { },

            // Background control
            BG0CNT  => self.bg[0].cnt = value,
            BG1CNT  => self.bg[1].cnt = value,
            BG2CNT  => self.bg[2].cnt = value,
            BG3CNT  => self.bg[3].cnt = value,

            // Background offsets
            BG0HOFS  => self.bg[0].h_off = value,
            BG0VOFS  => self.bg[0].v_off = value,
            BG1HOFS  => self.bg[1].h_off = value,
            BG1VOFS  => self.bg[1].v_off = value,
            BG2HOFS  => self.bg[2].h_off = value,
            BG2VOFS  => self.bg[2].v_off = value,
            BG3HOFS  => self.bg[3].h_off = value,
            BG3VOFS  => self.bg[3].v_off = value,

            // Background rotation
            BG2X_L   => self.bg[2].x_ref = (self.bg[2].x_ref & 0x0FFF0000) | (value as u32),
            BG2X_H   => self.bg[2].x_ref =
                (((((self.bg[2].x_ref & 0xFFFF) | (((value as u32) & 0x0FFF0000) << 16)) << 4) as i32) >> 4) as u32,
            BG2Y_L   => self.bg[2].y_ref = (self.bg[2].y_ref & 0x0FFF0000) | (value as u32),
            BG2Y_H   => self.bg[2].y_ref =
                (((((self.bg[2].y_ref & 0xFFFF) | (((value as u32) & 0x0FFF0000) << 16)) << 4) as i32) >> 4) as u32,
            BG2PA    => self.bg[2].par_a = value,
            BG2PB    => self.bg[2].par_b = value,
            BG2PC    => self.bg[2].par_c = value,
            BG2PD    => self.bg[2].par_d = value,

            BG3X_L   => self.bg[3].x_ref = (self.bg[3].x_ref & 0x0FFF0000) | (value as u32),
            BG3X_H   => self.bg[3].x_ref =
                (((((self.bg[3].x_ref & 0xFFFF) | (((value as u32) & 0x0FFF0000) << 16)) << 4) as i32) >> 4) as u32,
            BG3Y_L   => self.bg[3].y_ref = (self.bg[3].y_ref & 0x0FFF0000) | (value as u32),
            BG3Y_H   => self.bg[3].y_ref = 
                (((((self.bg[3].y_ref & 0xFFFF) | (((value as u32) & 0x0FFF0000) << 16)) << 4) as i32) >> 4) as u32,
            BG3PA    => self.bg[3].par_a = value,
            BG3PB    => self.bg[3].par_b = value,
            BG3PC    => self.bg[3].par_c = value,
            BG3PD    => self.bg[3].par_d = value,

            WIN0H    => self.win[0].h_off = value,
            WIN0V    => self.win[0].v_off = value,
            WIN1H    => self.win[1].h_off = value,
            WIN1V    => self.win[1].v_off = value,

            WININ    => self.winin = value,
            WINOUT   => self.winout = value,

            BLDCNT   => self.bldcnt = value,
            BLDALPHA => self.bldalpha = value,
            BLDY     => self.bldy = value,

            MOSAIC   => { }
            
            0x0400004E => { } // Not used
            0x04000056 => { } // Not used
            _ => warn!("Unmapped write_u16 to {:08x} (GPU, value={:04x})", address, value),
        }
    }
}

const DISPCNT:  usize = 0x04000000;
const DISPSTAT: usize = 0x04000004;
const VCOUNT:   usize = 0x04000006;

const BG0CNT:   usize = 0x04000008;
const BG1CNT:   usize = 0x0400000A;
const BG2CNT:   usize = 0x0400000C;
const BG3CNT:   usize = 0x0400000E;

const BG0HOFS:  usize = 0x04000010;
const BG0VOFS:  usize = 0x04000012;
const BG1HOFS:  usize = 0x04000014;
const BG1VOFS:  usize = 0x04000016;
const BG2HOFS:  usize = 0x04000018;
const BG2VOFS:  usize = 0x0400001A;
const BG3HOFS:  usize = 0x0400001C;
const BG3VOFS:  usize = 0x0400001E;

const BG2X_L:   usize = 0x04000028;
const BG2X_H:   usize = 0x0400002A;
const BG2Y_L:   usize = 0x0400002C;
const BG2Y_H:   usize = 0x0400002E;
const BG2PA:    usize = 0x04000020;
const BG2PB:    usize = 0x04000022;
const BG2PC:    usize = 0x04000024;
const BG2PD:    usize = 0x04000026;

const BG3X_L:   usize = 0x04000038;
const BG3X_H:   usize = 0x0400003A;
const BG3Y_L:   usize = 0x0400003C;
const BG3Y_H:   usize = 0x0400003E;
const BG3PA:    usize = 0x04000030;
const BG3PB:    usize = 0x04000032;
const BG3PC:    usize = 0x04000034;
const BG3PD:    usize = 0x04000036;

const WIN0H:    usize = 0x04000040;
const WIN1H:    usize = 0x04000042;
const WIN0V:    usize = 0x04000044;
const WIN1V:    usize = 0x04000046;
const WININ:    usize = 0x04000048;
const WINOUT:   usize = 0x0400004A;

const MOSAIC:   usize = 0x0400004C;

const BLDCNT:   usize = 0x04000050;
const BLDALPHA: usize = 0x04000052;
const BLDY:     usize = 0x04000054;
