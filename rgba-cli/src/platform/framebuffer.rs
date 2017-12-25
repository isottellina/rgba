// minifb.rs --- 
// 
// Filename: minifb.rs
// Author: Louise <louise>
// Created: Tue Dec 19 23:31:25 2017 (+0100)
// Last-Updated: Mon Dec 25 19:25:54 2017 (+0100)
//           By: Louise <louise>
//
use minifb::{Window, WindowOptions, Scale};
use rgba_common::Platform;
use rgba_common::Color;
use rgba_common;

pub struct FramebufferPlatform {
    buffer: Box<[u32]>,
    window: Window,

    width: u32
}

impl Platform for FramebufferPlatform {
    fn new(width: u32, height: u32, _scale: u32) -> FramebufferPlatform {
        let buffer: Vec<u32> = vec![0; (width * height) as usize];
        
        let window = Window::new("rGBA", width as usize, height as usize,
                                 WindowOptions {
                                     borderless: false,
                                     title: true,
                                     resize: false,
                                     scale: Scale::X2
                                 }
        ).unwrap();
        
        FramebufferPlatform {
            buffer: buffer.into_boxed_slice(),
            window,
            width
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let color_rgba = ((color.0 as u32) << 16) | ((color.1 as u32) << 8) |
                                                     (color.2 as u32);

        self.buffer[(y * self.width + x) as usize] = color_rgba;
    }

    fn present(&mut self) {
        if self.window.is_open() {
            if let Err(e) = self.window.update_with_buffer(&self.buffer) {
                warn!("Couldn't update window with buffer : {}", e);
            }
        }
    }

    fn poll_event(&mut self) -> Option<rgba_common::Event> {
        None
    }
    
    fn set_title(&mut self, s: String) {
        self.window.set_title(&s);
    }
}
