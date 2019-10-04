// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Wed Dec  6 14:33:34 2017 (+0100)
// Last-Updated: Fri Oct  4 01:36:08 2019 (+0200)
//           By: Louise <louise>
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

use rgba_common::{
    core::{Frontend, Core},
    Event,
    fnv_hash,
    declare_init_functions,
    declare_coreinfo,
    declare_is_file,
    declare_cbs,
    declare_core_trait
};

use crate::cpu::LR35902;
use crate::io::Interconnect;

use std::fs::File;
use std::io::{Seek, SeekFrom, Read};

pub struct Gameboy {
    cpu: LR35902,
    io: Interconnect,

    state: bool,
    rom_loaded: bool,
    bios_loaded: bool,
}

impl Gameboy {
    fn new() -> Gameboy {
	simplelog::TermLogger::init(simplelog::LevelFilter::Info,
                                    simplelog::Config::default(),
                                    simplelog::TerminalMode::Stderr
	).unwrap();
	
        Gameboy {
            cpu: LR35902::new(),
            io: Interconnect::new(),

            state: false,
	    rom_loaded: false,
	    bios_loaded: false,
        }
    }
    
    fn reset(&mut self) {
        self.cpu.reset();
        self.io.reset();
    }
    
    fn on_frame(&mut self, frontend: &mut Frontend) {
	self.io.push_frame(frontend);
        /* while let Some(e) = platform.poll_event() {
        match e {
        Event::Quit => self.state = false,
        Event::Debug => debugger.trigger(),
        Event::Reset => self.reset(),
        _ => self.io.handle_event(e),
    }
    } */
    }
}

/*  */

impl Core for Gameboy {
    fn run(&mut self, platform: &mut Frontend) {
        self.state = true;
        
        while !self.io.is_frame_done() {
            self.cpu.step(&mut self.io);
            self.io.spend_cycles();
            self.io.render(platform);
        }

	self.on_frame(platform);
	self.io.ack_frame();
    }

    fn load_rom(&mut self, filename: &str) {
	self.rom_loaded = self.io.load_rom(filename);
    }

    fn load_extra(&mut self, loadname: &str, filename: &str) {
	if loadname == "BIOS" {
	    self.bios_loaded = self.io.load_bios(filename).is_ok();
	} else {
	    panic!("No extra file {} for this core", loadname);
	}
    }

    fn finish(&mut self) -> Result<(), String> {
	if !(self.rom_loaded || self.bios_loaded) {
	    Err("Neither BIOS nor a ROM was loaded.".to_string())
	} else if !self.rom_loaded {
	    Err("No ROM was loaded".to_string())
	} else if !self.bios_loaded {
	    Err("No BIOS was loaded".to_string())
	} else {
	    Ok(())
	}
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

declare_init_functions!(Gameboy);
declare_is_file!(is_file);
declare_coreinfo!("rgba_dmg_core", "Ludwigette", "Gameboy", (160, 144));
declare_cbs!(Gameboy);
declare_core_trait!(Gameboy);
