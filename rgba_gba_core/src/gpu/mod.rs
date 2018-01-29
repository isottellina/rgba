// mod.rs --- 
// 
// Filename: mod.rs
// Author: Louise <louise>
// Created: Thu Jan 18 14:14:22 2018 (+0100)
// Last-Updated: Tue Jan 30 00:00:47 2018 (+0100)
//           By: Louise <louise>
// 
mod memory;
mod io;

use irq::{IrqManager, IRQ_VBLANK, IRQ_HBLANK, IRQ_VCOUNT};

use rgba_common::Platform;

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

    // Backgrounds
    bg: [Background; 4],

    // Windows
    win: [Window; 2],
    winin: u16,
    winout: u16,

    // Blend effect
    bldcnt: u16,
    bldalpha: u16,
    bldy: u16,
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

            bg: Default::default(),
            win: Default::default(),
            winin: 0,
            winout: 0,

            bldcnt: 0,
            bldalpha: 0,
            bldy: 0,
        }
    }

    #[inline]
    fn increment_lines(&mut self, irq: &mut IrqManager) {
        self.vcount = (self.vcount + 1) % 228;

        if self.vcount == self.vcount_match {
            irq.raise_irq(IRQ_VCOUNT);
        }
    }

    #[inline]
    pub fn render<T: Platform>(&mut self, _: &mut T) {
        if let Some(line) = self.render_line {
            self.render_line = None;
        }
    }

    #[inline]
    pub fn is_frame(&self) -> bool { self.is_frame }
    pub fn ack_frame(&mut self) { self.is_frame = false; }

    pub fn spend_cycles(&mut self, nb_cycles: u32, irq: &mut IrqManager) {
        let total_cycles = self.clock + nb_cycles;

        let dots = total_cycles >> 2;
        let new_clock = total_cycles & 3;
        
        self.dots += dots;
        self.clock = new_clock;

        match self.mode {
            GpuMode::Visible => {
                if self.dots >= 240 {
                    if self.irq_hblank_en {
                        irq.raise_irq(IRQ_HBLANK);
                    }
                    
                    self.mode = GpuMode::HBlank;
                }
            }
            GpuMode::HBlank => {
                while self.dots >= 308 {
                    self.dots -= 308;
                    self.render_line = Some(self.vcount);
                    self.increment_lines(irq);

                    if self.vcount == 160 {
                        if self.irq_vblank_en {
                            irq.raise_irq(IRQ_VBLANK);
                        }
                        
                        self.mode = GpuMode::VBlank;
                    } else {
                        self.mode = GpuMode::Visible;
                    }
                }
            }
            GpuMode::VBlank => {
                while self.dots >= 308 {
                    self.dots -= 308;
                    self.increment_lines(irq);

                    if self.vcount == 0 {
                        self.is_frame = true;
                        self.mode = GpuMode::Visible;
                    }
                }
            }
        }
    }
}

enum GpuMode {
    Visible,
    HBlank,
    VBlank,
}

#[derive(Default)]
struct Background {
    pub cnt: u16,
    pub h_off: u16,
    pub v_off: u16,

    pub x_ref: u32,
    pub y_ref: u32,
    pub par_a: u16,
    pub par_b: u16,
    pub par_c: u16,
    pub par_d: u16,
}

#[derive(Default)]
struct Window {
    pub h_off: u16,
    pub v_off: u16,
}
