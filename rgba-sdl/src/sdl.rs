// sdl.rs --- 
// 
// Filename: sdl.rs
// Author: Louise <louise>
// Created: Fri Dec 15 00:00:30 2017 (+0100)
// Last-Updated: Mon Jul  1 12:46:25 2019 (+0200)
//           By: Louise <ludwigette>
//
use rgba_common::{Platform, Key};

use sdl2::EventPump;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::surface::Surface;
use sdl2::keyboard::Scancode;
use sdl2::video::Window;
use sdl2::audio::{AudioSpecDesired, AudioQueue};

use rustyline::Editor;
use log::warn;

pub struct SDLPlatform {
    height: u32,
    width: u32,
    scale: u32,

    window: Window,
    video_data: Box<[u8]>,

    audio_device: AudioQueue<i16>,
    
    event_pump: EventPump,
    rl: Editor::<()>,
}

impl SDLPlatform {
    pub fn new(width: u32, height: u32, scale: u32) -> SDLPlatform {
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

        let audio_device = audio_sub.open_queue(None,
                                         &AudioSpecDesired {
                                             freq: Some(48_100),
                                             channels: Some(1),
                                             samples: Some(4096)
                                         }
        ).unwrap();

        audio_device.resume();

        let rl = Editor::<()>::new();

        SDLPlatform {
            width,
            height,
            scale,

            window,
            video_data,
            audio_device,
            event_pump,
            rl,
        }
    }

    pub fn set_title(&mut self, s: String) {
        if let Err(e) = self.window.set_title(&s) {
            warn!("{}", e);
        }
    }

    pub fn set_buffer(&mut self, buffer: &[u32]) {
        self.video_data.copy_from_slice(
            unsafe {
                std::slice::from_raw_parts(
                    buffer.as_ptr() as *const u8,
                    buffer.len() as usize * 4,
                )
            }
        );
    }

    pub fn present(&mut self) {
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

            surface.blit_scaled(rect1,
                &mut window_surface,
                rect2
            ).expect("Couldn't blit surface");


            window_surface.update_window().expect("Couldn't update window");
        }
    }

    pub fn poll_event(&mut self) -> Option<rgba_common::Event> {
        match self.event_pump.poll_event() {
            Some(Event::Quit { .. }) => Some(rgba_common::Event::Quit),
            Some(Event::KeyDown { scancode: Some(scan), .. }) =>
                match scan {
                    Scancode::F10 =>
                        Some(rgba_common::Event::FastMode),
                    Scancode::F11 =>
                        Some(rgba_common::Event::Debug),
                    Scancode::F12 =>
                        Some(rgba_common::Event::Reset),
                    Scancode::Q =>
                        Some(rgba_common::Event::KeyDown(Key::A)),
                    Scancode::W =>
                        Some(rgba_common::Event::KeyDown(Key::B)),
                    Scancode::Return =>
                        Some(rgba_common::Event::KeyDown(Key::Start)),
                    Scancode::Space =>
                        Some(rgba_common::Event::KeyDown(Key::Select)),
                    Scancode::Up =>
                        Some(rgba_common::Event::KeyDown(Key::Up)),
                    Scancode::Down =>
                        Some(rgba_common::Event::KeyDown(Key::Down)),
                    Scancode::Right =>
                        Some(rgba_common::Event::KeyDown(Key::Right)),
                    Scancode::Left =>
                        Some(rgba_common::Event::KeyDown(Key::Left)),
                    _ => None
                },
            Some(Event::KeyUp { scancode: Some(scan), .. }) =>
                match scan {
                    Scancode::Q =>
                        Some(rgba_common::Event::KeyUp(Key::A)),
                    Scancode::W =>
                        Some(rgba_common::Event::KeyUp(Key::B)),
                    Scancode::Return =>
                        Some(rgba_common::Event::KeyUp(Key::Start)),
                    Scancode::Space =>
                        Some(rgba_common::Event::KeyUp(Key::Select)),
                    Scancode::Up =>
                        Some(rgba_common::Event::KeyUp(Key::Up)),
                    Scancode::Down =>
                        Some(rgba_common::Event::KeyUp(Key::Down)),
                    Scancode::Right =>
                        Some(rgba_common::Event::KeyUp(Key::Right)),
                    Scancode::Left =>
                        Some(rgba_common::Event::KeyUp(Key::Left)),
                    _ => None,
                },
            _ => None
        }
    }
}

impl Platform for SDLPlatform {
    fn queue_samples(&mut self, samples: &[i16]) {
        self.audio_device.queue(samples);
    }

    fn read_line(&mut self, prompt: &str) -> Option<String> {
        if let Ok(s) = self.rl.readline(prompt) {
            self.rl.add_history_entry(&s);
            Some(s)
        } else {
            None
        }
    }
}
