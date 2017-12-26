<!-- README.md --- 
;; 
;; Filename: README.md
;; Author: Louise <louise>
;; Created: Tue Dec 26 11:55:16 2017 (+0100)
;; Last-Updated: Tue Dec 26 11:57:16 2017 (+0100)
;;           By: Louise <louise>
 -->

# rGBA-CLI

This is the current binary used to launch the rGBA cores.

## Building and running

### Building

To build, use `cargo build --release --features <the-frontend>` in the rgba-cli folder.
The binary will be in `rgba-cli/target/release/rgba`.

### Controls

Currently, the controls are (for the GB, on a QWERTY keyboard) :

 - Q for A
 - W for B
 - Space bar for Select
 - Return for Start
 - The direction keys for, well, the direction keys
 
## Front-ends

Currently, there are three front-ends. To build with a front-end, you have to add `--feature <the-frontend>`
to the `cargo` command. You can only build the binary with one front-end.

The current front-ends are :

|Name |Library used|Particularities|
|-----|------------|---------------|
|dummy|None|Has absolutely no output and no input. It's selected when you select no other frontend|
|sdl  | SDL2       | The recommanded front-end and only fully-functionning one|
|framebuffer| [minifb](https://github.com/emoon/rust_minifb) | Only supports output, no input. |
