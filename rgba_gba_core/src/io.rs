// io.rs --- 
// 
// Filename: io.rs
// Author: Louise <louise>
// Created: Wed Jan  3 15:30:01 2018 (+0100)
// Last-Updated: Tue Jan 16 18:26:44 2018 (+0100)
//           By: Louise <louise>
//
use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::Read;

pub struct Interconnect {
    bios: Vec<u8>,

    postflg: u8,
    ime: bool,
}

impl Interconnect {
    pub fn new() -> Interconnect {
        Interconnect {
            bios: vec![],

            postflg: 0,
            ime: false,
        }
    }

    pub fn read_u32(&self, address: usize) -> u32 {
        match address & 0x0F000000 {
            0x00000000 if address < 0x4000 =>
                LittleEndian::read_u32(&self.bios[address..]),
            _ => unimplemented!(),
        }
    }

    pub fn read_u16(&self, address: usize) -> u16 {
        match address & 0x0F000000 {
            0x00000000 if address < 0x4000 =>
                LittleEndian::read_u16(&self.bios[address..]),
            _ => unimplemented!(),
        }
    }

    pub fn read_u8(&self, address: usize) -> u8 {
        match address & 0x0F000000 {
            0x00000000 if address < 0x4000 => self.bios[address],
            0x04000000 => self.io_read_u8(address),
            _ => unimplemented!("Reading a byte from {:08x}", address)
        }
    }

    fn io_read_u8(&self, address: usize) -> u8 {
        match address {
            IME => self.ime as u8,
            POSTFLG => self.postflg,
            _ => unimplemented!(),
        }
    }

    pub fn write_u32(&mut self, address: usize, value: u32) {
        match address & 0x0F000000 {
            0x00000000 if address < 0x4000 => warn!("Ignored write to BIOS"),
            _ => unimplemented!(),
        }
    }

    pub fn write_u16(&mut self, address: usize, value: u16) {
        match address & 0x0F000000 {
            0x00000000 if address < 0x4000 => warn!("Ignored write to BIOS"),
            _ => unimplemented!(),
        }
    }
    
    pub fn write_u8(&mut self, address: usize, value: u8) {
        match address & 0x0F000000 {
            0x00000000 if address < 0x4000 => warn!("Ignored write to BIOS"),
            0x04000000 => self.io_write_u8(address, value),
            _ => unimplemented!(),
        }
    }

    fn io_write_u8(&mut self, address: usize, value: u8) {
        match address {
            IME => self.ime = value != 0,
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

const POSTFLG: usize = 0x04000300;
const IME: usize = 0x04000208;
