// lib.rs --- 
// 
// Filename: lib.rs
// Author: Louise <louise>
// Created: Wed Jan  3 12:26:37 2018 (+0100)
// Last-Updated: Sat Jan 13 12:12:56 2018 (+0100)
//           By: Louise <louise>
//
#[macro_use] extern crate log;
extern crate byteorder;
extern crate rgba_common;
use rgba_common::{Console, Core, Platform};
use rgba_common::fnv_hash;

mod debug;
mod cpu;
mod io;

use cpu::ARM7TDMI;
use io::Interconnect;

use debug::Debugger;

use std::fs::File;
use std::io::{Seek, SeekFrom, Read};

pub struct GBA {
    cpu: ARM7TDMI,
    io: Interconnect,

    state: bool,
}

impl GBA {
    pub fn new() -> GBA {
        GBA {
            cpu: ARM7TDMI::new(),
            io: Interconnect::new(),

            state: true,
        }
    }
}

impl Core for GBA {
    fn run<T: Platform>(&mut self, platform: &mut T, debug: bool) {
        let mut debugger = Debugger::new(debug);

        self.cpu.reset(&self.io);
        
        while self.state {
            debugger.handle(self, platform);
            
            self.cpu.next_instruction(&mut self.io);
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
        false
    }

    fn get_platform_parameters() -> (u32, u32) {
        (240, 160)
    }
    
    fn get_console_type() -> Console { Console::GBA }
}
