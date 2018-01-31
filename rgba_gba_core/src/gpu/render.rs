// render.rs --- 
// 
// Filename: render.rs
// Author: Louise <louise>
// Created: Tue Jan 30 11:50:54 2018 (+0100)
// Last-Updated: Wed Jan 31 01:21:12 2018 (+0100)
//           By: Louise <louise>
// 
use gpu::GPU;
use gpu::DisplayLine;

use rgba_common::{Platform, Color};

impl GPU {
    #[inline]
    pub fn render<T: Platform>(&mut self, platform: &mut T) {
        if let Some(line) = self.render_line {
            self.render_line = None;

            let mut display_line = DisplayLine {
                bg0: [0; 240],
                bg1: [0; 240],
                bg2: [0; 240],
                bg3: [0; 240],

                bg_enabled: [false; 4],
                obj: [0; 240],
                obj_data: [0; 240]
            };
            
            match self.dispcnt & 7 {
                0 => self.render_mode0(line, &mut display_line),
                4 => self.render_mode4(line, &mut display_line),
                mode => debug!("Rendering unimplemented mode {}", mode),
            }
            
            for x in 0..240 {
                platform.set_pixel(x, line as u32, expand_color(display_line.bg2[x as usize]));
            }
        }
    }
}

fn expand_color(source: u16) -> Color {
    Color (
        ((source as u8)  & 0x1f) << 3,
        (((source >> 5)  & 0x1f) as u8) << 3,
        (((source >> 10) & 0x1f) as u8) << 3
    )
}
