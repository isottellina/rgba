<!-- README.md --- 
;; 
<!-- README.md --- 
;; 
;; Filename: README.md
;; Author: Louise <louise>
;; Created: Thu Dec 21 20:26:39 2017 (+0100)
;; Last-Updated: Thu Dec 21 22:22:48 2017 (+0100)
;;           By: Louise <louise>
 -->

This is a WIP emulator in Rust. It currently supports the Gameboy, but I will be adding more core,
such as a GBA core, and, I hope, a NDS core.

# Building and running

## Building

To build, use `cargo build --release --features <the-frontend>`. The binary will be in `target/release/rgba`.

## Bootrom

To run, the emulator needs a bootrom/BIOS of the console it's emulating. The SHA-256 sums of the ones
I'm using are :

|Console|Sum|
|-------|---|
|Gameboy (DMG)|cf053eccb4ccafff9e67339d4e78e98dce7d1ed59be819d2a1ba2232c6fce1c7|
|Gameboy Color|b4f2e416a35eef52cba161b159c7c8523a92594facb924b3ede0d722867c50c7|
|GBA|fd2547724b505f487e6dcb29ec2ecff3af35a841a77ab2e85fd87350abd36570|

## Controls

Currently, the controls are (for the GB, on a QWERTY keyboard) :

 - Q for A
 - W for B
 - Space bar for Select
 - Return for Start
 - The direction keys for, well, the direction keys
 
# Front-ends

Currently, there are three front-ends. To build with a front-end, you have to add `--feature <the-frontend>`
to the `cargo` command. You can only build the binary with one front-end.

The current front-ends are :

|Name |Library used|Particularities|
|-----|------------|---------------|
|dummy|None|Has absolutely no output and no input. It's selected when you select no other frontend|
|sdl  | SDL2       | The recommanded front-end and only fully-functionning one|
|framebuffer| [minifb](https://github.com/emoon/rust_minifb) | Only supports output, no input. |
 
# Emulation cores

## Gameboy

The gb core is the first core to have been written. It's still being written.

### Supported features

 - Gameboy CPU
 - DMG video
 - Timer, DMA, Input
 - No MBC and MBC1
 - Basic debugger (stepping, watchpoints)
 
### Planned features

 - GBC support
 - Sound
 - MBC3
 - Watchpoints
 
## GBA

The gba core should be the next core to be written. It's not begun yet.
 
# Helpful ressources

## Gameboy

 - http://problemkaputt.de/pandocs.htm
 - http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
 
## GBA/NDS

 - http://problemkaputt.de/gbatek.htm
 
## Rust

 - https://doc.rust-lang.org/book/second-edition/
 - https://doc.rust-lang.org/std/
 
# Licence

This code is licensed under the MIT license.
