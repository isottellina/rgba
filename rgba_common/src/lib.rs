// core.rs --- 
// 
// Filename: core.rs
// Author: Louise <louise>
// Created: Wed Dec  6 14:34:12 2017 (+0100)
// Last-Updated: Sat Jul  6 22:57:07 2019 (+0200)
//           By: Louise <ludwigette>
//

// Enums

#[derive(Debug, Clone, Copy)]
pub enum ConsoleType {
    Gameboy,
    NES,
    GBA,
    NDS,
    None
}

#[derive(Debug, Clone, Copy)]
pub enum Key {
    A,
    B,
    Start,
    Select,

    Up,
    Down,
    Right,
    Left
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Quit,
    Debug,
    Reset,
    FastMode,
    KeyDown(Key),
    KeyUp(Key)
}

// Traits

pub trait Core {
    fn run_frame<T: Platform>(&mut self, platform: &mut T) -> &[u32];
    fn process_event(&mut self, event: Event);
    fn is_file(filename: &str) -> bool;
    fn load_bios<T: ToString>(&mut self, filename: Option<T>) -> Result<(), &'static str>;
    fn load_rom(&mut self, filename: &str) -> bool;

    fn get_platform_parameters(&self) -> (u32, u32);
    fn get_console_type() -> ConsoleType;
}

pub trait Platform {
    // Sound functions
    fn queue_samples(&mut self, _samples: &[i16]) {

    }
    
    // Input functions
    fn read_line(&mut self, _prompt: &str) -> Option<String> { None }
}

// Functions

pub fn fnv_hash(data: &[u8]) -> u32 {
    let mut hash: u32 = 0x811c9dc5;
    
    for byte in data.iter() {
        hash = hash.wrapping_mul(16777619);
        hash ^= (*byte) as u32;
    }

    hash
}
