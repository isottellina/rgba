<!-- README.md --- 
;; 
;; Filename: README.md
;; Author: Louise <louise>
;; Created: Thu Dec 21 20:26:39 2017 (+0100)
;; Last-Updated: Thu Dec 21 20:38:11 2017 (+0100)
;;           By: Louise <louise>
 -->

This is a WIP emulator in Rust. It currently supports the Gameboy, but I will be adding more core,
such as a GBA core, and, I hope, a NDS core.

# Building and running

To run, the emulator needs a bootrom/BIOS of the console it's emulating. The SHA-256 sums of the ones
I'm using are :

|Console|Sum|
|Gameboy (DMG)|cf053eccb4ccafff9e67339d4e78e98dce7d1ed59be819d2a1ba2232c6fce1c7|
|Gameboy Color|b4f2e416a35eef52cba161b159c7c8523a92594facb924b3ede0d722867c50c7|
|GBA|fd2547724b505f487e6dcb29ec2ecff3af35a841a77ab2e85fd87350abd36570|

# Controls

Currently, the controls are (for the GB, on a QWERTY keyboard) :

 - Q for A
 - W for B
 - Space bar for Select
 - Return for Start
 - The direction keys for, well, the direction keys
 
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
