// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Thu Jan  4 00:29:52 2018 (+0100)
// Last-Updated: Thu Jan  4 20:26:34 2018 (+0100)
//           By: Louise <louise>
// 
use ::GBA;
use rgba_common::Platform;

use std::collections::VecDeque;

pub struct Debugger {
    steps: u32,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            steps: 0,
        }
    }

    pub fn handle<T: Platform>(&mut self, gba: &mut GBA, platform: &T) {
        println!("{}", gba.cpu);
        
        while let Some(s) = platform.read_line("> ") {
            let mut cmd: VecDeque<&str> = s.split_whitespace().collect();

            match cmd.pop_front() {
                Some("q") | Some("quit") => {
                    gba.state = false;
                    break;
                },

                Some("c") | Some("continue") => break,
                

                Some(c) => println!("The command {} doesn't exist.", c),
                None => { },
            }
        }
    }
}
