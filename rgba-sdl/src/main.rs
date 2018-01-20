// main.rs --- 
// 
// Filename: main.rs
// Author: Louise <louise>
// Created: Wed Dec  6 12:07:11 2017 (+0100)
// Last-Updated: Sat Jan 20 23:14:52 2018 (+0100)
//           By: Louise <louise>
//
extern crate rgba_common;
extern crate rgba_builder;

extern crate clap;
#[macro_use] extern crate log;
extern crate simplelog;

extern crate readline;
extern crate sdl2;

mod sdl;

use clap::{App, Arg};

use sdl::SDLPlatform;

use rgba_common::Platform;
use rgba_builder::ConsoleBuilder;

fn main() {
    let matches = App::new("rgba").version(env!("CARGO_PKG_VERSION"))
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
        .arg(Arg::with_name("log")
             .short("l")
             .long("log")
             .takes_value(true)
             .possible_value("debug")
             .possible_value("info")
             .possible_value("warn")
             .possible_value("error")
             .default_value("warn")
             .help("Set the log level"))
        .get_matches();

    let rom_name = matches.value_of("ROM").unwrap();
    let bios_name = matches.value_of("bios").unwrap();
    let debug = matches.is_present("debug");
    let log = matches.value_of("log").unwrap();

    let log_level = match log {
        "debug" => simplelog::LogLevelFilter::Debug,
        "info" => simplelog::LogLevelFilter::Info,
        "warn" => simplelog::LogLevelFilter::Warn,
        "error" => simplelog::LogLevelFilter::Error,
        _ => unreachable!(),
    };

    let _ = simplelog::TermLogger::init(log_level, simplelog::Config::default()).unwrap();
    
    let console = ConsoleBuilder::default()
        .load_bios(bios_name)
        .load_rom(rom_name)
        .build();

    if console.is_determined() {
        let parameters = console.get_platform_parameters().unwrap();

        let mut platform = SDLPlatform::new(parameters.0, parameters.1, 2);
        
        let _ = console.run(&mut platform, debug);
    } else {
        panic!("Couldn't build Console");
    }
}
