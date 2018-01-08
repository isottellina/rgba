// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Thu Jan  4 00:29:52 2018 (+0100)
// Last-Updated: Mon Jan  8 14:13:11 2018 (+0100)
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
                Some("x") | Some("read") => {
                    let addr = if let Some(u) = get_argument(&mut cmd) {
                        u as usize
                    } else {
                        gba.cpu.get_register(15) as usize
                    };
                    
                    println!("{:08x}: {:02x}", addr, gba.io.read_u32(addr));
                }
                
                Some(c) => println!("The command {} doesn't exist.", c),
                None => { },
            }
        }
    }
}

fn get_argument(command: &mut VecDeque<&str>) -> Option<u32> {
    command.pop_front().and_then(
        |arg| {
            if arg.starts_with("0x") {
                u32::from_str_radix(&arg[2..], 16).ok()
            } else if arg.starts_with("0b") {
                u32::from_str_radix(&arg[2..], 2).ok()
            } else if arg.starts_with("0") {
                u32::from_str_radix(arg, 8).ok()
            } else {
                u32::from_str_radix(arg, 10).ok()
            }
        }
    )
}
