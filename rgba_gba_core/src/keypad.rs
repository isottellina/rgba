// keypad.rs --- 
// 
// Filename: keypad.rs
// Author: Louise <louise>
// Created: Wed Jan 31 00:52:09 2018 (+0100)
// Last-Updated: Wed Jan 31 10:46:47 2018 (+0100)
//           By: Louise <louise>
// 

#[derive(Default)]
pub struct Keypad {
    pub a_button: bool,
    pub b_button: bool,
    pub start: bool,
    pub select: bool,
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
    pub r_button: bool,
    pub l_button: bool,
}

impl Keypad {
    #[inline]
    pub fn as_register(&self) -> u16 {
        (!self.a_button as u16) |
        ((!self.b_button as u16) << 1) |
        ((!self.select as u16) << 2) |
        ((!self.start as u16) << 3) |
        ((!self.right as u16) << 4) |
        ((!self.left as u16) << 5) |
        ((!self.up as u16) << 6) |
        ((!self.down as u16) << 7) |
        ((!self.r_button as u16) << 8) |
        ((!self.l_button as u16) << 9)
    }
}
