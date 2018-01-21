// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Wed Dec  6 23:43:31 2017 (+0100)
// Last-Updated: Sun Jan 21 14:40:54 2018 (+0100)
//           By: Louise <louise>
//
use std::fs::File;
use std::io::{Read, Write};

#[derive(Clone)]
pub enum Cartridge {
    NoCartridge,
    RomOnly(
        Vec<u8>
    ),
    MBC1 {
        rom: Vec<u8>,
        ram: Box<[u8; 0x8000]>,
        
        mode: bool,
        ram_enable: bool,

        rom_bank: u8,
        ram_bank: u8,

        rom_banks: u8,

        save_filename: String,
    },
    MBC3 {
        rom: Vec<u8>,
        ram: Box<[u8; 0x8000]>,

        ram_enable: bool,
        rom_bank: u8,
        ram_bank: u8,

        rom_banks: u8,

        save_filename: String,
    },
    MBC5 {
        rom: Vec<u8>,
        ram: Box<[u8; 0x20_000]>,

        ram_enable: bool,
        rom_bank: usize,
        ram_bank: u8,

        rom_banks: u8,

        save_filename: String,
    },
}

impl Cartridge {
    pub fn new(filename: &str, rom: Vec<u8>) -> Cartridge {
        let rom_banks: u8 = 2 << rom[0x148];
        
        match rom[0x147] {
            0x00 => {
                Cartridge::RomOnly(rom)
            },

            0x01...0x03 => {
                let save_filename = format!("{}.sav", filename);
                let mut ram: Box<[u8; 0x8000]> = Box::new([0; 0x8000]);

                if let Ok(mut file) = File::open(&save_filename) {
                    if let Err(e) = file.read_exact(ram.as_mut()) {
                        warn!("Couldn't read savefile : {}", e);
                    } else {
                        info!("Savefile loaded!");
                    }
                }
                
                Cartridge::MBC1 {
                    rom,
                    ram,
                    ram_enable: false,
                    mode: false,
                    rom_bank: 1,
                    ram_bank: 0,
                    rom_banks,
                    save_filename,
                }
            },

            0x0F...0x13 => {
                let save_filename = format!("{}.sav", filename);
                let mut ram: Box<[u8; 0x8000]> = Box::new([0; 0x8000]);

                if let Ok(mut file) = File::open(&save_filename) {
                    if let Err(e) = file.read_exact(ram.as_mut()) {
                        warn!("Couldn't read savefile : {}", e);
                    } else {
                        info!("Savefile loaded!");
                    }
                }
                
                Cartridge::MBC3 {
                    rom,
                    ram,
                    ram_enable: false,
                    
                    rom_bank: 1,
                    ram_bank: 0,
                    rom_banks,
                    save_filename,
                }
            },

            0x19...0x1E => {
                let save_filename = format!("{}.sav", filename);
                let mut ram: Box<[u8; 0x20_000]> = Box::new([0; 0x20_000]);
                
                if let Ok(mut file) = File::open(&save_filename) {
                    if let Err(e) = file.read_exact(ram.as_mut()) {
                        warn!("Couldn't read savefile : {}", e);
                    } else {
                        info!("Savefile loaded!");
                    }
                }
                
                Cartridge::MBC5 {
                    rom,
                    ram,
                    ram_enable: false,
                    
                    rom_bank: 1,
                    ram_bank: 0,
                    rom_banks,
                    save_filename,
                }
            },
            
            mbc => unimplemented!("MBC type {:02x} has not been \
                                   implemented.", mbc),
        }
    }

    pub fn read_rom(&self, address: usize) -> u8 {
        match *self {
            Cartridge::NoCartridge => {
                warn!("Unmapped read from {:04x} (Cart ROM)", address);
                0xFF
            },
            Cartridge::RomOnly(ref v) => v[address],
            Cartridge::MBC1 { rom: ref v, rom_bank: b, .. } |
            Cartridge::MBC3 { rom: ref v, rom_bank: b, .. } => {
                match address {
                    0x0000...0x3FFF => v[address],
                    0x4000...0x7FFF => 
                        v[((b as usize) << 14) + (address & 0x3FFF)],
                    _ => unreachable!(),
                }
            }
            Cartridge::MBC5 { rom: ref v, rom_bank: b, .. } => {
                match address {
                    0x0000...0x3FFF => v[address],
                    0x4000...0x7FFF => 
                        v[(b << 14) + (address & 0x3FFF)],
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn write_rom(&mut self, address: usize, value: u8) {
        match *self {
            Cartridge::NoCartridge |
            Cartridge::RomOnly(_) =>
                warn!("Unmapped write to {:04x} (Cart ROM, value={:02x})", address, value),
            Cartridge::MBC1 {
                ref mut rom_bank,
                ref mut ram_bank,
                ref mut mode,
                ref mut ram_enable,
                ref rom_banks, ..
            } => {
                match address {
                    0x0000...0x1FFF => {
                        *ram_enable = (value & 0xF) == 0xA;
                    },
                    0x2000...0x3FFF => {
                        *rom_bank &= 0xe0;
                        *rom_bank |= if value == 0 { 1 } else { value & 0x1f };
                        *rom_bank %= rom_banks;
                    }
                    0x4000...0x5FFF => {
                        if *mode {
                            *ram_bank = value & 0x3;
                        } else {
                            *rom_bank &= 0x1f;
                            *rom_bank |= (value & 0x3) << 5;
                            *rom_bank %= rom_banks;
                        }
                    }
                    0x6000...0x7FFF => *mode = value != 0,
                    _ => unreachable!(),
                }
            }
            Cartridge::MBC3 {
                ref mut rom_bank,
                ref mut ram_bank,
                ref mut ram_enable,
                ref rom_banks, ..
            } => {
                match address {
                    0x0000...0x1FFF => {
                        *ram_enable = (value & 0xF) == 0xA;
                    },
                    0x2000...0x3FFF => {
                        *rom_bank = if value == 0 {
                            1
                        } else {
                            value & 0x7f
                        };

                        *rom_bank %= rom_banks;
                    }
                    0x4000...0x5FFF => {
                        if value < 4 {
                            *ram_bank = value;
                        } else {
                            // RTC here
                        }
                    }
                    0x6000...0x7FFF => { },
                    _ => unreachable!(),
                }
            }
            Cartridge::MBC5 {
                ref mut rom_bank,
                ref mut ram_bank,
                ref mut ram_enable,
                ref rom_banks, ..
            } => {
                match address {
                    0x0000...0x1FFF => {
                        *ram_enable = (value & 0xF) == 0xA;
                    },
                    0x2000...0x2FFF => {
                        *rom_bank = (*rom_bank & 0x100) | value as usize;
                        *rom_bank %= *rom_banks as usize;
                    }
                    0x3000...0x3FFF => {
                        *rom_bank = (*rom_bank & 0xFF) | (((value as usize) & 0x01) << 8);
                        *rom_bank %= *rom_banks as usize;
                    }
                    0x4000...0x5FFF => {
                            *ram_bank = value & 0xF;
                    }
                    _ => warn!("Unmapped write to {:04x} (Cart ROM, value={:02x})", address, value),
                }
            }
        }
    }

    pub fn read_ram(&self, address: usize) -> u8 {
        match *self {
            Cartridge::NoCartridge |
            Cartridge::RomOnly(_) => {
                warn!("Unmapped read from {:04x} (Cart RAM)", address);
                0xFF
            },
            Cartridge::MBC1 { ref ram, ram_bank, .. } |
            Cartridge::MBC3 { ref ram, ram_bank, .. } =>
                ram[((ram_bank as usize) << 13) + (address & 0x1FFF)],
            Cartridge::MBC5 { ref ram, ram_bank, .. } =>
                ram[((ram_bank as usize) << 13) + (address & 0x1FFF)],
        }
    }

    pub fn write_ram(&mut self, address: usize, value: u8) {
        match *self {
            Cartridge::NoCartridge |
            Cartridge::RomOnly(_) =>
                warn!("Unmapped write to {:04x} (Cart RAM, value={:02x})", address, value),
            Cartridge::MBC1 { ref mut ram, ram_bank, .. } |
            Cartridge::MBC3 { ref mut ram, ram_bank, .. } =>
                ram[((ram_bank as usize) << 13) + (address & 0x1FFF)] = value,
            Cartridge::MBC5 { ref mut ram, ram_bank, .. } =>
                ram[((ram_bank as usize) << 13) + (address & 0x1FFF)] = value,
        }
    }

    pub fn write_savefile(&self) {
        info!("Writing savefile!");
            
        match *self {
            Cartridge::NoCartridge | Cartridge::RomOnly(_) => { },
            Cartridge::MBC1 { ref ram, ref save_filename, .. } |
            Cartridge::MBC3 { ref ram, ref save_filename, .. } => {
                if let Ok(mut file) = File::create(save_filename) {
                    if let Err(e) = file.write(ram.as_ref()) {
                        warn!("Couldn't save to savefile : {}", e);
                    }
                }
            }
            
            Cartridge::MBC5 { ref ram, ref save_filename, .. } => {
                if let Ok(mut file) = File::create(save_filename) {
                    if let Err(e) = file.write(ram.as_ref()) {
                        warn!("Couldn't save to savefile : {}", e);
                    }
                }
            }
        }
    }
}
