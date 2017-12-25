// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Wed Dec  6 23:43:31 2017 (+0100)
// Last-Updated: Mon Dec 25 01:53:11 2017 (+0100)
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
        ram: [u8; 0x6000],
        
        mode: bool,
        ram_enable: bool,

        rom_bank: u8,
        ram_bank: u8,

        save_filename: String,
    },
    MBC3 {
        rom: Vec<u8>,
        ram: [u8; 0x6000],

        ram_enable: bool,
        rom_bank: u8,
        ram_bank: u8,

        save_filename: String,
    }
}

impl Cartridge {
    pub fn new(filename: &str, rom: Vec<u8>) -> Cartridge {
        match rom[0x147] {
            0x00 => {
                Cartridge::RomOnly(rom)
            },

            0x01...0x03 => {
                let save_filename = format!("{}.sav", filename);
                let mut ram: [u8; 0x6000] = [0; 0x6000];

                if let Ok(mut file) = File::open(&save_filename) {
                    if let Err(e) = file.read_exact(&mut ram) {
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
                    save_filename,
                }
            },

            0x0F...0x13 => {
                let save_filename = format!("{}.sav", filename);
                let mut ram: [u8; 0x6000] = [0; 0x6000];

                if let Ok(mut file) = File::open(&save_filename) {
                    if let Err(e) = file.read_exact(&mut ram) {
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
                    save_filename,
                }
            },
            
            mbc => unimplemented!("MBC type {:02x} has not been \
                                   implemented.", mbc),
        }
    }

    pub fn read_rom(&self, address: usize) -> u8 {
        match *self {
            Cartridge::NoCartridge => 0xFF,
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
        }
    }

    pub fn write_rom(&mut self, address: usize, value: u8) {
        match self {
            &mut Cartridge::NoCartridge => { }
            &mut Cartridge::RomOnly(_) => { },
            &mut Cartridge::MBC1 {
                ref mut rom_bank,
                ref mut ram_bank,
                ref mut mode,
                ref mut ram_enable,
                ref ram,
                ref save_filename, ..
            } => {
                match address {
                    0x0000...0x1FFF => {
                        *ram_enable = (value & 0xF) == 0xA;

                        if !*ram_enable {
                            if let Ok(mut file) = File::create(save_filename) {
                                if let Err(e) = file.write(ram) {
                                    warn!("Couldn't save to savefile {}: {}",
                                          save_filename,
                                          e
                                    );
                                }
                            } else {
                                warn!("Couldn't open savefile");
                            }
                        }
                    },
                    0x2000...0x3FFF => {
                        *rom_bank &= 0xe0;
                        *rom_bank |= if value == 0 { 1 } else { value & 0x1f };
                    }
                    0x4000...0x5FFF => {
                        if *mode {
                            *ram_bank = value & 0x3;
                        } else {
                            *rom_bank &= 0x1f;
                            *rom_bank |= (value & 0x3) << 5;
                        }
                    }
                    0x6000...0x7FFF => *mode = value != 0,
                    _ => unreachable!(),
                }
            }
            &mut Cartridge::MBC3 {
                ref mut rom_bank,
                ref mut ram_bank,
                ref mut ram_enable,
                ref ram,
                ref save_filename, ..
            } => {
                match address {
                    0x0000...0x1FFF => {
                        *ram_enable = (value & 0xF) == 0xA;

                        if !*ram_enable {
                            if let Ok(mut file) = File::open(save_filename) {
                                if let Err(e) = file.write(ram) {
                                    warn!("Couldn't save to savefile : {}", e);
                                }
                            } else {
                                warn!("Couldn't open savefile");
                            }
                        }
                    },
                    0x2000...0x3FFF => {
                        *rom_bank = if value == 0 {
                            1
                        } else {
                            value & 0x7f
                        };
                    }
                    0x4000...0x5FFF => {
                        if value < 4 {
                            *ram_bank = value;
                        } else {
                            warn!("RTC isn't implemented");
                        }
                    }
                    0x6000...0x7FFF => warn!("RTC isn't implemented"),
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn read_ram(&self, address: usize) -> u8 {
        match self {
            &Cartridge::NoCartridge => 0xFF,
            &Cartridge::RomOnly(_) => 0xFF,
            &Cartridge::MBC1 { ref ram, ram_bank, .. } |
            &Cartridge::MBC3 { ref ram, ram_bank, .. } =>
                ram[((ram_bank as usize) << 13) + (address & 0x1FFF)]
        }
    }

    pub fn write_ram(&mut self, address: usize, value: u8) {
        match self {
            &mut Cartridge::NoCartridge => { },
            &mut Cartridge::RomOnly(_) => { },
            &mut Cartridge::MBC1 { ref mut ram, ram_bank, .. } |
            &mut Cartridge::MBC3 { ref mut ram, ram_bank, .. } =>
                ram[((ram_bank as usize) << 13) + (address & 0x1FFF)] = value,
        }
    }
}
