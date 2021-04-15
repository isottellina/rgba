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
pub enum Console {
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
    KeyDown(Key),
    KeyUp(Key)
}

// Structs

#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);
impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        u32::from_be_bytes([0, color.0, color.1, color.2])
    }
}

// Traits

pub trait Core {
    fn run<T: Platform>(&mut self, &mut T, bool);
    fn is_file(&str) -> bool;
    fn load_bios<T: ToString>(&mut self, Option<T>) -> Result<(), &'static str>;
    fn load_rom(&mut self, &str) -> bool;

    fn get_platform_parameters() -> (u32, u32);
    fn get_console_type() -> Console;
}

pub trait Platform {
    fn new(u32, u32, u32) -> Self;

    // Video functions
    fn set_pixel(&mut self, _: u32, _: u32, _: Color) {

    }
    
    fn present(&mut self) {

    }

    fn set_title(&mut self, _: String) {
        
    }

    // Sound functions
    fn queue_samples(&mut self, _: &[i16]) {

    }
    
    // Input functions
    fn poll_event(&mut self) -> Option<Event> { None }
    fn read_line(&mut self, _: &str) -> Option<String> { None }
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
