// dummy.rs --- 
// 
// Filename: dummy.rs
// Author: Louise <louise>
// Created: Thu Dec 14 22:18:51 2017 (+0100)
// Last-Updated: Sun Dec 17 16:42:26 2017 (+0100)
//           By: Louise <louise>
//
use common;
use common::Platform;
use common::Color;

pub struct DummyPlatform {

}

impl Platform for DummyPlatform {
    fn new(_width: u32, _height: u32, _scale: u32) -> DummyPlatform {
        DummyPlatform {

        }
    }

    fn set_pixel(&mut self, _x: u32, _y: u32, _color: Color) {

    }

    fn present(&mut self) { }

    fn poll_event(&mut self) -> Option<common::Event> { None }
    fn set_title(&mut self, _: String) { }
}
