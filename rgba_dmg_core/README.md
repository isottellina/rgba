<!-- README.md --- 
;; 
;; Filename: README.md
;; Author: Louise <louise>
;; Created: Tue Dec 26 11:53:56 2017 (+0100)
;; Last-Updated: Fri Jan 19 12:45:18 2018 (+0100)
;;           By: Louise <louise>
 -->
 
# DMG Core (Gameboy)

The dmg core is the first core to have been written. It's currently in its 1.1 version.

## Changelog
### 1.1
 - Big optimisation in the APU emulation
 - Optimisations in general
 - Saving way less, because saving is slow
 
### 1.0
 - First release

## Supported features

 - Gameboy CPU
 - DMG video
 - Audio
 - Timer, DMA, Input
 - No MBC, MBC1 and MBC3 (without RTC)
 - Basic debugger (stepping, breakpoints, watchpoints, disassembly)
 
## Planned features

 - RTC support for MBC3
 - Booting without bootrom
 - Optimisations in general (in the OAM code for example)
