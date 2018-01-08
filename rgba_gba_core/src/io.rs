// io.rs --- 
// 
// Filename: io.rs
// Author: Louise <louise>
// Created: Wed Jan  3 15:30:01 2018 (+0100)
// Last-Updated: Mon Jan  8 14:12:29 2018 (+0100)
//           By: Louise <louise>
//
use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::Read;

pub struct Interconnect {
    bios: Vec<u8>
}

impl Interconnect {
    pub fn new() -> Interconnect {
        Interconnect {
            bios: vec![],
            
        }
    }

    pub fn read_u32(&self, address: usize) -> u32 {
        match address & 0x0F000000 {
            0x00000000 if address < 0x4000 =>
                LittleEndian::read_u32(&self.bios[address..]),
            _ => unimplemented!(),
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
