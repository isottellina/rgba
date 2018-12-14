// lib.rs --- 
// 
// Filename: lib.rs
// Author: Louise <ludwigette>
// Created: Fri Jul 13 11:20:46 2018 (+0200)
// Last-Updated: Sun Aug  5 13:49:01 2018 (+0200)
//           By: Louise <ludwigette>
//
#[macro_use] extern crate log;
extern crate rgba_common;

use rgba_common::{Core, Console, Platform};
use rgba_common::fnv_hash;

use std::io::Read;
use std::fs::File;

pub struct NES {
    state: bool,
}

impl NES {
    pub fn new() -> NES {
        NES {
            state: false,
        }
    }

    pub fn reset() {
        
    }
}

impl Core for NES {
    fn run<T: Platform>(&mut self, _: &mut T, _: bool) {
        
    }

    fn is_file(filename: &str) -> bool {
        match File::open(filename) {
            Ok(mut file) => {
                let mut magic: [u8; 0x4] = [0; 0x4];
                
                if let Err(e) = file.read_exact(&mut magic) {
                    warn!("Couldn't read ROM file : {}", e);
                    
                    false
                } else {
                    fnv_hash(&magic) == 0xb64b_4d29
                }
            },

            Err(e) => {
                warn!("Couldn't open ROM file : {}", e);
                false
            }
        }
    }

    fn load_bios<T: ToString>(&mut self, _: Option<T>) -> Result<(), &'static str> {
        Ok(())
    }

    fn load_rom(&mut self, filename: &str) -> bool {
        let mut content: Vec<u8> = Vec::new();

        match File::open(filename) {
            Ok(mut file) => {
                if let Err(e) = file.read_to_end(&mut content) {
                    warn!("Couldn't read ROM file : {}", e);

                    false
                } else {
                    true
                }
            },

            Err(e) => {
                warn!("Couldn't open ROM file : {}", e);
                false
            }
        }
    }

    fn get_platform_parameters() -> (u32, u32) { (256, 240) }
    fn get_console_type() -> Console { Console::NES }
}
