// tiles.rs --- 
// 
// Filename: tiles.rs
// Author: Louise <louise>
// Created: Tue Jan 30 20:58:44 2018 (+0100)
// Last-Updated: Thu Jul 12 20:45:48 2018 (+0200)
//           By: Louise <ludwigette>
//
use byteorder::{ByteOrder, LittleEndian};
use crate::gpu::{GPU, Background};

const TEXT_SCREEN_SIZE: [(u32, u32); 4] = [
    (256, 256),
    (512, 256),
    (256, 512),
    (512, 512)
];

const RS_SCREEN_SIZE: [(u32, u32); 4] = [
    (128,  128),
    (256,  256),
    (512,  512),
    (1024, 1024)
];

impl GPU {
    // Text-mode tile rendering
    pub fn render_text_tiles(&self, cnt: u16, x_off: u16, y_off: u16, line_id: u16, line: &mut [u16; 240]) {
        let mut column = 0;
        
        let tile_base_block = (((cnt as u32) >> 2) & 3) << 14;
        let map_base_block = (((cnt as u32) >> 8) & 0x1f) << 11;

        // Get function for palette mode
        let copy_fn = if ((cnt >> 7) & 1) == 1 {
            // 8bit
            tile_copy_8bit
        } else {
            // 4bit
            tile_copy_4bit
        };

        let (screen_width, screen_height) = TEXT_SCREEN_SIZE[((cnt >> 14) & 3) as usize];

        let screen_y = ((y_off as u32) + (line_id as u32)) & (screen_height - 1);

        // Draw first tile that may not be entirely in the screen
        {
            let screen_x = (x_off as u32) & (screen_width - 1);
            
            let screen_chunk = ((screen_x >> 8) & 1) + (((screen_y >> 8) & 1) << (screen_width >> 9));
            
            let tile_x = (screen_x & 255) >> 3;
            let tile_y = (screen_y & 255) >> 3;

            let map_tile_addr = map_base_block + (screen_chunk << 11) + (tile_y << 6) + (tile_x << 1);
            if map_tile_addr < 0xFFFF {
                let tile_info = LittleEndian::read_u16(&self.vram[(map_tile_addr as usize)..]);
                let tile_mem = &self.vram[(tile_base_block as usize)..0xFFFF];
                
                copy_fn(tile_mem, &self.pram[0x000..0x200], &mut line[0..(8 - (screen_x & 7)) as usize],
                        tile_info, screen_x & 7, screen_y & 7);
            }

            column += screen_x & 7;
        }

        // Draw the other tiles
        while column < 232 {
            let screen_x = (column + (x_off as u32)) & (screen_width - 1);
            
            let screen_chunk = ((screen_x >> 8) & 1) + (((screen_y >> 8) & 1) << (screen_width >> 9));
            
            let tile_x = (screen_x & 255) >> 3;
            let tile_y = (screen_y & 255) >> 3;

            let map_tile_addr = map_base_block + (screen_chunk << 11) + (tile_y << 6) + (tile_x << 1);
            if map_tile_addr < 0xFFFF {
                let tile_info = LittleEndian::read_u16(&self.vram[(map_tile_addr as usize)..]);
                let tile_mem = &self.vram[(tile_base_block as usize)..0xFFFF];
                
                copy_fn(tile_mem, &self.pram[0x000..0x200],
                        &mut line[(column as usize)..(column + (8 - (screen_x & 7))) as usize],
                        tile_info, 0, screen_y & 7);
            }

            column += 8;
        }

        // Draw last tile
        {
            let screen_x = (column + (x_off as u32)) & (screen_width - 1);
            let screen_chunk = ((screen_x >> 8) & 1) + (((screen_y >> 8) & 1) << (screen_width >> 9));
            
            let tile_x = (screen_x & 255) >> 3;
            let tile_y = (screen_y & 255) >> 3;

            let map_tile_addr = map_base_block + (screen_chunk << 11) + (tile_y << 6) + (tile_x << 1);
            
            if map_tile_addr < 0xFFFF {
                let tile_info = LittleEndian::read_u16(&self.vram[(map_tile_addr as usize)..]);
                let tile_mem = &self.vram[(tile_base_block as usize)..0xFFFF];
                
                copy_fn(tile_mem, &self.pram[0x000..0x200],
                        &mut line[(column as usize)..((column as usize) + ((240 - column) as usize))],
                        tile_info, 0, screen_y & 7);
            }
        }
    }

    // Rotation/scaling-mode tile rendering
    pub fn render_rs_tiles(&mut self, back: &mut Background, line: &mut [u16; 240]) {
        let mut x = back.x_ref;
        let mut y = back.y_ref;

        let dx = ((back.par_a as i16) as i32) as u32;
        let dy = ((back.par_c as i16) as i32) as u32;
        let dmx = ((back.par_b as i16) as i32) as u32;
        let dmy = ((back.par_d as i16) as i32) as u32;

        let tile_base_block = (((back.cnt as u32) >> 2) & 3) << 14;
        let map_base_block = (((back.cnt as u32) >> 8) & 0x1f) << 11;
        let wraparound = ((back.cnt >> 13) & 0x1) == 1;

        let (screen_width, screen_height) = RS_SCREEN_SIZE[((back.cnt >> 14) & 3) as usize];

        if wraparound {
            for r_x in 0..240 {
		let pixel_x = (x >> 8) & (screen_width - 1);
		let pixel_y = (y >> 8) & (screen_height - 1);
                
		let tile_x = pixel_x >> 3;
		let tile_y = pixel_y >> 3;
		let screen_data_offset = (tile_y * (screen_width >> 3)) + tile_x;
		let tile_number = self.vram[(map_base_block as usize) + screen_data_offset as usize];
                
		let tx = pixel_x & 7;
		let ty = pixel_y & 7;
                
		let dot_offset = (((tile_number as u32) << 6) + (ty << 3) + tx) as usize;
		let dot = self.vram[(tile_base_block as usize) + dot_offset];
                
		if dot == 0 {
		    line[r_x as usize] = 0;
		} else {
		    line[r_x as usize] = LittleEndian::read_u16(&self.pram[((dot as usize) << 1)..]) | 0x8000;
		}
                
		x += dx;
		y += dy;
            }
        } else {
            for r_x in 0..240 {
		let pixel_x = x >> 8;
		let pixel_y = y >> 8;

                if (pixel_x > 0) && (pixel_y > 0) && (pixel_x < screen_width) && (pixel_y < screen_height) {
		    let tile_x = pixel_x >> 3;
		    let tile_y = pixel_y >> 3;
		    let screen_data_offset = (tile_y * (screen_width >> 3)) + tile_x;
		    let tile_number = self.vram[(map_base_block as usize) + screen_data_offset as usize];
                    
		    let tx = pixel_x & 7;
		    let ty = pixel_y & 7;
                    
		    let dot_offset = (((tile_number as u32) << 6) + (ty << 3) + tx) as usize;
		    let dot = self.vram[(tile_base_block as usize) + dot_offset];
                    
		    if dot == 0 {
		        line[r_x as usize] = 0;
		    } else {
		        line[r_x as usize] = LittleEndian::read_u16(&self.pram[((dot as usize) << 1)..]) | 0x8000;
		    }
                } else {
                    line[r_x as usize] = 0;
                }
                
		x = x.wrapping_add(dx);
		y = y.wrapping_add(dy);
            }
        }

        back.x_ref = back.x_ref.wrapping_add(dmx);
        back.y_ref = back.y_ref.wrapping_add(dmy);
    }
}

fn tile_copy_4bit(tile_data: &[u8], palette: &[u8], output: &mut [u16], tile_info: u16, x: u32, y: u32) {
    let aligned = (x & 1) == 0;
    let mut tx = x;
    let mut ty = y;

    let tile_nb = tile_info & 0x3ff;

    let x_flip = ((tile_info >> 10) & 1) == 1;
    let y_flip = ((tile_info >> 11) & 1) == 1;
    let palette_nb = (tile_info >> 12) & 0xf;

    let (left_shift, right_shift, offset_inc) = if x_flip {
	tx = 7 - tx;
        (4, 0, usize::max_value())
    } else {
        (0, 4, 1)
    };

    if y_flip {
	ty = 7 - ty;
    }

    let mut offset = (((tile_nb as u32) << 5) + (ty << 2) + (tx >> 1)) as usize;
    let mut pindex = 0;

    if !aligned {
	let both_dots = tile_data[offset];
	let right_dot = (both_dots >> right_shift) & 0xf;
        
	if right_dot == 0 {
	    output[pindex] = 0
	} else {
	    output[pindex] = LittleEndian::read_u16(
                &palette[((palette_nb << 5) + ((right_dot << 1) as u16)) as usize..]
            ) | 0x8000;
	}
        
	pindex += 1;
	offset = offset.wrapping_add(offset_inc);
    }
    
    while (pindex < output.len()) && (offset < tile_data.len()) {
	let two_dots = tile_data[offset];

	let left_dot = (two_dots >> left_shift) & 0xf;
	if left_dot == 0 {
	    output[pindex] = 0;
	} else {
	    output[pindex] = LittleEndian::read_u16(
                &palette[((palette_nb << 5) + ((left_dot << 1) as u16)) as usize..]
            ) | 0x8000;
	}
        
	pindex += 1;

	if pindex >= output.len() { break; }

	// right pixel
	let right_dot = (two_dots >> right_shift) & 0xf;
	if right_dot == 0 {
	    output[pindex] = 0;
	} else {
	    output[pindex] = LittleEndian::read_u16(
                &palette[((palette_nb << 5) + ((right_dot << 1) as u16)) as usize..]
            ) | 0x8000;
	}
        
	pindex += 1;

	offset = offset.wrapping_add(offset_inc);
    }
}

fn tile_copy_8bit(tile_data: &[u8], palette: &[u8], output: &mut [u16], tile_info: u16, x: u32, y: u32) {
    let mut tx = x;
    let mut ty = y;

    let tile_nb = tile_info & 0x3ff;

    let x_flip = ((tile_info >> 10) & 1) == 1;
    let y_flip = ((tile_info >> 11) & 1) == 1;

    let offset_inc = if x_flip {
	tx = 7 - tx;
        usize::max_value()
    } else {
        1
    };

    if y_flip {
	ty = 7 - ty;
    }

    let mut offset = (((tile_nb as u32) << 6) + (ty << 3) + tx) as usize;

    if offset >= tile_data.len() {
        return;
    }

    let old_max = tile_data.len() - offset;
    let max = if old_max < output.len() { old_max } else { output.len() };
    
    for pindex in 0..max {
        let dot = tile_data[offset];

        if dot == 0 {
            output[pindex] = 0;
        } else {
            output[pindex] = LittleEndian::read_u16(&palette[((dot as usize) << 1)..]);
        }

        offset = offset.wrapping_add(offset_inc);
    }
}
