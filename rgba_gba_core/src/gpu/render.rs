// render.rs --- 
// 
// Filename: render.rs
// Author: Louise <louise>
// Created: Tue Jan 30 11:50:54 2018 (+0100)
// Last-Updated: Mon Aug 26 00:11:53 2019 (+0200)
//           By: Louise <louise>
//
use byteorder::{ByteOrder, LittleEndian};

use crate::gpu::GPU;
use crate::gpu::DisplayLine;

impl GPU {
	#[inline]
	pub fn render(&mut self) {
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
				3 => self.render_mode3(line, &mut display_line),
				4 => self.render_mode4(line, &mut display_line),
				mode => debug!("Rendering unimplemented mode {}", mode),
			}

			let result = self.blend_line(line, &mut display_line);
			
			for x in 0..240 {
				self.framebuffer[line as usize * 240 + x] = expand_color(result[x as usize]);
			}
		}
	}

	pub fn blend_line(&self, line: u16, display_line: &mut DisplayLine) -> [u16; 240] {
		let mut output: [u16; 240] = [0; 240];
		
		let bg0_enabled = display_line.bg_enabled[0] && ((self.dispcnt & 0x100) != 0);
		let bg1_enabled = display_line.bg_enabled[1] && ((self.dispcnt & 0x200) != 0);
		let bg2_enabled = display_line.bg_enabled[2] && ((self.dispcnt & 0x400) != 0);
		let bg3_enabled = display_line.bg_enabled[3] && ((self.dispcnt & 0x800) != 0);

		let bg0_priority = self.bg[0].cnt & 0x3;
		let bg1_priority = self.bg[1].cnt & 0x3;
		let bg2_priority = self.bg[2].cnt & 0x3;
		let bg3_priority = self.bg[3].cnt & 0x3;

		let win0_x0 = ((self.win[0].h_off) >> 8) & 0xff;
		let win0_x1 = if (self.win[0].h_off & 0xff) > 240 { 240 } else { self.win[0].h_off & 0xff };
		let win0_y0 = ((self.win[0].v_off) >> 8) & 0xff;
		let win0_y1 = if (self.win[0].v_off & 0xff) > 240 { 240 } else { self.win[0].v_off & 0xff };

		let win1_x0 = ((self.win[1].h_off) >> 8) & 0xff;
		let win1_x1 = if (self.win[1].h_off & 0xff) > 240 { 240 } else { self.win[1].h_off & 0xff };
		let win1_y0 = ((self.win[1].v_off) >> 8) & 0xff;
		let win1_y1 = if (self.win[1].v_off & 0xff) > 240 { 240 } else { self.win[1].v_off & 0xff };

		let win0_en = (self.dispcnt & 0x2000) == 0x2000;
		let win1_en = (self.dispcnt & 0x4000) == 0x4000;

		let win0_in = self.winin & 0x3f;
		let win1_in = (self.winin >> 8) & 0x3f;
		let winout = self.winout & 0x3f;

		let backdrop = LittleEndian::read_u16(&self.pram);
		
		for px in 0..240 {
			let win_bits = if win0_en || win1_en {
				if win0_en && (px >= win0_x0) && (px < win0_x1) && (line >= win0_y0) && (line < win0_y1) {
					win0_in
				} else if win1_en && (px >= win1_x0) && (px < win1_x1) && (line >= win1_y0) && (line < win1_y1) {
					win1_in
				} else {
					winout
				}
			} else {
				0x3f
			};
			
			output[px as usize] = backdrop;
			
			for i in (0..4).rev() {
				if bg3_enabled && (bg3_priority == i) && ((display_line.bg[3][px as usize] & 0x8000) != 0) && (win_bits & 8) == 8 {
					output[px as usize] = display_line.bg[3][px as usize];
				}
				
				if bg2_enabled && (bg2_priority == i) && ((display_line.bg[2][px as usize] & 0x8000) != 0) && (win_bits & 4) == 4 {
					output[px as usize] = display_line.bg[2][px as usize];
				}
				
				if bg1_enabled && (bg1_priority == i) && ((display_line.bg[1][px as usize] & 0x8000) != 0) && (win_bits & 2) == 2 {
					output[px as usize] = display_line.bg[1][px as usize];
				}
				
				if bg0_enabled && (bg0_priority == i) && ((display_line.bg[0][px as usize] & 0x8000) != 0) && (win_bits & 1) == 1 {
					output[px as usize] = display_line.bg[0][px as usize];
				}
			}
		};

		output
	}
}

fn expand_color(source: u16) -> u32 {
	u32::from_be_bytes (
		[
			0,
			((source as u8)  & 0x1f) << 3,
			(((source >> 5)  & 0x1f) as u8) << 3,
			(((source >> 10) & 0x1f) as u8) << 3
		]
	)
}
