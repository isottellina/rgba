// io.rs --- 
// 
// Filename: io.rs
// Author: Louise <louise>
// Created: Wed Dec  6 16:56:40 2017 (+0100)
// Last-Updated: Fri Dec 29 16:44:08 2017 (+0100)
//           By: Louise <louise>
// 
use rgba_common::Platform;
use rgba_common::Event;

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

use cart::Cartridge;
use gpu::GPU;
use apu::APU;
use timer::Timer;
use joypad::Joypad;

const JOYP: usize = 0xFF00;

const SB: usize = 0xFF01;
const SC: usize = 0xFF02;

const DIV: usize = 0xFF04;
const TIMA: usize = 0xFF05;
const TMA: usize = 0xFF06;
const TAC: usize = 0xFF07;

const NR10: usize = 0xFF10;
const NR11: usize = 0xFF11;
const NR12: usize = 0xFF12;
const NR13: usize = 0xFF13;
const NR14: usize = 0xFF14;
const NR21: usize = 0xFF16;
const NR22: usize = 0xFF17;
const NR23: usize = 0xFF18;
const NR24: usize = 0xFF19;
const NR30: usize = 0xFF1A;
const NR31: usize = 0xFF1B;
const NR32: usize = 0xFF1C;
const NR33: usize = 0xFF1D;
const NR34: usize = 0xFF1E;
const NR41: usize = 0xFF20;
const NR42: usize = 0xFF21;
const NR43: usize = 0xFF22;
const NR44: usize = 0xFF23;
const NR50: usize = 0xFF24;
const NR51: usize = 0xFF25;
const NR52: usize = 0xFF26;
const NR3_WAVE_START: usize = 0xFF30;
const NR3_WAVE_END: usize = 0xFF3F;

const LCDC: usize = 0xFF40;
const STAT: usize = 0xFF41;
const SCY: usize = 0xFF42;
const SCX: usize = 0xFF43;
const LY: usize = 0xFF44;
const LYC: usize = 0xFF45;
const WY: usize = 0xFF4A;
const WX: usize = 0xFF4B;

const BGP: usize = 0xFF47;
const OBP0: usize = 0xFF48;
const OBP1: usize = 0xFF49;

const BCPI: usize = 0xFF68;
const BCPD: usize = 0xFF69;
const OCPI: usize = 0xFF6A;
const OCPD: usize = 0xFF6B;

const DMA: usize = 0xFF46;
const KEY1: usize = 0xFF4D;
const BIOS: usize = 0xFF50;

const VBK: usize = 0xFF4F;
const SVBK: usize = 0xFF70;

const IF: usize = 0xFF0F;
const IE: usize = 0xFFFF;

pub struct Interconnect {
    bios: Vec<u8>,
    cart: Cartridge,
    wram: [u8; 0x2000],
    hram: [u8; 0x80],

    // Components
    timer: Timer,
    gpu: GPU,
    apu: APU,
    joypad: Joypad,

    // Interrupts
    it_vblank_enable: bool,
    it_lcd_enable: bool,
    it_timer_enable: bool,
    it_serial_enable: bool,
    it_joypad_enable: bool,

    // CGB stuff
    wram_bank: u8,
    
    
    // DMA
    dma_src: usize,
    dma_dest: usize,
    dma_ongoing: bool,

    // Watchpoints
    watchpoints_enabled: bool,
    watchpoints: HashSet<usize>,
    watchpoint_hit: Option<(usize, u8)>,
    
    // Other
    bios_inplace: bool,
    cgb: bool,
}

impl Interconnect {
    pub fn new() -> Interconnect {
        Interconnect {
            bios: Vec::new(),
            cart: Cartridge::NoCartridge,
            wram: [0; 0x2000],
            hram: [0; 0x80],

            gpu: GPU::new(),
            apu: APU::new(),
            timer: Timer::new(),
            joypad: Default::default(),
            
            it_vblank_enable: false,
            it_lcd_enable: false,
            it_timer_enable: false,
            it_serial_enable: false,
            it_joypad_enable: false,

            wram_bank: 1,
            
            dma_src: 0,
            dma_dest: 0,
            dma_ongoing: false,

            watchpoints_enabled: false,
            watchpoints: HashSet::new(),
            watchpoint_hit: None,
            
            bios_inplace: false,
            cgb: false,
        }
    }

    pub fn reset(&mut self) {
        self.bios_inplace = true;
        self.dma_ongoing = false;

        self.timer.reset();
        self.gpu.reset();
    }
    
    pub fn load_bios(&mut self, filename: &str) -> Result<(), &'static str> {
        match File::open(filename) {
            Ok(mut file) => {
                info!("BIOS file opened!");
                
                match file.read_to_end(&mut self.bios) {
                    Ok(n) => {
                        self.bios_inplace = true;

                        match n {
                            0x100 => {
                                info!("BIOS type: DMG");
                                self.cgb = false;
                                
                                Ok(())
                            }
                            
                            0x900 => {
                                info!("BIOS type: CGB");
                                self.cgb = true;
                                
                                Ok(())
                            }
                            
                            _ => {
                                Err("BIOS not recognized")
                            }
                        }
                    }

                    Err(e) => {
                        warn!("Couldn't read BIOS file : {}", e);

                        Err("Error while reading file")
                    }
                }
            }

            Err(e) => {
                warn!("Couldn't open BIOS file : {}", e);
                Err("Error opening file")
            }
        }
    }

    pub fn load_rom(&mut self, filename: &str) -> bool {
        match File::open(filename) {
            Ok(mut file) => {
                info!("ROM opened!");

                let mut rom: Vec<u8> = vec!();
                
                if let Err(e) = file.read_to_end(&mut rom) {
                    warn!("Couldn't read ROM file : {}", e);

                    false
                } else {
                    self.cart = Cartridge::new(filename, rom);
                    info!("ROM loaded!");
                    
                    true
                }
            },

            Err(e) => {
                warn!("Couldn't open ROM file : {}", e);
                false
            }
        }
    }
    
    pub fn read_u8(&self, address: usize) -> u8 {
        match address {
            0x0000...0x00FF if self.bios_inplace =>
                self.bios[address],
            0x0200...0x08FF if self.bios_inplace && self.cgb =>
                self.bios[address],
            0x0000...0x7FFF => self.cart.read_rom(address),
            0x8000...0x9FFF => self.gpu.read_vram_u8(address),
            0xA000...0xBFFF => self.cart.read_ram(address),
            0xC000...0xFDFF => self.wram[address & 0x1FFF],
            0xFE00...0xFE9F => self.gpu.read_oam_u8(address),
            0xFEA0...0xFEFF => 0xFF,
            0xFF80...0xFFFE => self.hram[address & 0x7F],

            // IO
            JOYP => self.joypad.read(),
            SB => 0xFF,
            SC => 0x00,

            DIV => self.timer.div(),
            TIMA => self.timer.tima(),
            TMA => self.timer.tma(),
            TAC => self.timer.tac(),

            LCDC => self.gpu.lcdc(),
            STAT => self.gpu.stat(),
            SCY  => self.gpu.scy(),
            SCX  => self.gpu.scx(),
            LY   => self.gpu.ly(),
            LYC  => self.gpu.lyc(),
            WY   => self.gpu.wy(),
            WX   => self.gpu.wx(),
            BGP  => self.gpu.bgp(),
            OBP0 => self.gpu.obp0(),
            OBP1 => self.gpu.obp1(),

            BCPI => self.gpu.bcpi(),
            BCPD => self.gpu.bcpd(),
            OCPI => self.gpu.ocpi(),
            OCPD => self.gpu.ocpd(),

            VBK  if self.cgb => self.gpu.vbk(),
            SVBK if self.cgb => self.wram_bank,

            NR10 => self.apu.nr10(),
            NR11 => self.apu.nr11(),
            NR12 => self.apu.nr12(),
            NR14 => self.apu.nr14(),
            
            NR21 => self.apu.nr21(),
            NR22 => self.apu.nr22(),
            NR24 => self.apu.nr24(),
            
            NR30 => self.apu.nr30(),
            NR31 => self.apu.nr31(),
            NR32 => self.apu.nr32(),
            NR34 => self.apu.nr34(),
            
            NR41 => self.apu.nr41(),
            NR42 => self.apu.nr42(),
            NR43 => self.apu.nr43(),
            NR44 => self.apu.nr44(),
            
            NR50 => self.apu.nr50(),
            NR51 => self.apu.nr51(),
            NR52 => self.apu.nr52(),
            NR3_WAVE_START...NR3_WAVE_END =>
                self.apu.nr3_wave(address),

            IF => self.it_f(),
            IE => self.it_e(),
            
            _ => { warn!("Reading from {:04x} is not supported",address); 0xFF }
        }
    }

    pub fn read_u16(&self, address: usize) -> u16 {
        (self.read_u8(address) as u16) | ((self.read_u8(address+1) as u16) << 8)
    }
    
    pub fn write_u8(&mut self, address: usize, value: u8) {
        if self.watchpoints_enabled {
            if self.watchpoints.contains(&address) {
                self.watchpoint_hit = Some((address, value));
            }
        }
        
        match address {
            0x0000...0x7FFF => self.cart.write_rom(address, value),
            0x8000...0x9FFF => self.gpu.write_vram_u8(address, value),
            0xA000...0xBFFF => self.cart.write_ram(address, value),
            0xC000...0xFDFF => self.wram[address & 0x1FFF] = value,
            0xFE00...0xFE9F => self.gpu.write_oam_u8(address, value),
            0xFEA0...0xFEFF => { },
            0xFF80...0xFFFE => self.hram[address & 0x7F] = value,

            // IO
            JOYP => self.joypad.write(value),
            SB => {},
            SC => {},

            DIV => self.timer.set_div(),
            TMA => self.timer.set_tma(value),
            TAC => self.timer.set_tac(value),

            LCDC => self.gpu.set_lcdc(value),
            STAT => self.gpu.set_stat(value),
            SCY  => self.gpu.set_scy(value),
            SCX  => self.gpu.set_scx(value),
            LY   => { },
            LYC  => self.gpu.set_lyc(value),
            WY   => self.gpu.set_wy(value),
            WX   => self.gpu.set_wx(value),
            BGP  => self.gpu.set_bgp(value),
            OBP0 => self.gpu.set_obp0(value),
            OBP1 => self.gpu.set_obp1(value),

            BCPI if self.cgb => self.gpu.set_bcpi(value),
            BCPD if self.cgb => self.gpu.set_bcpd(value),
            OCPI if self.cgb => self.gpu.set_ocpi(value),
            OCPD if self.cgb => self.gpu.set_ocpd(value),
            
            VBK  if self.cgb => self.gpu.set_vbk(value),
            SVBK if self.cgb => self.set_svbk(value),
            
            BIOS => self.bios_inplace = false,

            NR10 => self.apu.set_nr10(value),
            NR11 => self.apu.set_nr11(value),
            NR12 => self.apu.set_nr12(value),
            NR13 => self.apu.set_nr13(value),
            NR14 => self.apu.set_nr14(value),
            NR21 => self.apu.set_nr21(value),
            NR22 => self.apu.set_nr22(value),
            NR23 => self.apu.set_nr23(value),
            NR24 => self.apu.set_nr24(value),
            NR30 => self.apu.set_nr30(value),
            NR31 => self.apu.set_nr31(value),
            NR32 => self.apu.set_nr32(value),
            NR33 => self.apu.set_nr33(value),
            NR34 => self.apu.set_nr34(value),
            NR41 => self.apu.set_nr41(value),
            NR42 => self.apu.set_nr42(value),
            NR43 => self.apu.set_nr43(value),
            NR44 => self.apu.set_nr44(value),
            NR50 => self.apu.set_nr50(value),
            NR51 => self.apu.set_nr51(value),
            NR52 => self.apu.set_nr52(value),
            NR3_WAVE_START...NR3_WAVE_END =>
                self.apu.set_nr3_wave(address, value),

            DMA  => self.begin_dma(value),
            
            IF   => self.set_if(value),
            IE   => self.set_ie(value),
            
            _ => warn!("Writing to {:04x} is not supported", address),
        }
    }

    pub fn write_u16(&mut self, address: usize, value: u16) {
        self.write_u8(address, value as u8);
        self.write_u8(address + 1, (value >> 8) as u8);
    }

    fn set_svbk(&mut self, svbk: u8) {
        if svbk == 0 {
            self.wram_bank = 1;
        } else {
            self.wram_bank = svbk & 0x7;
        }
    }
    
    fn it_e(&self) -> u8 {
        ((self.it_joypad_enable as u8) << 4) |
        ((self.it_serial_enable as u8) << 3) |
        ((self.it_timer_enable as u8) << 2) |
        ((self.it_lcd_enable as u8) << 1) |
        (self.it_vblank_enable as u8)
    }
    
    fn set_ie(&mut self, value: u8) {
        self.it_vblank_enable = (value & 0x01) != 0;
        self.it_lcd_enable = (value & 0x02) != 0;
        self.it_timer_enable = (value & 0x04) != 0;
        self.it_serial_enable = (value & 0x08) != 0;
        self.it_joypad_enable = (value & 0x10) != 0;
    }

    fn it_f(&self) -> u8 {
        (self.gpu.it_vblank() as u8) |
        ((self.gpu.it_lcd() as u8) << 1) |
        ((self.timer.it_timer() as u8) << 2)
    }
    
    fn set_if(&mut self, value: u8) {
        self.gpu.set_it_vblank((value & 0x01) != 0);
        self.gpu.set_it_lcd((value & 0x02) != 0);
        self.timer.set_it_timer((value & 0x04) != 0);
    }

    fn begin_dma(&mut self, value: u8) {
        self.dma_ongoing = true;
        self.dma_dest = 0xFE00;
        self.dma_src = (value as usize) << 8;
    }

    fn handle_dma(&mut self) {
        let v = self.read_u8(self.dma_src);
        self.gpu.write_oam_u8(self.dma_dest, v);
        
        self.dma_src += 1;
        self.dma_dest += 1;
        
        if self.dma_dest == 0xFEA0 {
            self.dma_ongoing = false;
            self.gpu.rebuild_cache();
        }
    }
    
    pub fn next_interrupt(&self) -> Option<u16> {
        if self.it_vblank_enable && self.gpu.it_vblank() {
            Some(0x40)
        } else if self.it_lcd_enable && self.gpu.it_lcd() {
            Some(0x48)
        } else if self.it_timer_enable && self.timer.it_timer() {
            Some(0x50)
        } else {
            None
        }
    }

    pub fn next_interrupt_ack(&mut self) {
        if self.it_vblank_enable && self.gpu.it_vblank() {
            self.gpu.ack_it_vblank();
        } else if self.it_lcd_enable && self.gpu.it_lcd() {
            self.gpu.ack_it_lcd();
        } else if self.it_timer_enable && self.timer.it_timer() {
            self.timer.ack_it_timer();
        }
    }
    
    #[inline(always)]
    pub fn is_frame_done(&self) -> bool { self.gpu.is_frame_done() }
    #[inline(always)]
    pub fn ack_frame(&mut self) { self.gpu.ack_frame() }

    // Watchpoints
    #[inline]
    pub fn watchpoint_hit(&self) -> Option<(usize, u8)> { self.watchpoint_hit }
    #[inline]
    pub fn set_watchpoint(&mut self, address: usize) {
        self.watchpoints_enabled = true;
        self.watchpoints.insert(address);
    }
    #[inline]
    pub fn rem_watchpoint(&mut self, address: usize) -> bool {
        let r = self.watchpoints.remove(&address);
        self.watchpoints_enabled = !self.watchpoints.is_empty();

        r
    }
    #[inline]
    pub fn ack_watchpoint(&mut self) { self.watchpoint_hit = None }

    
    pub fn delay(&mut self, m_cycles: u32) {
        let mut cycles = m_cycles << 2;

        self.gpu.spend_cycles(cycles);

        while cycles != 0 {
            if self.dma_ongoing {
                self.handle_dma();
            }
            
            self.timer.handle();

            if self.apu.enabled() {
                self.apu.step();
            }

            cycles -= 1;
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        self.joypad.handle_event(event);
    }
    
    pub fn render<T: Platform>(&mut self, platform: &mut T) {
        if self.cgb {
            self.gpu.render_cgb(platform);
        } else {
            self.gpu.render_dmg(platform);
        }
        
        self.apu.render(platform);
    }
}
