// core.rs --- 
// 
// Filename: core.rs
// Author: Louise <louise>
// Created: Wed Dec  6 14:34:12 2017 (+0100)
// Last-Updated: Fri Oct  4 01:58:19 2019 (+0200)
//           By: Louise <louise>
//
#![feature(weak_into_raw)]
pub mod core;
pub mod frontend;

// Enums
#[repr(C)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Event {
    Quit,
    Debug,
    Reset,
    KeyDown(Key),
    KeyUp(Key)
}

// Common things
pub type Pixel = u32; // Format used is RGB24

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CoreInfo {
    pub name: String,
    pub console: String,
    pub version: String,
    pub author: String,
    pub geometry: (u32, u32)
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
