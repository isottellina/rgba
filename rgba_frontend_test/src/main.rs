// main.rs --- 
// 
// Filename: main.rs
// Author: Louise <louise>
// Created: Mon Sep 30 21:58:11 2019 (+0200)
// Last-Updated: Thu Oct  3 19:30:42 2019 (+0200)
//           By: Louise <louise>
//
use std::env::args;
use std::rc::Rc;
use std::cell::RefCell;
use rgba_common::Pixel;
use rgba_common::frontend::Core;

#[derive(Debug)]
struct Frontend {
    pub a: u32
}

extern "C" fn cb_present_frame(_s: &[Pixel], d: *const RefCell<Frontend>) {
    let d: Rc<RefCell<Frontend>> = unsafe { Rc::from_raw(d) };
    let mut f = d.borrow_mut();
    f.a = 67;
}

fn main() {
    let frontend_data = Rc::new(
        RefCell::new(
            Frontend {
                a: 45,
            }
        )
    );
    
    let name = args().nth(1).unwrap();
    let rom = args().nth(2).unwrap();
    println!("Loading {}", name);

    let mut core = Core::new(&name);
    println!("is_file: {}", core.is_file(&rom));

    let coreinfo = core.get_coreinfo();
    println!("{:?}", coreinfo);

    let c = frontend_data.clone();
    core.set_cb_present_frame(cb_present_frame, c);
    core.load_rom(rom);
    core.load_extra("BIOS", "lol");
    println!("{:?}", core.finish());
    core.run();
    println!("{:?}", frontend_data);
}
