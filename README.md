<!-- README.md --- 
;; 
;; Filename: README.md
;; Author: Louise <louise>
;; Created: Thu Dec 21 20:26:39 2017 (+0100)
;; Last-Updated: Wed Dec 27 00:49:56 2017 (+0100)
;;           By: Louise <louise>
 -->

# rGBA

This is a WIP emulator in Rust. It currently supports the Gameboy, but I will be adding more cores,
such as a GBA core, and, I hope, a NDS core.

## Current state

A new version of this emulator, from scratch, is being worked on. The main goals are :

 - Cleaner architecture, with an event-driven clock/scheduler, which will allow us to improve performance, consider a JIT execution core, and emulating several "main components" (namely CPUs here)
 - Cleaner code, using Rust macros to eliminate scheduler and bus access boilerplate while keeping performance, and to following a "turtles all the way down" philosophy to ease (and actually do!) unit testing
 - GDB protocol implementation, to simplify the integration with other tools (such as radare2)
 - VRAM debugger
 - Actual ARM emulation with clean code, not the Python mess you can see here (sorry for that), to keep myself sane
 
Therefore, this repo is not going to receive updates. I'll see you when it's done!

## Building and running

Currently, the binary used to launch rGBA cores is the rGBA-sdl crate.

### Bootrom

To run, the emulator needs a bootrom/BIOS of the console it's emulating. The SHA-256 sums of the ones
I'm using are :

|Console|Sum|
|-------|---|
|Gameboy (DMG)|cf053eccb4ccafff9e67339d4e78e98dce7d1ed59be819d2a1ba2232c6fce1c7|
|Gameboy Color|b4f2e416a35eef52cba161b159c7c8523a92594facb924b3ede0d722867c50c7|
|GBA|fd2547724b505f487e6dcb29ec2ecff3af35a841a77ab2e85fd87350abd36570|
 
## Helpful ressources

### Gameboy

 - http://problemkaputt.de/pandocs.htm
 - http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
 
### GBA/NDS

 - http://problemkaputt.de/gbatek.htm
 
### Rust

 - https://doc.rust-lang.org/book/second-edition/
 - https://doc.rust-lang.org/std/
 - https://crates.io
 
## Licence

This code is licensed under the MIT license.
