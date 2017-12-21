// main.rs --- 
// 
// Filename: main.rs
// Author: Louise <louise>
// Created: Wed Dec  6 12:07:11 2017 (+0100)
// Last-Updated: Tue Dec 19 23:31:06 2017 (+0100)
//           By: Louise <louise>
//
extern crate minifb;
#[cfg(feature = "sdl")] extern crate sdl2;

extern crate clap;
#[macro_use] extern crate log;
extern crate readline;
extern crate env_logger;

mod common;
mod platform;
mod gb;

use clap::{App, Arg};
use common::{Core, Console, Platform};
#[cfg(not(feature = "sdl"))] use platform::minifb::MiniFBPlatform;
#[cfg(feature = "sdl")] use platform::sdl::SDLPlatform;
use gb::Gameboy;

fn main() {
    env_logger::init();

    let matches = App::new("rgba").version("0.1")
            .about("Multi-console emulator")
            .author("Louise Z.")
            .arg(Arg::with_name("bios")
                 .short("b")
                 .long("bios")
                 .value_name("BIOS")
                 .help("Sets the BIOS/bootrom to use")
                 .required(false))
            .arg(Arg::with_name("ROM")
                 .required(true)
                 .index(1))
            .arg(Arg::with_name("debug")
                 .short("d")
                 .long("debug")
                 .help("Enables the debugger at start-up"))
            .get_matches();

    let rom_name = matches.value_of("ROM").unwrap();
    let bios_name = matches.value_of("bios").unwrap();
    let debug = matches.is_present("debug");
    
    let (console, parameters) =
        if Gameboy::is_file(rom_name) {
            (Console::Gameboy, Gameboy::get_platform_parameters())
        } else {
            (Console::None, (0, 0, 0))
        };

    #[cfg(feature = "sdl")]
    let mut platform =
        SDLPlatform::new(parameters.0, parameters.1, parameters.2);

    #[cfg(not(feature = "sdl"))]
    let mut platform =
        MiniFBPlatform::new(parameters.0, parameters.1, parameters.2);
    
    match console {
        Console::Gameboy => {
            let mut gb = Gameboy::new();

            gb.load_bios(bios_name);
            gb.load_rom(rom_name);
            gb.run(&mut platform, debug);
        },

        Console::None =>
            eprintln!("Couldn't guess what console this ROM is for.")
    }
}
