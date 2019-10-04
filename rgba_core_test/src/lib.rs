// lib.rs --- 
// 
// Filename: lib.rs
// Author: Louise <louise>
// Created: Mon Sep 30 21:04:20 2019 (+0200)
// Last-Updated: Thu Oct  3 19:31:18 2019 (+0200)
//           By: Louise <louise>
//
use rgba_common::core::{Core, Frontend};
use rgba_common::{
    declare_init_functions,
    declare_coreinfo,
    declare_is_file,
    declare_cbs,
    declare_core_trait
};

#[derive(Debug)]
struct EmulatorCore {
    pub test1: u32,
    pub test2: Vec<i32>
}

impl EmulatorCore {
    fn new() -> EmulatorCore {
        EmulatorCore {
            test1: 42,
            test2: vec![-5, 6, 72]
        }
    }
}

impl Core for EmulatorCore {
    fn run(&mut self, frontend: &mut Frontend) {
        frontend.present_frame(&[]);
        println!("Running one frame");
    }

    fn load_rom(&mut self, filename: &str) {
	println!("Loading ROM: {:?}", filename);
    }

    fn load_extra(&mut self, loadname: &str, filename: &str) {
	println!("Loading extra: {:?}, {:?}", loadname, filename);
    }

    fn finish(&mut self) -> Result<(), String> {
	Err("Didn't load lol".to_string())
    }
}

impl Drop for EmulatorCore {
    fn drop(&mut self) {
        println!("Dropping emulator!");
    }
}

fn is_file(filename: &str) -> bool {
    println!("{}", filename);

    true
}

declare_init_functions!(EmulatorCore);
declare_coreinfo!("core_test", "Ludwigette", "none", (100, 100));
declare_is_file!(is_file);
declare_cbs!(EmulatorCore);
declare_core_trait!(EmulatorCore);
