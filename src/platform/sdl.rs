// sdl.rs --- 
// 
// Filename: sdl.rs
// Author: Louise <louise>
// Created: Fri Dec 15 00:00:30 2017 (+0100)
// Last-Updated: Thu Dec 21 13:02:40 2017 (+0100)
//           By: Louise <louise>
//
use common;
use common::{Color, Platform, Key};

use sdl2;
use sdl2::EventPump;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::surface::Surface;
use sdl2::keyboard::Scancode;
use sdl2::video::Window;

pub struct SDLPlatform<'a> {
    height: u32,
    width: u32,
    scale: u32,

    window: Window,
    surface: Surface<'a>,
    event_pump: EventPump,
}

impl<'a> Platform for SDLPlatform<'a> {
    fn new(width: u32, height: u32, scale: u32) -> SDLPlatform<'a> {
        let context = sdl2::init().unwrap();
        let video_sub = context.video().unwrap();
        let surface = Surface::new(width, height, PixelFormatEnum::RGB888)
            .unwrap();
        
        let window = video_sub.window("rGBA", width * scale, height * scale)
            .position_centered()
            .build()
            .unwrap();

        let event_pump = context.event_pump().unwrap();
        
        SDLPlatform {
            width,
            height,
            scale,

            window,
            surface,
            event_pump,
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let width = self.width;
        
        self.surface.with_lock_mut(
            move |array| {
                let i = ((y * width + x) << 2) as usize;
                
                array[i + 2] = color.0;
                array[i + 1] = color.1;
                array[i] = color.2;
            }
        );
    }

    fn present(&mut self) {
        let rect1 = sdl2::rect::Rect::new(0, 0, self.width, self.height);
        let rect2 = sdl2::rect::Rect::new(0, 0,
                                     self.width * self.scale,
                                     self.height * self.scale);
        
        if let Ok(mut surface) = self.window.surface(&self.event_pump) {
            if let Err(e) = self.surface.blit_scaled(rect1,&mut surface,rect2) {
                error!("{}", e);
            }
            
            if let Err(e) = surface.update_window() {
                error!("{}", e);
            }
        }
    }

    fn set_title(&mut self, s: String) {
        if let Err(e) = self.window.set_title(&s) {
            warn!("{}", e);
        }
    }
    
    fn poll_event(&mut self) -> Option<common::Event> {
        match self.event_pump.poll_event() {
            Some(Event::Quit { .. }) => Some(common::Event::Quit),
            Some(Event::KeyDown { scancode: Some(scan), .. }) =>
                match scan {
                    Scancode::F11 => Some(common::Event::Debug),
                    Scancode::F12 => Some(common::Event::Reset),
                    Scancode::Q => Some(common::Event::KeyDown(Key::A)),
                    Scancode::W => Some(common::Event::KeyDown(Key::B)),
                    Scancode::Return =>Some(common::Event::KeyDown(Key::Start)),
                    Scancode::Space =>Some(common::Event::KeyDown(Key::Select)),
                    Scancode::Up => Some(common::Event::KeyDown(Key::Up)),
                    Scancode::Down => Some(common::Event::KeyDown(Key::Down)),
                    Scancode::Right => Some(common::Event::KeyDown(Key::Right)),
                    Scancode::Left => Some(common::Event::KeyDown(Key::Left)),
                    _ => None
                },
            Some(Event::KeyUp { scancode: Some(scan), .. }) =>
                match scan {
                    Scancode::Q => Some(common::Event::KeyUp(Key::A)),
                    Scancode::W => Some(common::Event::KeyUp(Key::B)),
                    Scancode::Return => Some(common::Event::KeyUp(Key::Start)),
                    Scancode::Space => Some(common::Event::KeyUp(Key::Select)),
                    Scancode::Up => Some(common::Event::KeyUp(Key::Up)),
                    Scancode::Down => Some(common::Event::KeyUp(Key::Down)),
                    Scancode::Right => Some(common::Event::KeyUp(Key::Right)),
                    Scancode::Left => Some(common::Event::KeyUp(Key::Left)),
                    _ => None,
                },
            _ => None
        }
    }
}
