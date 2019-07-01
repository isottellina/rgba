// oam.rs --- 
// 
// Filename: oam.rs
// Author: Louise <louise>
// Created: Mon Dec 18 14:45:17 2017 (+0100)
// Last-Updated: Sat Sep  1 00:54:14 2018 (+0200)
//           By: Louise <ludwigette>
// 
use crate::gpu::GPU;

impl GPU {
    pub fn rebuild_cache(&mut self) {
        for i in self.line_cache.iter_mut() {
            i[0] = None;
            i[1] = None;
            i[2] = None;
            i[3] = None;
            i[4] = None;
            i[5] = None;
            i[6] = None;
            i[7] = None;
            i[8] = None;
            i[9] = None;
        }

        let height = if self.obj_size { 16 } else { 8 };
        for (nb_sprite, sprite) in self.oam.iter().enumerate() {
            let begin = sprite.y.wrapping_sub(16) as usize;
            let end = begin.wrapping_add(height) as usize;

            for y in begin..end {
                if y >= 144 { break; }
                if self.line_cache[y][9].is_some() { break; }

                for i in 0..10 {
                    if self.line_cache[y][i].is_none() {
                        self.line_cache[y][i] = Some(nb_sprite as u8);

                        break;
                    }
                }
            }
        }
    }
}
