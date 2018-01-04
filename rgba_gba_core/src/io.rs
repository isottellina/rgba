// io.rs --- 
// 
// Filename: io.rs
// Author: Louise <louise>
// Created: Wed Jan  3 15:30:01 2018 (+0100)
// Last-Updated: Thu Jan  4 20:57:25 2018 (+0100)
//           By: Louise <louise>
// 
use std::fs::File;
use std::io::Read;

pub struct Interconnect {
    bios: Vec<u8>
}

impl Interconnect {
    pub fn new() -> Interconnect {
        Interconnect {
            bios: vec![]
        }
    }
    
    pub fn load_bios(&mut self, filename: &str) -> Result<(), &'static str> {
        match File::open(filename) {
            Ok(mut file) => {    
                debug!("BIOS file openend");
            
                if let Err(e) = file.read_to_end(&mut self.bios) {
                    warn!("Error reading BIOS file : {}", e);
                    Err("Error reading BIOS")
                } else {
                    Ok(())
                }
            }

            Err(e) => {
                warn!("Couldn't load BIOS : {}", e);
                Err("Error opening BIOS file")
            }
        }
    }
}
