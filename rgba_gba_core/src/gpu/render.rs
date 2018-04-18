// render.rs --- 
// 
// Filename: render.rs
// Author: Louise <louise>
// Created: Tue Jan 30 11:50:54 2018 (+0100)
// Last-Updated: Wed Apr 18 13:26:12 2018 (+0200)
//           By: Louise <louise>
//
use byteorder::{ByteOrder, LittleEndian};

use gpu::GPU;
use gpu::DisplayLine;

use rgba_common::{Platform, Color};

impl GPU {
    #[inline]
    pub fn render<T: Platform>(&mut self, platform: &mut T) {
        if let Some(line) = self.render_line {
            self.render_line = None;

            let mut display_line = DisplayLine {
                bg: [[0; 240]; 4],
                bg_enabled: [false; 4],
                obj: [0; 240],
                obj_data: [0; 240]
            };
            
            match self.dispcnt & 7 {
                0 => self.render_mode0(line, &mut display_line),
                1 => self.render_mode1(line, &mut display_line),
                2 => self.render_mode2(line, &mut display_line),
                4 => self.render_mode4(line, &mut display_line),
                mode => debug!("Rendering unimplemented mode {}", mode),
            }

            let result = self.blend_line(&mut display_line);
            
            for x in 0..240 {
                platform.set_pixel(x, line as u32, expand_color(result[x as usize]));
            }
        }
    }

    pub fn blend_line(&self, display_line: &mut DisplayLine) -> [u16; 240] {
        let mut output: [u16; 240] = [0; 240];
        
        let bg0_enabled = display_line.bg_enabled[0] && ((self.dispcnt & 0x100) != 0);
        let bg1_enabled = display_line.bg_enabled[1] && ((self.dispcnt & 0x200) != 0);
        let bg2_enabled = display_line.bg_enabled[2] && ((self.dispcnt & 0x400) != 0);
        let bg3_enabled = display_line.bg_enabled[3] && ((self.dispcnt & 0x800) != 0);

        let bg0_priority = self.bg[0].cnt & 0x3;
        let bg1_priority = self.bg[1].cnt & 0x3;
        let bg2_priority = self.bg[2].cnt & 0x3;
        let bg3_priority = self.bg[3].cnt & 0x3;

        for px in 0..240 {
            let mut layer = 5;
            
            for i in (0..4).rev() {
                if bg3_enabled && (bg3_priority == i) && ((display_line.bg[3][px] & 0x8000) != 0) {
                    layer = 3;
                }
                
                if bg2_enabled && (bg2_priority == i) && ((display_line.bg[2][px] & 0x8000) != 0) {
                    layer = 2;
                }
                
                if bg1_enabled && (bg1_priority == i) && ((display_line.bg[1][px] & 0x8000) != 0) {
                    layer = 1;
                }
                
                if bg0_enabled && (bg0_priority == i) && ((display_line.bg[0][px] & 0x8000) != 0) {
                    layer = 0;
                }
            }

            if layer == 5 {
                output[px] = LittleEndian::read_u16(&self.pram);
            } else {
                output[px] = display_line.bg[layer][px];
            }
        };

        output
    }
}

fn expand_color(source: u16) -> Color {
    Color (
        ((source as u8)  & 0x1f) << 3,
        (((source >> 5)  & 0x1f) as u8) << 3,
        (((source >> 10) & 0x1f) as u8) << 3
    )
}
