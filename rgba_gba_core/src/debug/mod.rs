// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Thu Jan  4 00:29:52 2018 (+0100)
// Last-Updated: Tue Jan 16 20:24:25 2018 (+0100)
//           By: Louise <louise>
//
mod disasm;

use ::GBA;
use cpu::CpuState;
use debug::disasm::{disasm_arm, disasm_thumb};
use rgba_common::Platform;

use std::collections::VecDeque;
use std::collections::HashSet;

pub struct Debugger {
    steps: u32,
    breakpoints: HashSet<u32>
}

impl Debugger {
    pub fn new(debug: bool) -> Debugger {
        Debugger {
            steps: if debug { 1 } else { 0 },
            breakpoints: HashSet::new(),
        }
    }

    pub fn handle<T: Platform>(&mut self, gba: &mut GBA, platform: &T) {
        let pc = match gba.cpu.state() {
            CpuState::ARM => gba.cpu.get_register(15) - 8,
            CpuState::Thumb => gba.cpu.get_register(15) - 4,
        };
        
        if self.should_break(pc) || self.enough_steps() {
            println!("{}", gba.cpu);
            println!("{:08x}: {}",
                     pc,
                     match gba.cpu.state() {
                         CpuState::ARM => disasm_arm(pc, gba.io.read_u32(pc as usize)),
                         CpuState::Thumb => disasm_thumb(pc, gba.io.read_u16(pc as usize)),
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

                    Some("b") | Some("break") => {
                        match get_argument(&mut cmd) {
                            Some(address) => {
                                println!("Setting breakpoint at {:08x}", address);
                                self.breakpoints.insert(address);
                            }
                            _ => println!("This command requires an argument"),
                        }
                    }

                    Some("rb") | Some("rbreak") => {
                        match get_argument(&mut cmd) {
                            Some(address) => {
                                if self.breakpoints.remove(&address) {
                                    println!("Breakpoint removed at {:08x}", address);
                                } else {
                                    println!("There was no breakpoint to remove at {:08x}", address);
                                }
                            }
                            _ => println!("This command requires an argument"),
                        }
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
    
    pub fn should_break(&self, pc: u32) -> bool {
        if !self.breakpoints.is_empty() {
            self.breakpoints.contains(&pc)
        } else {
            false
        }
    }
    
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
