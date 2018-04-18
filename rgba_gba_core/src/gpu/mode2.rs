// mode2.rs --- 
// 
// Filename: mode2.rs
// Author: Louise <louise>
// Created: Tue Apr 17 12:17:20 2018 (+0200)
// Last-Updated: Wed Apr 18 14:18:08 2018 (+0200)
//           By: Louise <louise>
// 
use gpu::{GPU, DisplayLine};

impl GPU {
    pub fn render_mode2(&mut self, _line_id: u16, line: &mut DisplayLine) {
        line.bg_enabled[0] = false;
        line.bg_enabled[1] = false;
        line.bg_enabled[2] = true;
        line.bg_enabled[3] = true;

        // BG2
        if (self.dispcnt & 0x400) != 0 {
            let mut bg = self.bg[2].to_owned();
            
            self.render_rs_tiles(&mut bg, &mut line.bg[2]);
            self.bg[2] = bg;
        }
        
        // BG3
        if (self.dispcnt & 0x800) != 0 {
            let mut bg = self.bg[3].to_owned();
     
            self.render_rs_tiles(&mut bg, &mut line.bg[3]);

            self.bg[3] = bg;
        }
    }
}
