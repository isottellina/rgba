// main.rs --- 
// 
// Filename: main.rs
// Author: Louise <louise>
// Created: Wed Dec  6 12:07:11 2017 (+0100)
// Last-Updated: Fri Oct  4 14:16:39 2019 (+0200)
//           By: Louise <louise>
//
#![feature(weak_into_raw)]
mod sdl;

use clap::{App, Arg};
use std::{
    rc::Rc,
    cell::RefCell,
};
use rgba_common::Pixel;
use rgba_common::frontend::Core;
use crate::sdl::{SDLPlatform, present_frame};

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
        .arg(Arg::with_name("core")
             .short("c")
             .long("core")
             .takes_value(true)
             .required(true))
        .get_matches();

    let core_name = matches.value_of("core").unwrap();
    let rom_name = matches.value_of("ROM").unwrap();
    let bios_name = matches.value_of("bios");
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
    
    let mut core = Core::new(core_name);
    let core_info = core.get_coreinfo();

    let mut platform = Rc::new(
	RefCell::new(
	    SDLPlatform::new(core_info.geometry.0, core_info.geometry.1, 2)
	)
    );
    
    if !core.is_file(rom_name) {
	panic!("{} is not a GB ROM.", rom_name);
    }

    core.load_rom(rom_name);
    core.load_extra("BIOS", bios_name.unwrap());
    core.set_cb_present_frame(present_frame, platform.clone());
    core.finish().unwrap();

    'main_loop: loop {
	core.run();
	let mut pl = platform.borrow_mut();
	pl.present();

	while let Some(event) = pl.poll_event() {
	    match event {
		rgba_common::Event::Quit => break 'main_loop,
		_ => { }
	    }
	}
    }
}
