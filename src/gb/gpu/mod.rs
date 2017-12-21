// gpu.rs --- 
// 
// Filename: gpu.rs
// Author: Louise <louise>
// Created: Thu Dec  7 13:38:58 2017 (+0100)
// Last-Updated: Tue Dec 19 19:51:23 2017 (+0100)
//           By: Louise <louise>
//
use common;

mod render;
mod oam;

pub struct GPU {
    vram: [u8; 0x4000],
    oam: [Sprite; 40],
    
    line_cache: [[Option<u8>; 10]; 144],
    render_line: Option<u8>,
    frame_done: bool,
    
    mode: GpuMode,
    clock: u32,
    ly: u8,
    lyc: u8,
    scy: u8,
    scx: u8,
    wy: u8,
    wx: u8,
    
    // LCDC
    display_enable: bool,
    window_map: bool,
    window_enable: bool,
    tile_data: bool,
    bg_map: bool,
    obj_size: bool,
    obj_enable: bool,
    bg_enable: bool,

    // STAT
    coincidence_irq: bool,
    mode2_irq: bool,
    mode1_irq: bool,
    mode0_irq: bool,

    // Palettes
    bgp: [Color; 4],
    obp0: [Color; 4],
    obp1: [Color; 4],

    // Interrupts
    it_vblank: bool,
    it_lcd: bool,
}

impl GPU {
    pub fn new() -> GPU {
        GPU {
            vram: [0; 0x4000],
            oam: [Sprite::new(); 40],
            line_cache: [[None; 10]; 144],

            mode: GpuMode::ReadingOAM,
            render_line: None,
            frame_done: false,
            
            clock: 0,
            ly: 0,
            lyc: 0,
            scy: 0,
            scx: 0,
            wy: 0,
            wx: 0,

            // LCDC
            display_enable: false,
            window_map: false,
            window_enable: false,
            tile_data: false,
            bg_map: false,
            obj_size: false,
            obj_enable: false,
            bg_enable: false,

            // STAT
            coincidence_irq: false,
            mode2_irq: false,
            mode1_irq: false,
            mode0_irq: false,

            // Palettes
            bgp: [Color::White; 4],
            obp0: [Color::White; 4],
            obp1: [Color::White; 4],

            // Interrupts
            it_vblank: false,
            it_lcd: false,
        }
    }

    pub fn reset(&mut self) {
        self.mode = GpuMode::ReadingOAM;
        self.render_line = None;
        self.frame_done = false;
        self.clock = 0;
        self.ly = 0;
        self.lyc = 0;
        self.scy = 0;
        self.scx = 0;

        self.display_enable = false;
        self.it_vblank = false;
        self.it_lcd = false;
    }
    
    fn increment_line(&mut self) {
        self.ly = (self.ly + 1) % 155;

        if self.coincidence_irq && (self.ly == self.lyc) {
            self.it_lcd = true;
        }
        
        if self.ly < 144 {
            match self.render_line {
                None => self.render_line = Some(self.ly),
                Some(_) =>
                    panic!("Tried to render a line when the previous wasn't"),
            }
        }
    }
    
    pub fn spend_cycles(&mut self, cycles: u32) {
        if !self.display_enable {
            return;
        }
        
        self.clock += cycles;

        match self.mode {
            GpuMode::HBlank => {
                if self.clock >= 456 {
                    self.increment_line();
                    self.clock -= 456;
                    
                    if self.ly == 144 {
                        self.it_vblank = true;

                        if self.mode1_irq {
                            self.it_lcd = true;
                        }
                        
                        self.mode = GpuMode::VBlank;
                    } else {
                        if self.mode2_irq {
                            self.it_lcd = true;
                        }
                        
                        self.mode = GpuMode::ReadingOAM;
                    }
                }
            },

            GpuMode::VBlank => {
                if self.clock >= 456 {
                    self.increment_line();
                    self.clock -= 456;

                    if self.ly == 0 {
                        if self.mode2_irq {
                            self.it_lcd = true;
                        }
                        
                        self.mode = GpuMode::ReadingOAM;
                    }
                }
            },

            GpuMode::ReadingOAM => {
                if self.clock >= 80 {
                    self.mode = GpuMode::ReadingVRAM;
                }
            },

            GpuMode::ReadingVRAM => {
                if self.clock >= 252 {
                    if self.mode0_irq {
                        self.it_lcd = true;
                    }
                    
                    self.mode = GpuMode::HBlank;
                }
            }
        }
    }

    pub fn is_frame_done(&self) -> bool { self.frame_done }
    pub fn ack_frame(&mut self) { self.frame_done = false }
    
    // Register functions
    
    #[inline]
    pub fn ly(&self) -> u8 { self.ly }
    
    #[inline]
    pub fn lyc(&self) -> u8 { self.lyc }
    #[inline]
    pub fn set_lyc(&mut self, lyc: u8) { self.lyc = lyc }

    #[inline]
    pub fn scy(&self) -> u8 { self.scy }
    #[inline]
    pub fn set_scy(&mut self, scy: u8) { self.scy = scy }
    
    #[inline]
    pub fn scx(&self) -> u8 { self.scx }
    #[inline]
    pub fn set_scx(&mut self, scx: u8) { self.scx = scx }

    #[inline]
    pub fn wy(&self) -> u8 { self.wy }
    #[inline]
    pub fn set_wy(&mut self, wy: u8) { self.wy = wy; }

    #[inline]
    pub fn wx(&self) -> u8 { self.wx }
    #[inline]
    pub fn set_wx(&mut self, wx: u8) { self.wx = wx }

    #[inline]
    pub fn bgp(&self) -> u8 { self.bgp.get_register() }
    #[inline]
    pub fn set_bgp(&mut self, bgp: u8) { self.bgp.set_register(bgp) }

    #[inline]
    pub fn obp0(&self) -> u8 { self.obp0.get_register() }
    #[inline]
    pub fn set_obp0(&mut self, obp0: u8) { self.obp0.set_register(obp0) }

    #[inline]
    pub fn obp1(&self) -> u8 { self.obp1.get_register() }
    #[inline]
    pub fn set_obp1(&mut self, obp1: u8) { self.obp1.set_register(obp1) }
    
    pub fn lcdc(&self) -> u8 {
        ((self.display_enable as u8) << 7) |
        ((self.window_map as u8) << 6) |
        ((self.window_enable as u8) << 5) |
        ((self.tile_data as u8) << 4) |
        ((self.bg_map as u8) << 3) |
        ((self.obj_size as u8) << 2) |
        ((self.obj_enable as u8) << 1) |
        (self.bg_enable as u8)
    }

    pub fn set_lcdc(&mut self, lcdc: u8) {
        self.display_enable = (lcdc & 0x80) != 0;
        self.window_map = (lcdc & 0x40) != 0;
        self.window_enable = (lcdc & 0x20) != 0;
        self.tile_data = (lcdc & 0x10) != 0;
        self.bg_map = (lcdc & 0x08) != 0;
        self.obj_size = (lcdc & 0x04) != 0;
        self.obj_enable = (lcdc & 0x02) != 0;
        self.bg_enable = (lcdc & 0x01) != 0;
    }

    pub fn stat(&self) -> u8 {
        ((self.coincidence_irq as u8) << 6) |
        ((self.mode2_irq as u8) << 5) |
        ((self.mode1_irq as u8) << 4) |
        ((self.mode0_irq as u8) << 3) |
        (((self.ly == self.lyc) as u8) << 2) |
        (self.mode as u8)
    }

    pub fn set_stat(&mut self, stat: u8) {
        self.coincidence_irq = (stat & 0x40) != 0;
        self.mode2_irq = (stat & 0x20) != 0;
        self.mode1_irq = (stat & 0x10) != 0;
        self.mode0_irq = (stat & 0x08) != 0;
    }

    pub fn it_vblank(&self) -> bool { self.it_vblank }
    pub fn ack_it_vblank(&mut self) { self.it_vblank = false }
    pub fn set_it_vblank(&mut self, v: bool) { self.it_vblank = v }

    pub fn it_lcd(&self) -> bool { self.it_lcd }
    pub fn ack_it_lcd(&mut self) { self.it_lcd = false }
    pub fn set_it_lcd(&mut self, v: bool) { self.it_lcd = v }
    
    pub fn read_vram_u8(&self, address: usize) -> u8 {
        self.vram[address & 0x1FFF]
    }
    pub fn write_vram_u8(&mut self, address: usize, value: u8) {
        self.vram[address & 0x1FFF] = value
    }

    pub fn read_oam_u8(&self, address: usize) -> u8 {
        self.oam[(address & 0xFF) >> 2].read(address)
    }
    
    pub fn write_oam_u8(&mut self, address: usize, value: u8) {
        self.oam[(address & 0xFF) >> 2].write(address, value);
    }
}

#[derive(Debug, Clone, Copy)]
struct Sprite {
    x: u8,
    y: u8,
    tile: u8,

    priority: bool,
    y_flip: bool,
    x_flip: bool,
    dmg_palette: bool,
    cgb_bank: u8,
    cgb_palette: u8,
}

impl Sprite {
    pub fn new() -> Sprite {
        Sprite {
            x: 0,
            y: 0,
            tile: 0,

            priority: false,
            y_flip: false,
            x_flip: false,
            dmg_palette: false,
            cgb_bank: 0,
            cgb_palette: 0,
        }
    }
    
    pub fn read(&self, address: usize) -> u8 {
        match address % 4 {
            0 => self.y,
            1 => self.x,
            2 => self.tile,
            3 => ((self.priority as u8) << 7)
                | ((self.y_flip as u8) << 6)
                | ((self.x_flip as u8) << 5)
                | ((self.dmg_palette as u8) << 4)
                | (self.cgb_bank << 3)
                | self.cgb_palette,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, address: usize, value: u8) {
        match address % 4 {
            0 => self.y = value,
            1 => self.x = value,
            2 => self.tile = value,
            3 => {
                self.priority = (value & 0x80) != 0;
                self.y_flip = (value & 0x40) != 0;
                self.x_flip = (value & 0x20) != 0;
                self.dmg_palette = (value & 0x10) != 0;
                self.cgb_bank = (value & 0x08) >> 3;
                self.cgb_palette = value & 0x7;
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum GpuMode {
    HBlank = 0,
    VBlank = 1,
    ReadingOAM = 2,
    ReadingVRAM = 3,
}

#[derive(Debug, Clone, Copy)]
enum Color {
    White = 0,
    LightGray = 1,
    DarkGray = 2,
    Black = 3
}

impl Color {
    fn as_real(self) -> common::Color {
        match self {
            Color::White => common::Color(224, 248, 208),
            Color::LightGray => common::Color(136, 192, 112),
            Color::DarkGray => common::Color(52, 104, 86),
            Color::Black => common::Color(8, 24, 32),
        }
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Color {
        match value {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            3 => Color::Black,
            _ => unreachable!(),
        }
    }
}

trait PaletteRegister {
    fn get_register(&self) -> u8;
    fn set_register(&mut self, u8);
}

impl PaletteRegister for [Color; 4] {
    fn get_register(&self) -> u8 {
        ((self[3] as u8) << 6) |
        ((self[2] as u8) << 4) |
        ((self[1] as u8) << 2) |
        (self[0] as u8)
    }

    fn set_register(&mut self, value: u8) {
        self[0] = Color::from(value & 0b00000011);
        self[1] = Color::from((value & 0b00001100) >> 2);
        self[2] = Color::from((value & 0b00110000) >> 4);
        self[3] = Color::from((value & 0b11000000) >> 6);
    }
}
