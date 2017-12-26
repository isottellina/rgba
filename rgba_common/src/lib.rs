// core.rs --- 
// 
// Filename: core.rs
// Author: Louise <louise>
// Created: Wed Dec  6 14:34:12 2017 (+0100)
// Last-Updated: Tue Dec 26 11:47:24 2017 (+0100)
//           By: Louise <louise>
//

// Enums

#[derive(Debug, Clone, Copy)]
pub enum Console {
    Gameboy,
    GameboyAdvance,
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

// Traits

pub trait Core {
    fn run<T: Platform>(&mut self, &mut T, bool);
    fn is_file(&str) -> bool;
    fn load_bios(&mut self, &str) -> Result<(), &'static str>;
    fn load_rom(&mut self, &str) -> bool;

    fn get_platform_parameters() -> (u32, u32, u32);
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
    fn poll_event(&mut self) -> Option<Event> {
        None
    }
}
