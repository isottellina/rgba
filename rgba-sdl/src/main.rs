// main.rs ---
//
// Filename: main.rs
// Author: Louise <louise>
// Created: Wed Dec  6 12:07:11 2017 (+0100)
// Last-Updated: Sat Jul  6 22:58:48 2019 (+0200)
//           By: Louise <ludwigette>
//
mod sdl;

use clap::builder::PossibleValue;
use clap::{Arg, Command};
use std::thread::sleep;
use std::time::{Duration, Instant};

use sdl::SDLPlatform;

use rgba_builder::ConsoleBuilder;
use rgba_common::{ConsoleType, Core, Event};

fn main() {
    let matches = Command::new("rgba")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Multi-console emulator")
        .author("Louise Z.")
        .arg(
            Arg::new("bios")
                .short('b')
                .long("bios")
                .value_name("BIOS")
                .help("Sets the BIOS/bootrom to use")
                .required(false),
        )
        .arg(Arg::new("ROM").required(true).index(1))
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enables the debugger at start-up"),
        )
        .arg(
            Arg::new("log")
                .short('l')
                .long("log")
                .num_args(1)
                .value_parser([
                    PossibleValue::new("debug"),
                    PossibleValue::new("info"),
                    PossibleValue::new("warn"),
                    PossibleValue::new("error"),
                ])
                .default_value("warn")
                .help("Set the log level"),
        )
        .arg(
            Arg::new("console")
                .short('c')
                .long("console")
                .num_args(1)
                .value_parser([
                    PossibleValue::new("gb"),
                    PossibleValue::new("gba"),
                    PossibleValue::new("nes"),
                    PossibleValue::new("nds"),
                ])
                .required(false),
        )
        .get_matches();

    let rom_name = matches.get_one::<String>("ROM").unwrap();
    let bios_name = matches.get_one::<String>("bios");
    let debug = matches.contains_id("debug");
    let log = matches.get_one::<String>("log").unwrap();

    let log_level = match log.as_str() {
        "debug" => simplelog::LevelFilter::Debug,
        "info" => simplelog::LevelFilter::Info,
        "warn" => simplelog::LevelFilter::Warn,
        "error" => simplelog::LevelFilter::Error,
        _ => unreachable!(),
    };

    simplelog::TermLogger::init(
        log_level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Stderr,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    let console = ConsoleBuilder::default()
        .load_bios(bios_name.map(String::as_str))
        .load_rom(rom_name);

    let mut console = match matches.get_one::<String>("console").map(String::as_str) {
        Some("gb") => console.set_console(ConsoleType::Gameboy),
        Some("gba") => console.set_console(ConsoleType::GBA),
        Some("nes") => console.set_console(ConsoleType::NES),
        Some("nds") => console.set_console(ConsoleType::NDS),
        None => console,
        _ => unreachable!(),
    }
    .build()
    .unwrap();

    if debug {
        console.process_event(Event::Debug);
    }

    let parameters = console.get_platform_parameters();
    let mut platform = SDLPlatform::new(parameters.0, parameters.1, 2);

    'main_loop: loop {
        let frame_start = Instant::now();

        while let Some(event) = platform.poll_event() {
            match event {
                Event::Quit => break 'main_loop,
                _ => console.process_event(event),
            }
        }

        let buffer = console.run_frame(&mut platform);
        platform.set_buffer(buffer);
        platform.present();

        let time_frame_took = Instant::now() - frame_start;
        let time_to_wait = Duration::new(0, 16_600_000).saturating_sub(time_frame_took);

        if time_to_wait > Duration::new(0, 600_000) {
            sleep(time_to_wait);
        }
    }
}
