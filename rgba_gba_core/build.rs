// build.rs --- 
// 
// Filename: build.rs
// Author: Louise <louise>
// Created: Sat Jan 13 00:51:32 2018 (+0100)
// Last-Updated: Sat Jan 13 17:47:56 2018 (+0100)
//           By: Louise <louise>
// 
extern crate cpython;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let dir = env::var("OUT_DIR").unwrap();

    // Setting up Python environment
    let gil = cpython::Python::acquire_gil();
    let py = gil.python();

    let locals = cpython::PyDict::new(py);
    let _ = locals.set_item(py, "out_dir", dir);

    // Generating ARM code
    let mut file = File::open("src/cpu/arm_gen.py").unwrap();
    let mut content = String::new();

    let _ = file.read_to_string(&mut content);
    
    py.run(&content, None, Some(&locals)).unwrap();
}
