// debug.rs --- 
// 
// Filename: debug.rs
// Author: Louise <louise>
// Created: Sat Dec  9 23:52:10 2017 (+0100)
// Last-Updated: Mon Dec 25 19:18:03 2017 (+0100)
//           By: Louise <louise>
//
mod disasm;

use std::collections::{BTreeSet, VecDeque};
use readline::{readline, add_history};

use ::Gameboy;
use debug::disasm::disasm;

pub struct Debugger {
    breakpoints: BTreeSet<usize>,
    
    steps: u32,
}

impl Debugger {
    pub fn new(trigger: bool) -> Debugger {
        Debugger {
            breakpoints: BTreeSet::new(),
            
            steps: trigger as u32
        }
    }

    pub fn trigger(&mut self) {
        self.steps = 1;
    }
    
    pub fn handle(&mut self, gb: &mut Gameboy) {
        if self.should_break(gb.cpu.pc()) || self.enough_steps() {
            println!("{}", gb.cpu);
            println!("0x{:04x}: {}", gb.cpu.pc(), disasm(&gb.io, gb.cpu.pc()));
            
            while let Ok(s) = readline("> ") {
                if let Err(_e) = add_history(&s) {
                    error!("Couldn't add to history!");
                }
                
                let mut command: VecDeque<&str> =
                    s.split_whitespace().collect();

                match command.pop_front() {
                    Some("c") | Some("continue") => {
                        break;
                    },

                    Some("q") | Some("quit") => {
                        gb.state = false;
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

                    Some("x") | Some("read") => {
                        let addr = if let Some(u) = get_argument(&mut command) {
                            u as usize
                        } else {
                            gb.cpu.pc()
                        };

                        println!("{:04x}: {:02x}", addr, gb.io.read_u8(addr));
                    },
                    
                    Some("d") | Some("disassemble") => {
                        let addr = if let Some(u) = get_argument(&mut command) {
                            u as usize
                        } else {
                            gb.cpu.pc()
                        };

                        println!("{:04x}: {}", addr, disasm(&gb.io, addr));
                    },

                    Some("reset") => gb.reset(),
                    
                    Some("h") | Some("help") => {
                        println!("c, continue\tContinue emulation\n\
                                  q, quit\t\tQuit the emulator\n\
                                  h, help\t\tPrint this help\n\
                                  s, step\t\tStep executgb.ion\n\
                                  b, break\tSet a breakpoint\n\
                                  rb, rbreak\tRemove a breakpoint\n\
                                  d, disassemble\tDisassemble an instructgb.ion");
                    }

                    other => { println!("This command doesn't exist : {:?}", other); }
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
    
    fn should_break(&mut self, pc: usize) -> bool {
        if self.breakpoints.len() == 0 {
            false
        } else {
            let r = self.breakpoints.contains(&pc);
            if r { println!("Breakpoint triggered"); }
            r
        }
    }
}

fn get_argument(command: &mut VecDeque<&str>) -> Option<u32> {
    match command.pop_front() {
        Some(arg) => {
            if arg.starts_with("0x") {
                if let Ok(u) = u32::from_str_radix(&arg[2..], 16) {
                    Some(u)
                } else {
                    None
                }
            } else {
                if let Ok(u) = u32::from_str_radix(arg, 10) {
                    Some(u)
                } else {
                    None
                }
            }
        },

        None => None,
    }
}
