// main.rs --- 
// 
// Filename: main.rs
// Author: Louise <louise>
// Created: Wed Dec  6 12:07:11 2017 (+0100)
// Last-Updated: Thu Jan  4 13:05:12 2018 (+0100)
//           By: Louise <louise>
//
extern crate rgba_common;
extern crate rgba_builder;

extern crate clap;
#[macro_use] extern crate log;
extern crate env_logger;

extern crate readline;
extern crate sdl2;

mod sdl;

use clap::{App, Arg};

use sdl::SDLPlatform;

use rgba_common::Platform;
use rgba_builder::ConsoleBuilder;

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
    
    let console = ConsoleBuilder::default()
        .load_bios(bios_name)
        .load_rom(rom_name)
        .build();

    if console.is_determined() {
        let parameters = console.get_platform_parameters().unwrap();

        let mut platform = SDLPlatform::new(parameters.0, parameters.1, 2);
        
        let _ = console.run(&mut platform, debug).unwrap();
    } else {
        panic!("Couldn't build Console");
    }
}
