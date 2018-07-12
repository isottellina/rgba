// joypad.rs --- 
// 
// Filename: joypad.rs
// Author: Louise <louise>
// Created: Thu Dec 14 23:45:40 2017 (+0100)
// Last-Updated: Thu Jul 12 17:54:10 2018 (+0200)
//           By: Louise <ludwigette>
// 
use rgba_common::{Event, Key};

#[derive(Debug, Default)]
pub struct Joypad {
    mode: bool,

    // Button Keys
    start: bool,
    select: bool,
    a: bool,
    b: bool,

    // Direction Keys
    down: bool,
    up: bool,
    left: bool,
    right: bool,
}

impl Joypad {
    pub fn read(&self) -> u8 {
        if self.mode {
            0xDF &
                !((self.start as u8) << 3) &
                !((self.select as u8) << 2) &
                !((self.b as u8) << 1) &
                !(self.a as u8)
        } else {
            0xEF &
                !((self.down as u8) << 3) &
                !((self.up as u8) << 2) &
                !((self.left as u8) << 1) &
                !(self.right as u8)
        }
    }

    pub fn write(&mut self, value: u8) {
        self.mode = (value & 0x20) == 0;
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::KeyDown(key) => match key {
                Key::Start => self.start = true,
                Key::Select => self.select = true,
                Key::A => self.a = true,
                Key::B => self.b = true,

                Key::Up => self.up = true,
                Key::Down => self.down = true,
                Key::Left => self.left = true,
                Key::Right => self.right = true
            },
            Event::KeyUp(key) => match key {
                Key::Start => self.start = false,
                Key::Select => self.select = false,
                Key::A => self.a = false,
                Key::B => self.b = false,

                Key::Up => self.up = false,
                Key::Down => self.down = false,
                Key::Left => self.left = false,
                Key::Right => self.right = false
            },
            _ => { }
        }
    }
}
