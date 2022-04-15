// lib.rs --- 
// 
// Filename: lib.rs
// Author: Louise <louise>
// Created: Wed Jan  3 12:26:37 2018 (+0100)
// Last-Updated: Fri Jul 13 12:00:53 2018 (+0200)
//           By: Louise <ludwigette>
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

use crate::cpu::ARM7TDMI;
use crate::io::Interconnect;

use crate::debug::Debugger;

use std::fs::File;
use std::io::{Seek, SeekFrom, Read};

pub struct GBA {
    cpu: ARM7TDMI,
    io: Interconnect,
}

impl GBA {
    pub fn new() -> GBA {
        GBA {
            cpu: ARM7TDMI::new(),
            io: Interconnect::new(),
        }
    }
}

impl Core for GBA {
    fn run_frame<T: Platform>(&mut self, platform: &mut T) -> &[u32] {
        let mut debugger = Debugger::new(false);
        
        while !self.io.is_frame() {
            if !self.io.halt() {
                debugger.handle(self, platform);

                self.cpu.next_instruction(&mut self.io);
            } else {
                self.io.delay(25);
            }
            
            self.io.spend(&mut self.cpu);
            self.io.render();
        }

        self.io.ack_frame();
        self.io.get_framebuffer()
    }

    fn process_event(&mut self, event: Event) {
        match event {
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
    
    fn load_bios<T: ToString>(&mut self, filename: Option<T>) -> Result<(), &'static str> {
        match filename {
            Some(f) => self.io.load_bios(&f.to_string()),
            None => panic!("A BIOS is mandatory to run GBA games."),
        }
    }
    
    fn load_rom(&mut self, filename: &str) -> bool {
        self.io.load_rom(filename)
    }

    fn get_platform_parameters(&self) -> (u32, u32) {
        (240, 160)
    }
    
    fn get_console_type() -> Console { Console::GBA }
}
