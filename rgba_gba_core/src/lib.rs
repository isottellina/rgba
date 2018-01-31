// lib.rs --- 
// 
// Filename: lib.rs
// Author: Louise <louise>
// Created: Wed Jan  3 12:26:37 2018 (+0100)
// Last-Updated: Wed Jan 31 01:01:00 2018 (+0100)
//           By: Louise <louise>
//
#[macro_use] extern crate log;
extern crate byteorder;
extern crate rgba_common;
use rgba_common::{Console, Core, Platform, Event, Key};
use rgba_common::fnv_hash;

mod debug;
mod cpu;
mod io;
mod irq;
mod keypad;
mod gpu;
mod apu;

use cpu::ARM7TDMI;
use io::Interconnect;

use debug::Debugger;

use std::fs::File;
use std::io::{Seek, SeekFrom, Read};
use std::time::{Instant, Duration};

use std::thread;

pub struct GBA {
    cpu: ARM7TDMI,
    io: Interconnect,

    state: bool,
    last_frame: Instant,
}

impl GBA {
    pub fn new() -> GBA {
        GBA {
            cpu: ARM7TDMI::new(),
            io: Interconnect::new(),

            state: true,
            last_frame: Instant::now(),
        }
    }
    
    fn on_frame<T: Platform>(&mut self, platform: &mut T, debugger: &mut Debugger) {
        let elapsed = self.last_frame.elapsed();
                
        if elapsed < Duration::new(0, 16_600_000) {
            let to_wait = Duration::new(0, 16_600_000) - elapsed;

            if to_wait > Duration::new(0, 600_000) {
                thread::sleep(to_wait);
            }
        }
        
        let new_elapsed = self.last_frame.elapsed();
        let elapsed_nanos = new_elapsed.as_secs() * 1_000_000_000 +
            u64::from(new_elapsed.subsec_nanos());
        
        let s = format!(
            "rGBA [{}/60]",
            ((1.0 / (elapsed_nanos as f64)) * 1000000000.0).round() as u32
        );
        
        platform.set_title(s);
        platform.present();
        
        self.last_frame = Instant::now();
        while let Some(event) = platform.poll_event() {
            match event {
                Event::Debug => debugger.trigger(),
                Event::Quit => self.state = false,

                // Key down
                Event::KeyDown(Key::A) => self.io.keypad.a_button = true,
                Event::KeyDown(Key::B) => self.io.keypad.b_button = true,
                Event::KeyDown(Key::Start) => self.io.keypad.start = true,
                Event::KeyDown(Key::Select) => self.io.keypad.select = true,
                
                Event::KeyDown(Key::Left) => self.io.keypad.left = true,
                Event::KeyDown(Key::Right) => self.io.keypad.right = true,
                Event::KeyDown(Key::Up) => self.io.keypad.up = true,
                Event::KeyDown(Key::Down) => self.io.keypad.down = true,
                
                // Key up
                Event::KeyUp(Key::A) => self.io.keypad.a_button = false,
                Event::KeyUp(Key::B) => self.io.keypad.b_button = false,
                Event::KeyUp(Key::Start) => self.io.keypad.start = false,
                Event::KeyUp(Key::Select) => self.io.keypad.select = false,
                
                Event::KeyUp(Key::Left) => self.io.keypad.left = false,
                Event::KeyUp(Key::Right) => self.io.keypad.right = false,
                Event::KeyUp(Key::Up) => self.io.keypad.up = false,
                Event::KeyUp(Key::Down) => self.io.keypad.down = false,
                _ => (),
            }
        }
    }
}

impl Core for GBA {
    fn run<T: Platform>(&mut self, platform: &mut T, debug: bool) {
        let mut debugger = Debugger::new(debug);

        self.cpu.reset(&mut self.io);
        
        while self.state {
            if !self.io.halt() {
                debugger.handle(self, platform);

                self.cpu.next_instruction(&mut self.io);
            } else {
                self.io.delay(1);
            }

            if self.io.is_frame() {
                self.on_frame(platform, &mut debugger);
                self.io.ack_frame();
            }
            
            self.io.spend(&mut self.cpu);
            self.io.render(platform)
        }
    }
    
    fn is_file(filename: &str) -> bool {
        match File::open(filename) {
            Ok(mut file) => {
                let mut logo: [u8; 0x9c] = [0; 0x9c];
                
                if let Err(e) = file.seek(SeekFrom::Start(4)) {
                    warn!("Couldn't seek in ROM file : {}", e);

                    false
                } else {
                    if let Err(e) = file.read_exact(&mut logo) {
                        warn!("Couldn't read ROM file : {}", e);

                        false
                    } else {
                        fnv_hash(&logo) == 0xc412c784
                    }
                }
            },

            Err(e) => {
                warn!("Couldn't open ROM file : {}", e);
                false
            }
        }
    }
    
    fn load_bios(&mut self, filename: &str) -> Result<(), &'static str> {
        self.io.load_bios(filename)
    }
    
    fn load_rom(&mut self, filename: &str) -> bool {
        self.io.load_rom(filename)
    }

    fn get_platform_parameters() -> (u32, u32) {
        (240, 160)
    }
    
    fn get_console_type() -> Console { Console::GBA }
}
