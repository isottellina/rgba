// background.rs --- 
// 
// Filename: background.rs
// Author: Louise <louise>
// Created: Fri Dec 15 19:27:05 2017 (+0100)
// Last-Updated: Thu Jul 12 17:53:16 2018 (+0200)
//           By: Louise <ludwigette>
//
use rgba_common::Platform;

use crate::gpu::GPU;

impl GPU {   
	pub fn render_dmg<T: Platform>(&mut self, platform: &mut T) {
		if let Some(y) = self.render_line {
			let mut bg_fifo: [u8; 160] = [0; 160];
			let tile_y = (y + self.scy) & 7;
			let mut x = 0;

			// Draw the first tile
			if self.scx & 7 != 0{
				let tile = self.get_background(x, y as usize);
				let tile_limit = (self.scx & 7) as usize;

				bg_fifo[0..tile_limit].copy_from_slice(&self.tile_get(self.tile_data, tile, tile_y as usize)[(8 - tile_limit)..8]);

				x = tile_limit;
			}

			// Draw all the other tiles
			while x < 153 {
				let tile = self.get_background(x, y as usize);
				bg_fifo[x..x+8].copy_from_slice(&self.tile_get(self.tile_data, tile, tile_y as usize));

				x += 8;
			}

			// Draw the last one
			if x != 160 {
				let tile = self.get_background(x, y as usize);
				let tile_limit = (160 - x) as usize;

				bg_fifo[x..160].copy_from_slice(&self.tile_get(self.tile_data, tile, tile_y as usize)[0..tile_limit]);
			}

			// Merge FIFOs into scanline
			{
				for x in 0..160 {
					self.line_buffer[x] = self.bgp[bg_fifo[x] as usize].as_real();
				}
			}
			platform.set_scanline(y as u32, &self.line_buffer);

			if y == 143 {
				self.frame_done = true;
			}

			self.render_line = None;
		}
	}

	#[inline]
	pub fn in_window(&self, x: u8, y: u8) -> bool {
		let wx = self.wx as i32;
		let dx = x as i32;

		dx >= (wx - 7) && y >= self.wy
	}
	
	fn tile_get(&self, tile_data: bool, tile: u8, y: usize) -> [u8; 8] {
		let offset = if tile_data {
			((tile as usize) << 4) + (y << 1)
		} else {
			let tile = tile as i8;
			let offset = 0x1000 + ((tile as i16) << 4) + ((y as i16) << 1);

			offset as usize
		};

		let (data1, data2) = unsafe {
			(self.vram.get_unchecked(offset as usize),
			 self.vram.get_unchecked(offset as usize + 1))
		};
		
		[
			((data1 >> 7) & 1) | (((data2 >> 7) & 1) << 1),
			((data1 >> 6) & 1) | (((data2 >> 6) & 1) << 1),
			((data1 >> 5) & 1) | (((data2 >> 5) & 1) << 1),
			((data1 >> 4) & 1) | (((data2 >> 4) & 1) << 1),
			((data1 >> 3) & 1) | (((data2 >> 3) & 1) << 1),
			((data1 >> 2) & 1) | (((data2 >> 2) & 1) << 1),
			((data1 >> 1) & 1) | (((data2 >> 1) & 1) << 1),
			(data1 & 1) | ((data2 & 1) << 1),
		]
	}

	fn get_window(&self, x: u8, y: u8) -> u8 {
		let actual_x = x.wrapping_sub(self.wx).wrapping_add(7) as u16;
		let actual_y = y.wrapping_sub(self.wy) as u16;
		
		let tile = if self.window_map {
			self.vram
				[(0x1C00 + (actual_x >> 3) + ((actual_y & 0xf8) << 2)) as usize]
		} else {
			self.vram
				[(0x1800 + (actual_x >> 3) + ((actual_y & 0xf8) << 2)) as usize]
		};

		tile
	}
	
	pub fn get_background(&self, x: usize, y: usize) -> u8 {
		let actual_x = (self.scx as usize).wrapping_add(x);
		let actual_y = (self.scy as usize).wrapping_add(y);
		
		if self.bg_map {
			self.vram
				[0x1C00 + (actual_x >> 3) + ((actual_y & 0xf8) << 2)]
		} else {
			self.vram
				[0x1800 + (actual_x >> 3) + ((actual_y & 0xf8) << 2)]
		}
	}

	// fn get_sprite(&self, x: u8, y: u8, bg_color: u8) -> Option<DmgColor> {
	// 	let sprites = self.line_cache[y as usize];
		
	// 	for opt_sprite in &sprites {
	// 		if let Some(nb_sprite) = *opt_sprite {
	// 			let sprite = self.oam[nb_sprite as usize];

	// 			let sprite_x = sprite.x.wrapping_sub(8);
	// 			let sprite_y = sprite.y.wrapping_sub(16);
				
	// 			if (x >= sprite_x) && (x < (sprite_x + 8)) {
	// 				if sprite.priority && (bg_color != 0) {
	// 					continue;
	// 				}
					
	// 				let tile = if self.obj_size {
	// 					sprite.tile & 0xfe
	// 				} else {
	// 					sprite.tile
	// 				};
					
	// 				let tile_x = if !sprite.x_flip {
	// 					x - sprite_x
	// 				} else {
	// 					7 - (x - sprite_x)
	// 				};
					
	// 				let tile_y = if !sprite.y_flip {
	// 					y - sprite_y
	// 				} else if self.obj_size {
	// 					15 - (y - sprite_y)
	// 				} else {
	// 					7 - (y - sprite_y)
	// 				};
					
	// 				let c = self.tile_get(true, tile, tile_x, tile_y);
					
	// 				if c == 0 {
	// 					continue;
	// 				}
					
	// 				return if sprite.dmg_palette {
	// 					Some(self.obp1[c as usize])
	// 				} else {
	// 					Some(self.obp0[c as usize])
	// 				}
	// 			}
	// 		} else {
	// 			return None;
	// 		}
	// 	}

	// 	None
	// }
}
