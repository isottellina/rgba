// mode1.rs --- 
// 
// Filename: mode1.rs
// Author: Louise <louise>
// Created: Tue Apr 17 16:52:42 2018 (+0200)
// Last-Updated: Tue Apr 17 18:49:59 2018 (+0200)
//           By: Louise <louise>
// 
use gpu::{GPU, DisplayLine};

impl GPU {
    pub fn render_mode1(&mut self, line_id: u16, line: &mut DisplayLine) {
        line.bg_enabled[0] = true;
        line.bg_enabled[1] = true;
        line.bg_enabled[2] = true;
        line.bg_enabled[3] = false;

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
            self.render_rs_tiles(self.bg[2].cnt, self.bg[2].x_ref, self.bg[2].y_ref,
                                 self.bg[2].par_a, self.bg[2].par_b,
                                 self.bg[2].par_c, self.bg[2].par_d,
                                 line_id, &mut line.bg[2]);
        }
    }
}
