// sdl.rs --- 
// 
// Filename: sdl.rs
// Author: Louise <louise>
// Created: Fri Dec 15 00:00:30 2017 (+0100)
// Last-Updated: Fri Dec 22 03:58:41 2017 (+0100)
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
use sdl2::audio::{AudioSpecDesired, AudioQueue};

pub struct SDLPlatform {
    height: u32,
    width: u32,
    scale: u32,

    window: Window,
    video_data: Box<[u8]>,

    queue: AudioQueue<i16>,
    
    event_pump: EventPump,
}

impl Platform for SDLPlatform {
    fn new(width: u32, height: u32, scale: u32) -> SDLPlatform {
        let context = sdl2::init().unwrap();
        let video_sub = context.video().unwrap();
        let audio_sub = context.audio().unwrap();
        
        let video_data = vec![0; ((width * height) << 2) as usize]
            .into_boxed_slice();
        
        let window = video_sub.window("rGBA", width * scale, height * scale)
            .position_centered()
            .build()
            .unwrap();

        let event_pump = context.event_pump().unwrap();

        let queue = audio_sub.open_queue(None,
                                         &AudioSpecDesired {
                                             freq: Some(48_000),
                                             channels: Some(2),
                                             samples: Some(4)
                                         }
        ).unwrap();
        
        SDLPlatform {
            width,
            height,
            scale,

            window,
            video_data,
            queue,
            event_pump,
        }
    }

    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let width = self.width;
        let i = ((y * width + x) << 2) as usize;
        
        self.video_data[i + 2] = color.0;
        self.video_data[i + 1] = color.1;
        self.video_data[i] = color.2;
    }

    fn present(&mut self) {
        let rect1 = sdl2::rect::Rect::new(0, 0, self.width, self.height);
        let rect2 = sdl2::rect::Rect::new(0, 0,
                                          self.width * self.scale,
                                          self.height * self.scale);
        
        if let Ok(mut window_surface) = self.window.surface(&self.event_pump) {
            let surface = Surface::from_data(&mut self.video_data,
                                             self.width, self.height,
                                             self.width * 4,
                                             PixelFormatEnum::RGB888)
                .unwrap();
            
            if let Err(e) = surface.blit_scaled(rect1,
                                                &mut window_surface,
                                                rect2) {
                error!("{}", e);
            }
            
            if let Err(e) = window_surface.update_window() {
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
