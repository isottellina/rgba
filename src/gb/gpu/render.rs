// background.rs --- 
// 
// Filename: background.rs
// Author: Louise <louise>
// Created: Fri Dec 15 19:27:05 2017 (+0100)
// Last-Updated: Fri Dec 22 01:11:39 2017 (+0100)
//           By: Louise <louise>
//
use common::Platform;
use gb::gpu::Color;

use gb::gpu::GPU;

impl GPU {   
    pub fn render<T: Platform>(&mut self, platform: &mut T) {
        if let Some(y) = self.render_line {    
            for x in 0..160 {
                let bg_color = if self.window_enable && self.in_window(x, y) {
                    self.get_window(x, y)
                } else if self.bg_enable {
                    self.get_background(x, y)
                } else {
                    0
                };

                let real_color =
                    if self.obj_enable {
                        if let Some(spr_color) = self.get_sprite(x, y, bg_color) {
                            spr_color.as_real()
                        } else {
                            self.bgp[bg_color as usize].as_real()
                        }
                    } else {
                        self.bgp[bg_color as usize].as_real()
                    };
                
                platform.set_pixel(x as u32, y as u32, real_color);
            }
            
            if y == 143 {
                self.frame_done = true;
            }

            self.render_line = None;
        }
    }

    #[inline]
    fn in_window(&self, x: u8, y: u8) -> bool {
        let wx = self.wx as i32;
        let dx = x as i32;

        dx >= (wx - 7) && y >= self.wy
    }
    
    fn tile_get_color(&self, bank: bool, tile: u8, x: u8, y: u8) -> u8 {
        let offset = if bank {
            ((tile as usize) << 4) + ((y as usize) << 1)
        } else {
            let tile = tile as i8;
            let offset = 0x1000 + ((tile as i16) << 4) + ((y as i16) << 1);

            offset as usize
        };

        let data1 = self.vram[offset as usize];
        let data2 = self.vram[offset as usize + 1];
        
        ((data1 >> (7 - x)) & 1) | (((data2 >> (7 - x)) & 1) << 1)
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

        let tile_x = actual_x as u8 & 0x7;
        let tile_y = actual_y as u8 & 0x7;
        
        self.tile_get_color(self.tile_data, tile, tile_x, tile_y)
    }
    
    fn get_background(&self, x: u8, y: u8) -> u8 {
        let actual_x = self.scx.wrapping_add(x) as u16;
        let actual_y = self.scy.wrapping_add(y) as u16;
        
        let tile = if self.bg_map {
            self.vram
                [(0x1C00 + (actual_x >> 3) + ((actual_y & 0xf8) << 2)) as usize]
        } else {
            self.vram
                [(0x1800 + (actual_x >> 3) + ((actual_y & 0xf8) << 2)) as usize]
        };

        let tile_x = actual_x as u8 & 0x7;
        let tile_y = actual_y as u8 & 0x7;
        
        self.tile_get_color(self.tile_data, tile, tile_x, tile_y)
    }

    fn get_sprite(&self, x: u8, y: u8, bg_color: u8) -> Option<Color> {
        let sprites = self.line_cache[y as usize];

        for opt_sprite in sprites.iter() {
            if let &Some(nb_sprite) = opt_sprite {
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
                    } else {
                        if self.obj_size {
                            15 - (y - sprite_y)
                        } else {
                            7 - (y - sprite_y)
                        }
                    };
                    
                    let c = self.tile_get_color(true, tile, tile_x, tile_y);
                    
                    if c == 0 {
                        continue;
                    }
                    
                    return if sprite.dmg_palette {
                        Some(self.obp1[c as usize])
                    } else {
                        Some(self.obp0[c as usize])
                    }
                }
            }
        }

        None
    }
}
