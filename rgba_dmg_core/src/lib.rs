// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Wed Dec  6 14:33:34 2017 (+0100)
// Last-Updated: Tue Jul  9 14:17:14 2019 (+0200)
//           By: Louise <ludwigette>
//
#[macro_use] extern crate log;
extern crate rgba_common;

mod cpu;
mod gpu;
mod apu;
mod timer;
mod joypad;
mod io;
mod cart;
mod debug;

use rgba_common::{Core, Platform, Event, ConsoleType};
use rgba_common::fnv_hash;
use crate::cpu::LR35902;
use crate::io::Interconnect;
use crate::debug::Debugger;

use std::fs::File;
use std::io::{Seek, SeekFrom, Read};

pub struct Gameboy {
    cpu: LR35902,
    io: Interconnect,
    debug: Debugger,

    fast_mode: bool,
}

impl Gameboy {
    pub fn new() -> Gameboy {
        Gameboy {
            cpu: LR35902::new(),
            io: Interconnect::new(),
            debug: Debugger::new(),

            fast_mode: false,
        }
    }

    fn reset(&mut self) {
        self.cpu.reset();
        self.io.reset();
    }
}

impl Core for Gameboy {
    fn run_frame<T: Platform>(&mut self, platform: &mut T) -> &[u32] {
        while !self.io.is_frame_done() {
            self.debug.handle(&mut self.cpu, &mut self.io, platform);
            self.cpu.step(&mut self.io);
            self.io.spend_cycles();
            self.io.render(platform);
        }

        self.io.ack_frame();
        self.io.write_savefile();

        self.io.get_framebuffer()
    }

    fn process_event(&mut self, event: Event) {
        match event {
            Event::Debug => {
                self.debug.trigger();
            },
            Event::FastMode => {
                self.fast_mode = !self.fast_mode;
                self.io.set_sound_enabled(!self.fast_mode);
            },
            Event::Reset => self.reset(),
            _ => self.io.handle_event(event),
        }
    }
    
    fn is_file(filename: &str) -> bool {
        match File::open(filename) {
            Ok(mut file) => {
                let mut logo: [u8; 0x30] = [0; 0x30];
                
                if let Err(e) = file.seek(SeekFrom::Start(0x104)) {
                    warn!("Couldn't seek in ROM file : {}", e);

                    false
                } else if let Err(e) = file.read_exact(&mut logo) {
                    warn!("Couldn't read ROM file : {}", e);
                    
                    false
                } else {
                    fnv_hash(&logo) == 0x8fcb_d5b7
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
            None => panic!("A bootrom is mandatory for GB games.")
        }
    }

    fn load_rom(&mut self, filename: &str) -> bool {
        self.io.load_rom(filename)
    }

    fn get_platform_parameters(&self) -> (u32, u32) {
        (160, 144)
    }

    fn get_console_type() -> ConsoleType {
        ConsoleType::Gameboy
    }
}

impl Default for Gameboy {
    fn default() -> Gameboy { Gameboy::new() }
}
