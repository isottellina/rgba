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
extern crate rgba_gba_core;

use rgba_common::{Console, Core};
use rgba_dmg_core::Gameboy;
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

    pub fn build(mut self) -> Option<impl Core> {
        if self.console.is_none() {
            if let Some(ref rom_name) = self.rom {
                if Gameboy::is_file(rom_name) {
                    self.console = Some(Console::Gameboy);
                } else if GBA::is_file(rom_name) {
                    self.console = Some(Console::GBA);
                } else {
                    error!("Couldn't guess what console this ROM is for.")
                }
            }
        }

        match self.console {
            Some(Console::Gameboy) => {
                let mut gb = Gameboy::new();
                let _ = gb.load_bios(self.bios);

                if let Some(file_name) = self.rom {
                    gb.load_rom(&file_name);
                };

                Some(gb)
            },

            _ => None
        }
    }

    pub fn is_determined(&self) -> bool { self.console.is_some() }
}
