// lib.rs ---
//
// Filename: lib.rs
// Author: Louise <louise>
// Created: Tue Dec 26 11:12:56 2017 (+0100)
// Last-Updated: Sat Jul  6 22:58:36 2019 (+0200)
//           By: Louise <ludwigette>
//
#[macro_use]
extern crate log;

extern crate rgba_common;
extern crate rgba_dmg_core;
extern crate rgba_gba_core;

use rgba_common::{ConsoleType, Core};
use rgba_dmg_core::Gameboy;
use rgba_gba_core::GBA;

#[derive(Debug, Default)]
/// This struct is used to build and run Console
pub struct ConsoleBuilder {
    bios: Option<String>,
    rom: Option<String>,

    console: Option<ConsoleType>,
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

    pub fn set_console(mut self, console: ConsoleType) -> ConsoleBuilder {
        self.console = Some(console);

        self
    }

    pub fn build(mut self) -> Option<Console> {
        if self.console.is_none() {
            if let Some(ref rom_name) = self.rom {
                if Gameboy::is_file(rom_name) {
                    self.console = Some(ConsoleType::Gameboy);
                } else if GBA::is_file(rom_name) {
                    self.console = Some(ConsoleType::GBA);
                } else {
                    error!("Couldn't guess what console this ROM is for.")
                }
            }
        }

        match self.console {
            Some(ConsoleType::Gameboy) => {
                let mut gb = Gameboy::new();
                let _ = gb.load_bios(self.bios);

                if let Some(file_name) = self.rom {
                    gb.load_rom(&file_name);
                };

                Some(Console::Gameboy(gb))
            }

            _ => None,
        }
    }

    pub fn is_determined(&self) -> bool {
        self.console.is_some()
    }
}

pub enum Console {
    Gameboy(Gameboy),
}

impl Core for Console {
    fn run_frame<T: rgba_common::Platform>(&mut self, platform: &mut T) -> &[u32] {
        match self {
            Console::Gameboy(gb) => gb.run_frame(platform),
        }
    }

    fn process_event(&mut self, event: rgba_common::Event) {
        match self {
            Console::Gameboy(gb) => gb.process_event(event),
        }
    }

    fn is_file(filename: &str) -> bool {
        false
    }

    fn load_bios<T: ToString>(&mut self, filename: Option<T>) -> Result<(), &'static str> {
        match self {
            Console::Gameboy(gb) => gb.load_bios(filename),
        }
    }

    fn load_rom(&mut self, filename: &str) -> bool {
        match self {
            Console::Gameboy(gb) => gb.load_rom(filename),
        }
    }

    fn get_platform_parameters(&self) -> (u32, u32) {
        match self {
            Console::Gameboy(gb) => gb.get_platform_parameters(),
        }
    }

    fn get_console_type() -> ConsoleType {
        ConsoleType::None
    }
}
