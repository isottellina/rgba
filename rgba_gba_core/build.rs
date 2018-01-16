// build.rs --- 
// 
// Filename: build.rs
// Author: Louise <louise>
// Created: Sat Jan 13 00:51:32 2018 (+0100)
// Last-Updated: Tue Jan 16 20:04:04 2018 (+0100)
//           By: Louise <louise>
// 
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    let dir = env::var("OUT_DIR").unwrap().to_string();

    let python = "python3";
    
    // Generate ARM code
    let gen_arm = Command::new(python)
        .arg("src/cpu/arm_gen.py")
        .output()
        .expect("Failed to launch python");

    let mut file = File::create(dir.clone() + "/arm_generated.rs").unwrap();
    let _ = file.write(gen_arm.stdout.as_slice());

    // Generate Thumb code
    let gen_thumb = Command::new(python)
        .arg("src/cpu/thumb_gen.py")
        .output()
        .expect("Failed to launch python");

    let mut file = File::create(dir.clone() + "/thumb_generated.rs").unwrap();
    let _ = file.write(gen_thumb.stdout.as_slice());
}
