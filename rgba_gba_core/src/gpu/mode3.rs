// mode3.rs --- 
// 
// Filename: mode3.rs
// Author: Louise <louise>
// Created: Sun Aug 25 20:18:53 2019 (+0200)
// Last-Updated: Sun Aug 25 21:01:12 2019 (+0200)
//           By: Louise <louise>
// 
use crate::gpu::{GPU, DisplayLine};

impl GPU {
    pub fn render_mode3(&mut self, _line_id: u16, line: &mut DisplayLine) {
        let line_offset = 480 * _line_id as usize;

        line.bg_enabled[0] = false;
        line.bg_enabled[1] = false;
        line.bg_enabled[2] = true;
        line.bg_enabled[3] = false;
        
        for x in 0..240 {
            let offset = line_offset + (x << 1);
            let pix = self.vram_read_u16(offset) | 0x8000;

            line.bg[2][x] = pix;
        }
    }
}
