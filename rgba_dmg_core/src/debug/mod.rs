// debug.rs --- 
// 
// Filename: debug.rs
// Author: Louise <louise>
// Created: Sat Dec  9 23:52:10 2017 (+0100)
// Last-Updated: Mon Jul  1 12:33:37 2019 (+0200)
//           By: Louise <ludwigette>
//
mod disasm;

use std::collections::{BTreeSet, VecDeque};

use crate::cpu::LR35902;
use crate::io::Interconnect;
use crate::debug::disasm::disasm;

use rgba_common::Platform;

pub struct Debugger {
    breakpoints: BTreeSet<usize>,
    
    steps: u32,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            breakpoints: BTreeSet::new(),
            
            steps: 0
        }
    }

    pub fn trigger(&mut self) {
        self.steps = 1;
    }
    
    pub fn handle<T: Platform>(&mut self, cpu: &mut LR35902, io: &mut Interconnect, platform: &mut T) {
        if self.should_break(cpu.pc()) || self.enough_steps() ||
            self.hit_watchpoint(io) {
                println!("{}", cpu);
                println!("Timer track: {:04x}", io.get_internal());
                println!("0x{:04x}: {}", cpu.pc(), disasm(&io, cpu.pc()));
                
                while let Some(s) = platform.read_line("> ") {
                    let mut command: VecDeque<&str> =
                        s.split_whitespace().collect();

                    match command.pop_front() {
                        Some("c") | Some("continue") => {
                            break;
                        },

                        Some("s") | Some("step") => {
                            self.steps = 1;
                            break;
                        }

                        Some("b") | Some("break") => {
                            match get_argument(&mut command) {
                                Some(addr) => {
                                    println!("Setting breakpoint at {:#04x}", addr);
                                    self.breakpoints.insert(addr as usize);
                                },
                                
                                _ => println!("This function requires an argument"),
                            }
                        },

                        Some("rb") | Some("rbreak") => {
                            match get_argument(&mut command) {
                                Some(addr) => {
                                    println!("Removing breakpoint at {:#04x}", addr);
                                    
                                    if !self.breakpoints.remove(&(addr as usize)) {
                                        println!("There was no breakpoint to remove.");
                                    }
                                },
                                
                                _ => println!("This function requires an argument"),
                            }
                        },

                        Some("w") | Some("watch") => {
                            match get_argument(&mut command) {
                                Some(addr) => {
                                    println!("Setting watchpoint at {:#04x}", addr);
                                    io.set_watchpoint(addr as usize);
                                },
                                
                                _ => println!("This function requires an argument"),
                            }
                        },

                        Some("rw") | Some("rwatch") => {
                            match get_argument(&mut command) {
                                Some(addr) => {
                                    println!("Removing watchpoint at {:#04x}", addr);
                                    
                                    if !io.rem_watchpoint(addr as usize) {
                                        println!("There was no watchpoint to remove.");
                                    }
                                },
                                
                                _ => println!("This function requires an argument"),
                            }
                        },

                        Some("x") | Some("read") => {
                            let addr = if let Some(u) = get_argument(&mut command) {
                                u as usize
                            } else {
                                cpu.pc()
                            };

                            println!("{:04x}: {:02x}", addr, io.read_u8(addr));
                        },
                        
                        Some("d") | Some("disassemble") => {
                            let addr = if let Some(u) = get_argument(&mut command) {
                                u as usize
                            } else {
                                cpu.pc()
                            };

                            println!("{:04x}: {}", addr, disasm(&io, addr));
                        },
                        
                        Some("h") | Some("help") => {
                            println!("c, continue\tContinue emulation\n\
                                      q, quit\t\tQuit the emulator\n\
                                      h, help\t\tPrint this help\n\
                                      s, step\t\tStep execution\n\
                                      b, break\tSet a breakpoint\n\
                                      rb, rbreak\tRemove a breakpoint\n\
                                      w, watch\tSet a watchpoint\n\
                                      rw, rwatch\tRemove a watchpoint\n\
                                      d, disassemble\tDisassemble an instruction"
                            );
                        }

                        Some(o) => { println!("This command doesn't exist : {}", o); }
                        None => { println!("You didn't enter a command"); }
                    }
                }
            }
    }

    fn enough_steps(&mut self) -> bool {
        if self.steps > 0 {
            self.steps -= 1;

            self.steps == 0
        } else {
            false
        }
    }
    
    fn should_break(&self, pc: usize) -> bool {
        if self.breakpoints.is_empty() {
            false
        } else {
            let r = self.breakpoints.contains(&pc);
            if r { println!("Breakpoint triggered"); }
            r
        }
    }

    fn hit_watchpoint(&mut self, io: &mut Interconnect) -> bool {
        if let Some((address, value)) = io.watchpoint_hit() {
            println!("Watchpoint hit at {:04x} (value: {:02x})",
                     address, value);

            io.ack_watchpoint();
            true
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
            } else if arg.starts_with('0') {
                u32::from_str_radix(arg, 8).ok()
            } else {
                u32::from_str_radix(arg, 10).ok()
            }
        }
    )
}
