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
    FastMode,
    KeyDown(Key),
    KeyUp(Key)
}

// Traits

pub trait Core {
    fn run<T: Platform>(&mut self, platform: &mut T, use_debug: bool);
    fn is_file(filename: &str) -> bool;
    fn load_bios<T: ToString>(&mut self, filename: Option<T>) -> Result<(), &'static str>;
    fn load_rom(&mut self, filename: &str) -> bool;

    fn get_platform_parameters() -> (u32, u32);
    fn get_console_type() -> Console;
}

pub trait Platform {
    fn new(width: u32, height: u32, scale: u32) -> Self;

    // Video functions
    fn set_pixel(&mut self, x: u32, y: u32, color: u32);
    fn set_scanline(&mut self, y: u32, scanline: &[u32]);
    fn present(&mut self);
    fn set_title(&mut self, _title: String) {
        
    }

    // Sound functions
    fn queue_samples(&mut self, _samples: &[i16]) {

    }
    
    // Input functions
    fn poll_event(&mut self) -> Option<Event> { None }
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
