// sdl.rs --- 
// 
// Filename: sdl.rs
// Author: Louise <louise>
// Created: Fri Dec 15 00:00:30 2017 (+0100)
// Last-Updated: Wed Jun 12 14:38:48 2019 (+0200)
//           By: Louise <ludwigette>
//
use rgba_common;
use rgba_common::{Color, Platform, Key};

use std::fs::OpenOptions;
use memmap::MmapMut;

use libc::{
    // Types
    c_int,

    // Functions
    open,
    ioctl,

    // Constants
    O_RDONLY,
};

pub struct BarePlatform {
    height: u32,
    width: u32,

    fb_file: File,
    fb_buf: MmapMut,
}

impl Platform for BarePlatform {
    fn new(width: u32, height: u32, scale: u32) -> Self {
        let fb_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/fb0");

        let fb_buf = unsafe {
            MmapMut::map_mut(&fb_file).unwrap()
        };
        
        Self {
            width: 0,
            height: 0,

            fb_file,
            fb_buf,
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let width = self.width;
        let i = (y * width + x) as usize;
        let color16: u16 = (((color.0 >> 3) as u16) << 11) | (((color.1 >> 2)  as u16) << 5) | (color.2 as u16) >> 3;

        unsafe {
            let ptr = (self.fb_buf.as_ptr() as *mut u16).add(i);
            ptr.write_volatile(color16);
        }
    }

    fn present(&mut self) {
        
    }

    fn set_title(&mut self, s: String) {
        
    }

    fn queue_samples(&mut self, samples: &[i16]) {
        
    }
    
    fn poll_event(&mut self) -> Option<rgba_common::Event> {
        None
    }

    fn read_line(&self, prompt: &str) -> Option<String> {
        None
    }
}
