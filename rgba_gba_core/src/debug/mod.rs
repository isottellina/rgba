// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Thu Jan  4 00:29:52 2018 (+0100)
// Last-Updated: Sat Jan 13 12:14:54 2018 (+0100)
//           By: Louise <louise>
//
mod disasm;

use ::GBA;
use cpu::CpuState;
use debug::disasm::{disasm_arm, disasm_thumb};
use rgba_common::Platform;

use std::collections::VecDeque;

pub struct Debugger {
    steps: u32,
}

impl Debugger {
    pub fn new(debug: bool) -> Debugger {
        Debugger {
            steps: if debug { 1 } else { 0 },
        }
    }

    pub fn handle<T: Platform>(&mut self, gba: &mut GBA, platform: &T) {
        if self.should_break() || self.enough_steps() {
            let pc = gba.cpu.get_register(15) - 8;
        
            println!("{}", gba.cpu);
            println!("{:08x}: {}",
                     pc,
                     match gba.cpu.state() {
                         CpuState::ARM => disasm_arm(pc, gba.io.read_u32(pc as usize)),
                         CpuState::Thumb => "u16 reading not implementd".to_string(),
                     }
            );
            
            while let Some(s) = platform.read_line("> ") {
                let mut cmd: VecDeque<&str> = s.split_whitespace().collect();
                
                match cmd.pop_front() {
                    Some("q") | Some("quit") => {
                        gba.state = false;
                        break;
                    },
                    
                    Some("c") | Some("continue") => break,
                    Some("s") | Some("step") => {
                        self.steps = 1;
                        break;
                    }
                    Some("x") | Some("read") => {
                        let addr = if let Some(u) = get_argument(&mut cmd) {
                            u as usize
                        } else {
                            gba.cpu.get_register(15) as usize
                        };
                        
                        println!("{:08x}: {:08x}", addr, gba.io.read_u32(addr));
                    }
                    
                    Some("d") | Some("dis") => {
                        let addr = if let Some(u) = get_argument(&mut cmd) {
                            u
                        } else {
                            gba.cpu.get_register(15) - 8
                        };
                        
                        let instr = gba.io.read_u32(addr as usize);
                        
                        println!("{:08x}: {}", addr, disasm_arm(addr, instr));
                    }
                    
                    Some(c) => println!("The command {} doesn't exist.", c),
                    None => { },
                }
            }
        }
    }
    
    pub fn should_break(&self) -> bool { false }
    pub fn enough_steps(&mut self) -> bool {
        if self.steps > 0 {
            self.steps -= 1;

            self.steps == 0
        } else {
            false
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
