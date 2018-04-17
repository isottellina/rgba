// mode2.rs --- 
// 
// Filename: mode2.rs
// Author: Louise <louise>
// Created: Tue Apr 17 12:17:20 2018 (+0200)
// Last-Updated: Tue Apr 17 13:37:17 2018 (+0200)
//           By: Louise <louise>
// 
use gpu::{GPU, DisplayLine};

impl GPU {
    pub fn render_mode2(&mut self, line_id: u16, line: &mut DisplayLine) {
        line.bg_enabled[0] = false;
        line.bg_enabled[1] = false;
        line.bg_enabled[2] = true;
        line.bg_enabled[3] = true;

        // BG2
        /* if (self.dispcnt & 0x400) != 0 {
            self.render_text_tiles(self.bg[2].cnt, self.bg[2].h_off,
                                   self.bg[2].v_off, line_id, &mut line.bg[2]);
        }
        
        // BG3
        if (self.dispcnt & 0x800) != 0 {
            self.render_text_tiles(self.bg[3].cnt, self.bg[3].h_off,
                                   self.bg[3].v_off, line_id, &mut line.bg[3]);
        } */
    }
}
