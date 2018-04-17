// mode4.rs --- 
// 
// Filename: mode4.rs
// Author: Louise <louise>
// Created: Wed Jan 31 00:34:55 2018 (+0100)
// Last-Updated: Tue Apr 17 16:40:42 2018 (+0200)
//           By: Louise <louise>
// 
use byteorder::{ByteOrder, LittleEndian};
use gpu::GPU;
use gpu::DisplayLine;

impl GPU {
    pub fn render_mode4(&mut self, line_id: u16, line: &mut DisplayLine) {
        line.bg_enabled[0] = false;
        line.bg_enabled[1] = false;
        line.bg_enabled[2] = true;
        line.bg_enabled[3] = false;

        let base_addr = if ((self.dispcnt >> 4) & 1) != 0 { 0xA000 } else { 0x0000 };
        let offset = (line_id as usize) * 240;

        for x in 0..240 {
            let x_off = offset + x;
            let color = self.vram[base_addr + x_off];

            line.bg[2][x] = LittleEndian::read_u16(
                &self.pram[((color as usize) << 1)..]
            ) | 0x8000;
        }
    }
}
