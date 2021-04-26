// background.rs --- 
// 
// Filename: background.rs
// Author: Louise <louise>
// Created: Fri Dec 15 19:27:05 2017 (+0100)
// Last-Updated: Thu Jul 12 17:52:38 2018 (+0200)
//           By: Louise <ludwigette>
//
use rgba_common::Platform;

use crate::gpu::GPU;
use crate::gpu::CgbColor;

impl GPU {   
    pub fn render_cgb<T: Platform>(&mut self, platform: &mut T) {
        if let Some(y) = self.render_line {    
            for x in 0..160 {
                let (bg_color, bg_pal) = if self.window_enable && self.in_window(x, y) {
                    self.get_window_cgb(x, y)
                } else if self.bg_enable {
                    self.get_background_cgb(x, y)
                } else {
                    (0, 0)
                };

                let real_color =
                    if self.obj_enable {
                        if let Some(spr_color) = self.get_sprite_cgb(x, y, bg_color) {
                            spr_color.as_real()
                        } else {
                            self.bcpd
                                [bg_pal as usize]
                                [bg_color as usize].as_real()
                        }
                    } else {
                        self.bcpd
                            [bg_pal as usize]
                            [bg_color as usize].as_real()
                    };
                
                self.line_buffer[x as usize] = real_color;
            }

            platform.set_scanline(y as u32, &self.line_buffer);
            
            if y == 143 {
                self.frame_done = true;
            }

            self.render_line = None;
        }
    }
    
    fn tile_get_banked(&self, tile_data: bool, bank: u8, tile: u8, x: u8, y: u8) -> u8 {
        let offset = if tile_data {
            ((tile as usize) << 4) + ((y as usize) << 1)
        } else {
            let tile = tile as i8;
            let offset = 0x1000 + ((tile as i16) << 4) + ((y as i16) << 1);

            offset as usize
        };

        let (data1, data2) = unsafe {
            (self.vram.get_unchecked(((bank as usize) << 13) + offset),
             self.vram.get_unchecked(((bank as usize) << 13) + offset + 1))
        };
        
        ((data1 >> (7 - x)) & 1) | (((data2 >> (7 - x)) & 1) << 1)
    }

    fn get_window_cgb(&self, x: u8, y: u8) -> (u8, u8) {
        let actual_x = x.wrapping_sub(self.wx).wrapping_add(7) as u16;
        let actual_y = y.wrapping_sub(self.wy) as u16;
        
        let offset = if self.window_map {
            (0x1C00 + (actual_x >> 3) + ((actual_y & 0xf8) << 2)) as usize
        } else {
            (0x1800 + (actual_x >> 3) + ((actual_y & 0xf8) << 2)) as usize
        };

        let tile = self.vram[offset];
        let attr = self.vram[0x2000 + offset];

        let bank = (attr & 0x08) >> 3;

        let tile_x = actual_x as u8 & 0x7;
        let tile_y = actual_y as u8 & 0x7;
        
        (self.tile_get_banked(self.tile_data, bank, tile, tile_x, tile_y),
         attr & 0x7)
    }
    
    pub fn get_background_cgb(&self, x: u8, y: u8) -> (u8, u8) {
        let actual_x = self.scx.wrapping_add(x) as u16;
        let actual_y = self.scy.wrapping_add(y) as u16;
        
        let offset = if self.bg_map {
            (0x1C00 + (actual_x >> 3) + ((actual_y & 0xf8) << 2)) as usize
        } else {
            (0x1800 + (actual_x >> 3) + ((actual_y & 0xf8) << 2)) as usize
        };

        let tile = self.vram[offset];
        let attr = self.vram[0x2000 + offset];

        let bank = (attr & 0x08) >> 3;

        let tile_x = actual_x as u8 & 0x7;
        let tile_y = actual_y as u8 & 0x7;
        
        (self.tile_get_banked(self.tile_data, bank, tile, tile_x, tile_y),
         attr & 0x7)
    }

    fn get_sprite_cgb(&self, x: u8, y: u8, bg_color: u8) -> Option<CgbColor> {
        let sprites = self.line_cache[y as usize];

        assert!(sprites.len() == 10);
        
        for opt_sprite in &sprites {
            if let Some(nb_sprite) = *opt_sprite {
                let sprite = self.oam[nb_sprite as usize];

                let sprite_x = sprite.x.wrapping_sub(8);
                let sprite_y = sprite.y.wrapping_sub(16);
                
                if (x >= sprite_x) && (x < (sprite_x + 8)) {
                    if sprite.priority && (bg_color != 0) {
                        continue;
                    }
                    
                    let tile = if self.obj_size {
                        sprite.tile & 0xfe
                    } else {
                        sprite.tile
                    };
                    
                    let tile_x = if !sprite.x_flip {
                        x - sprite_x
                    } else {
                        7 - (x - sprite_x)
                    };
                    
                    let tile_y = if !sprite.y_flip {
                        y - sprite_y
                    } else if self.obj_size {
                        15 - (y - sprite_y)
                    } else {
                        7 - (y - sprite_y)
                    };
                    
                    let c = self.tile_get_banked(true, sprite.cgb_bank, tile, tile_x, tile_y);
                    
                    if c == 0 {
                        continue;
                    }
                    
                    return Some(self.ocpd[sprite.cgb_palette as usize][c as usize]);
                }
            } else {
                return None;
            }
        }

        None
    }
}
