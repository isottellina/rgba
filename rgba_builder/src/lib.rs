// lib.rs --- 
// 
// Filename: lib.rs
// Author: Louise <louise>
// Created: Tue Dec 26 11:12:56 2017 (+0100)
// Last-Updated: Sat Jul  6 22:58:36 2019 (+0200)
//           By: Louise <ludwigette>
//
#[macro_use] extern crate log;

extern crate rgba_common;
extern crate rgba_dmg_core;
extern crate rgba_nes_core;
extern crate rgba_gba_core;

use rgba_common::{Console, Core, Platform};
use rgba_dmg_core::Gameboy;
use rgba_nes_core::NES;
use rgba_gba_core::GBA;

#[derive(Debug, Default)]
/// This struct is used to build and run Console
pub struct ConsoleBuilder {
    bios: Option<String>,
    rom: Option<String>,

    console: Option<Console>
}

impl ConsoleBuilder {
    pub fn load_bios(mut self, bios: Option<&str>) -> ConsoleBuilder {
        self.bios = bios.map(|s| s.to_owned());

        self
    }

    pub fn load_rom(mut self, rom: &str) -> ConsoleBuilder {
        self.rom = Some(rom.to_string());

        self
    }

    pub fn set_console(mut self, console: Console) -> ConsoleBuilder {
        self.console = Some(console);

        self
    }

    pub fn build(mut self) -> ConsoleBuilder {
        if self.console.is_none() {
            if let Some(ref rom_name) = self.rom {
                if Gameboy::is_file(rom_name) {
                    self.console = Some(Console::Gameboy);
                } else if NES::is_file(rom_name) {
                    self.console = Some(Console::NES)
                } else if GBA::is_file(rom_name) {
                    self.console = Some(Console::GBA);
                } else {
                    error!("Couldn't guess what console this ROM is for.")
                }
            }
        }

        self
    }

    pub fn is_determined(&self) -> bool { self.console.is_some() }
    
    pub fn get_platform_parameters(&self) -> Option<(u32, u32)> {
        if let Some(ref console) = self.console {
            match *console {
                Console::Gameboy => Some(Gameboy::get_platform_parameters()),
                Console::NES => Some(NES::get_platform_parameters()),
                Console::GBA => Some(GBA::get_platform_parameters()),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn run<T: Platform>(&self, platform: &mut T, debug: bool) -> Result<(), String> {
        if let Some(console) = self.console {
            match console {
                Console::Gameboy => {
                    let mut gb = Gameboy::new();

                    gb.load_bios(self.bios.clone())?;

                    if let Some(ref rom_name) = self.rom {
                        gb.load_rom(rom_name);
                    }
                    
                    gb.run(platform, debug);
                    Ok(())
                },

                Console::GBA => {
                    let mut gba = GBA::new();

                    gba.load_bios(self.bios.clone())?;

                    if let Some(ref rom_name) = self.rom {
                        gba.load_rom(rom_name);
                    }

                    gba.run(platform, debug);
                    Ok(())
                }

                Console::NES => {
                    let mut nes = NES::new();

                    if let Some(ref rom_name) = self.rom {
                        nes.load_rom(rom_name);
                    }

                    nes.run(platform, debug);
                    Ok(())
                }
                
                _ => panic!("There isn't a core yet for {:?}", console)
            }
        } else {
            Err("Console wasn't determined".to_string())
        }
    }
}
