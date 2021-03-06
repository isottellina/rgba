// mode0.rs --- 
// 
// Filename: mode0.rs
// Author: Louise <louise>
// Created: Tue Jan 30 11:59:23 2018 (+0100)
// Last-Updated: Tue Apr 17 13:34:35 2018 (+0200)
//           By: Louise <louise>
// 
use crate::gpu::{GPU, DisplayLine};

impl GPU {
    pub fn render_mode0(&mut self, line_id: u16, line: &mut DisplayLine) {
        line.bg_enabled[0] = true;
        line.bg_enabled[1] = true;
        line.bg_enabled[2] = true;
        line.bg_enabled[3] = true;

        // BG0
        if (self.dispcnt & 0x100) != 0 {
            self.render_text_tiles(self.bg[0].cnt, self.bg[0].h_off,
                                   self.bg[0].v_off, line_id, &mut line.bg[0]);
        }
        
        // BG1
        if (self.dispcnt & 0x200) != 0 {
            self.render_text_tiles(self.bg[1].cnt, self.bg[1].h_off,
                                   self.bg[1].v_off, line_id, &mut line.bg[1]);
        }
        
        // BG2
        if (self.dispcnt & 0x400) != 0 {
            self.render_text_tiles(self.bg[2].cnt, self.bg[2].h_off,
                                   self.bg[2].v_off, line_id, &mut line.bg[2]);
        }
        
        // BG3
        if (self.dispcnt & 0x800) != 0 {
            self.render_text_tiles(self.bg[3].cnt, self.bg[3].h_off,
                                   self.bg[3].v_off, line_id, &mut line.bg[3]);
        }
    }
}
