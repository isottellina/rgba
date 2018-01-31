// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Thu Jan  4 00:29:52 2018 (+0100)
// Last-Updated: Wed Jan 31 10:46:54 2018 (+0100)
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

    pub fn trigger(&mut self) { self.steps = 1; }
    
    pub fn handle<T: Platform>(&mut self, gba: &mut GBA, platform: &T) {
        let pc = gba.cpu.pc;
        
        if self.should_break(pc) || self.enough_steps() {
            println!("{}", gba.cpu);
            println!("{:08x}: {}",
                     pc,
                     match gba.cpu.state {
                         CpuState::ARM => disasm_arm(&gba.io, pc),
                         CpuState::Thumb => disasm_thumb(&gba.io, pc),
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

                    Some("x/1") => {
                        match get_argument(&mut cmd) {
                            Some(address) => {
                                print!("{:08x}:", address);

                                for i in 0..16 {
                                    print!(" {:02x}", gba.io.read_u8((address as usize) + i));
                                }

                                println!("");
                            }
                            _ => println!("This command requires an argument"),
                        }
                    }

                    Some("x/2") => {
                        match get_argument(&mut cmd) {
                            Some(address) => {
                                let aligned = (address & 0xFFFFFFFE) as usize;
                                print!("{:08x}:", aligned);

                                for i in 0..8 {
                                    print!(" {:04x}", gba.io.read_u16(aligned + (i << 1)));
                                }

                                println!("");
                            }
                            _ => println!("This command requires an argument"),
                        }
                    }

                    Some("x/4") => {
                        match get_argument(&mut cmd) {
                            Some(address) => {
                                let aligned = (address & 0xFFFFFFFC) as usize;
                                print!("{:08x}:", aligned);

                                for i in 0..4 {
                                    print!(" {:08x}", gba.io.read_u32(aligned + (i << 2)));
                                }

                                println!("");
                            }
                            _ => println!("This command requires an argument"),
                        }
                    }
                    
                    Some("d") | Some("dis") => {
                        let addr = if let Some(u) = get_argument(&mut cmd) {
                            u
                        } else {
                            pc
                        };

                        match gba.cpu.state {
                            CpuState::ARM => {
                                println!("{:08x}: {}", addr, disasm_arm(&gba.io, addr));
                            },
                            CpuState::Thumb => {
                                println!("{:08x}: {}", addr, disasm_thumb(&gba.io, addr));
                            }
                        }
                    }

                    Some("d/a") => {
                        let addr = if let Some(u) = get_argument(&mut cmd) {
                            u
                        } else {
                            pc
                        } & 0xFFFFFFFC;
                        
                        println!("{:08x}: {}", addr, disasm_arm(&gba.io, addr));
                    }

                    Some("d/t") => {
                        let addr = if let Some(u) = get_argument(&mut cmd) {
                            u
                        } else {
                            pc
                        } & 0xFFFFFFFE;
                        
                        println!("{:08x}: {}", addr, disasm_thumb(&gba.io, addr));
                    }
                    
                    Some("h") | Some("help") => {
                        println!("h, help\t\tPrint this help \n\
                                  c, continue\tResume execution \n\
                                  s, step\t\tExecute one instruction \n\
                                  b, break\tSet a breakpoint \n\
                                  rb, rbreak\tRemove a breakpoint \n\
                                  x/1\t\tRead bytes at a specified address \n\
                                  x/2\t\tRead halfwords at a specified address \n\
                                  x/4\t\tRead words at a specified address \n\
                                  d, dis\t\tDisassemble one instruction \n\
                                  d/a\t\tDisassemble one ARM instruction \n\
                                  d/t\t\tDisassemble one Thumb instruction \n\
                                  q, quit\t\tQuit the emulator");
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
