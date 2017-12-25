// dummy.rs --- 
// 
// Filename: dummy.rs
// Author: Louise <louise>
// Created: Thu Dec 14 22:18:51 2017 (+0100)
// Last-Updated: Mon Dec 25 19:23:22 2017 (+0100)
//           By: Louise <louise>
//
use rgba_common;
use rgba_common::Platform;
use rgba_common::Color;

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

    fn poll_event(&mut self) -> Option<rgba_common::Event> { None }
    fn set_title(&mut self, _: String) { }
}
