// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Thu Jan 18 14:14:22 2018 (+0100)
// Last-Updated: Tue Jan 23 23:06:29 2018 (+0100)
//           By: Louise <louise>
// 
mod memory;
mod io;

pub struct GPU {
    // Memory
    pram: [u8; 0x400],
    vram: [u8; 0x18000],
    oam:  [u8; 0x400],

    // State
    render_line: Option<u16>,
    is_frame: bool,
    vcount: u16,
    clock: u32,
    dots: u32,
    mode: GpuMode,

    dispcnt: u16,

    // DISPSTAT
    irq_vblank_en: bool,
    irq_hblank_en: bool,
    irq_vcount_en: bool,
    vcount_match: u16,
    
    // IRQ
    irq_vblank: bool,
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            pram: [0; 0x400],
            vram: [0; 0x18000],
            oam:  [0; 0x400],

            render_line: None,
            is_frame: false,
            vcount: 0,
            clock: 0,
            dots: 0,
            mode: GpuMode::Visible,

            // DISPSTAT
            irq_vblank_en: false,
            irq_hblank_en: false,
            irq_vcount_en: false,
            vcount_match: 0,
            
            dispcnt: 0,
            // IRQ
            irq_vblank: false,
        }
    }

    #[inline]
    fn increment_lines(&mut self) {
        self.vcount = (self.vcount + 1) % 228;
    }

    #[inline]
    pub fn is_frame(&self) -> bool { self.is_frame }
    pub fn ack_frame(&mut self) { self.is_frame = false; }
    
    pub fn spend_cycles(&mut self, nb_cycles: u32) {
        let total_cycles = self.clock + nb_cycles;

        let dots = total_cycles >> 2;
        let new_clock = total_cycles & 3;
        
        self.dots += dots;
        self.clock = new_clock;

        match self.mode {
            GpuMode::Visible => {
                if self.dots >= 240 {
                    self.mode = GpuMode::HBlank;
                }
            }
            GpuMode::HBlank => {
                while self.dots >= 308 {
                    self.dots -= 308;
                    self.increment_lines();

                    if self.vcount == 160 {
                        self.irq_vblank = true;
                        self.mode = GpuMode::VBlank;
                    } else {
                        self.mode = GpuMode::Visible;
                    }
                }
            }
            GpuMode::VBlank => {
                while self.dots >= 308 {
                    self.dots -= 308;
                    self.increment_lines();

                    if self.vcount == 0 {
                        self.is_frame = true;
                        self.mode = GpuMode::Visible;
                    }
                }
            }
        }
    }

    // IRQ
    pub fn irq_vblank(&self) -> bool { self.irq_vblank }
    pub fn ack_vblank(&mut self) { self.irq_vblank = false; }
}

enum GpuMode {
    Visible,
    HBlank,
    VBlank,
}
