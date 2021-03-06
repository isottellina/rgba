// main.rs --- 
// 
// Filename: main.rs
// Author: Louise <louise>
// Created: Wed Dec  6 12:07:11 2017 (+0100)
// Last-Updated: Sat Jul  6 22:58:48 2019 (+0200)
//           By: Louise <ludwigette>
//
mod sdl;

use clap::{App, Arg};

use sdl::SDLPlatform;

use rgba_common::{Console, Platform};
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
        .arg(Arg::with_name("console")
             .short("c")
             .long("console")
             .takes_value(true)
             .possible_value("gb")
             .possible_value("gba")
             .possible_value("nes")
             .possible_value("nds")
             .required(false))
        .get_matches();

    let rom_name = matches.value_of("ROM").unwrap();
    let bios_name = matches.value_of("bios");
    let debug = matches.is_present("debug");
    let log = matches.value_of("log").unwrap();

    let log_level = match log {
        "debug" => simplelog::LevelFilter::Debug,
        "info" => simplelog::LevelFilter::Info,
        "warn" => simplelog::LevelFilter::Warn,
        "error" => simplelog::LevelFilter::Error,
        _ => unreachable!(),
    };

    simplelog::TermLogger::init(log_level,
                                simplelog::Config::default(),
                                simplelog::TerminalMode::Stderr
    ).unwrap();
    
    let console = ConsoleBuilder::default()
        .load_bios(bios_name)
        .load_rom(rom_name);

    let console = match matches.value_of("console") {
        Some("gb") => console.set_console(Console::Gameboy),
        Some("gba") => console.set_console(Console::GBA),
        Some("nes") => console.set_console(Console::NES),
        Some("nds") => console.set_console(Console::NDS),
        None => console,
        _ => unreachable!(),
    }.build();

    if console.is_determined() {
        let parameters = console.get_platform_parameters().unwrap();
        let mut platform = SDLPlatform::new(parameters.0, parameters.1, 2);
        
        let _ = console.run(&mut platform, debug);
    } else {
        panic!("Couldn't build Console");
    }
}
