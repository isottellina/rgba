// io.rs --- 
// 
// Filename: io.rs
// Author: Louise <louise>
// Created: Wed Jan  3 15:30:01 2018 (+0100)
// Last-Updated: Thu Nov  5 21:07:37 2020 (+0100)
//           By: Louise <louise>
//
mod dma;
mod timer;

use crate::cpu::ARM7TDMI;
use crate::gpu::GPU;
use crate::apu::APU;
use crate::keypad::Keypad;
use crate::irq::IrqManager;
use crate::io::dma::DmaChannel;
use crate::io::timer::Timer;

// Import DMA I/O ports
use crate::io::dma::{DMA0SAD_L, DMA0SAD_H, DMA0DAD_L, DMA0DAD_H, DMA0CNT_L, DMA0CNT_H};
use crate::io::dma::{DMA1SAD_L, DMA1SAD_H, DMA1DAD_L, DMA1DAD_H, DMA1CNT_L, DMA1CNT_H};
use crate::io::dma::{DMA2SAD_L, DMA2SAD_H, DMA2DAD_L, DMA2DAD_H, DMA2CNT_L, DMA2CNT_H};
use crate::io::dma::{DMA3SAD_L, DMA3SAD_H, DMA3DAD_L, DMA3DAD_H, DMA3CNT_L, DMA3CNT_H};

// Import Timer I/O ports
use crate::io::timer::{TM0CNT_L, TM1CNT_L, TM2CNT_L, TM3CNT_L,
                       TM0CNT_H, TM1CNT_H, TM2CNT_H, TM3CNT_H};

use byteorder::{ByteOrder, LittleEndian};
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::cell::RefCell;

#[macro_use] mod macros;

pub struct Interconnect {
    bios: Vec<u8>,
    rom:  Vec<u8>,
    iram: [u8; 0x8000],
    eram: [u8; 0x40000],
    io: [u16; 0x200],

    gpu: GPU,
    apu: APU,
    pub keypad: Keypad,

    cycles_to_spend: u32,
    rom_len: usize,
    waitstates: [[[u32; 2]; 3]; 16],

    postflg: u8,
    irq: IrqManager,
    dma: [DmaChannel; 4],
    timer: [Rc<RefCell<Timer>>; 4],
}

impl Interconnect {
    pub fn new() -> Interconnect {
        let timer1: Rc<RefCell<Timer>> = Default::default();
        let timer2: Rc<RefCell<Timer>> = Default::default();
        let timer3: Rc<RefCell<Timer>> = Default::default();
        let timer4: Rc<RefCell<Timer>> = Default::default();

        timer1.borrow_mut().set_id(1 << 3);
        timer2.borrow_mut().set_id(1 << 4);
        timer3.borrow_mut().set_id(1 << 5);
        timer4.borrow_mut().set_id(1 << 6);
        
        timer1.borrow_mut().set_next(timer2.clone());
        timer2.borrow_mut().set_next(timer3.clone());
        timer3.borrow_mut().set_next(timer4.clone());
            
        Interconnect {
            bios: vec![],
            rom:  vec![],
            iram: [0; 0x8000],
            eram: [0; 0x40000],
            io:   [0; 0x200],
            gpu: GPU::new(),
            apu: APU::new(),
            keypad: Keypad::default(),

            cycles_to_spend: 0,
            rom_len: 0,
            waitstates: [
                [[1, 1], [1, 1], [1, 1]], // BIOS
                [[1, 1], [1, 1], [1, 1]],
                [[3, 3], [3, 3], [6, 6]], // ERAM
                [[1, 1], [1, 1], [1, 1]], // IRAM
                [[1, 1], [1, 1], [1, 1]], // IO
                [[1, 1], [1, 1], [2, 2]], // Palette RAM
                [[1, 1], [1, 1], [2, 2]], // VRAM
                [[1, 1], [1, 1], [1, 1]], // OAM
                [[5, 5], [5, 5], [8, 8]], // GamePak WaitState 0
                [[5, 5], [5, 5], [8, 8]],
                [[5, 5], [5, 5], [8, 8]], // GamePak WaitState 1
                [[5, 5], [5, 5], [8, 8]],
                [[5, 5], [5, 5], [8, 8]], // GamePak WaitState 2
                [[5, 5], [5, 5], [8, 8]],
                [[1, 1], [1, 1], [1, 1]], // GamePak SRAM
                [[1, 1], [1, 1], [1, 1]],
            ],

            postflg: 0,
            irq: IrqManager::new(),
            dma: [DmaChannel::new(0), DmaChannel::new(1), DmaChannel::new(2), DmaChannel::new(3)],
            timer: [timer1, timer2, timer3, timer4],
        }
    }

    pub fn declare_access(&mut self, address: usize, width: usize) {
        self.cycles_to_spend += self.waitstates[(address >> 24) & 0xF][width][0];
    }

    pub fn delay(&mut self, cycles: u32) {
        self.cycles_to_spend += cycles;
    }

    pub fn spend(&mut self, cpu: &mut ARM7TDMI) {
        self.gpu.spend_cycles(self.cycles_to_spend, &mut self.irq);

        // Handle DMA (immediately)
        if self.dma[0].start_timing == 0 && self.dma[0].enable {
            handle_dma!(self, self.dma[0]);
        } else if self.dma[1].start_timing == 0 && self.dma[1].enable {
            handle_dma!(self, self.dma[1]);
        } else if self.dma[2].start_timing == 0 && self.dma[2].enable {
            handle_dma!(self, self.dma[2]);
        } else if self.dma[3].start_timing == 0 && self.dma[3].enable {
            handle_dma!(self, self.dma[3]);
        }

        for timer_ref in &self.timer {
            timer_ref.borrow_mut().spend_cycles(self.cycles_to_spend, &mut self.irq);
        }
        
        self.irq.handle(cpu);
        self.cycles_to_spend = 0;
    }

    pub fn render(&mut self) {
        self.gpu.render();
    }

    pub fn get_framebuffer(&self) -> &[u32] {
        &self.gpu.framebuffer
    }
    
    pub fn read_u32(&self, address: usize) -> u32 {
        match address & 0x0F000000 {
            0x00000000 if address < 0x4000 =>
                LittleEndian::read_u32(&self.bios[address..]),
            0x02000000 =>
                LittleEndian::read_u32(&self.eram[(address & 0x3ffff)..]),
            0x03000000 =>
                LittleEndian::read_u32(&self.iram[(address & 0x7fff)..]),
            0x04000000 => self.io_read_u32(address),
            0x05000000 => self.gpu.pram_read_u32(address),
            0x06000000 => self.gpu.vram_read_u32(address),
            0x07000000 => self.gpu.oam_read_u32(address),
            0x08000000 |
            0x09000000 |
            0x0A000000 |
            0x0B000000 |
            0x0C000000 |
            0x0D000000 => if (address & 0x01FFFFFF) < self.rom_len {
                LittleEndian::read_u32(&self.rom[(address & 0x01FFFFFF)..])
            } else {
                unused_pattern!(address, 32) as u32
            },
            _ => { warn!("Unmapped read_u32 from {:08x}", address); 0 },
        }
    }

    pub fn read_u16(&self, address: usize) -> u16 {
        match address & 0x0F000000 {
            0x00000000 if address < 0x4000 =>
                LittleEndian::read_u16(&self.bios[address..]),
            0x02000000 =>
                LittleEndian::read_u16(&self.eram[(address & 0x3ffff)..]),
            0x03000000 =>
                LittleEndian::read_u16(&self.iram[(address & 0x7fff)..]),
            0x04000000 => self.io_read_u16(address),
            0x05000000 => self.gpu.pram_read_u16(address),
            0x06000000 => self.gpu.vram_read_u16(address),
            0x07000000 => self.gpu.oam_read_u16(address),
            0x08000000 |
            0x09000000 |
            0x0A000000 |
            0x0B000000 |
            0x0C000000 |
            0x0D000000 => if (address & 0x01FFFFFF) < self.rom_len {
                LittleEndian::read_u16(&self.rom[(address & 0x01FFFFFF)..])
            } else {
                unused_pattern!(address, 16) as u16
            },
            _ => { warn!("Unmapped read_u16 from {:08x}", address); 0 },
        }
    }

    pub fn read_u8(&self, address: usize) -> u8 {
        match address & 0x0F000000 {
            0x00000000 if address < 0x4000 => self.bios[address],
            0x02000000 => self.eram[address & 0x3ffff],
            0x03000000 => self.iram[address & 0x7fff],
            0x04000000 => self.io_read_u8(address),
            0x08000000 |
            0x09000000 |
            0x0A000000 |
            0x0B000000 |
            0x0C000000 |
            0x0D000000 => if (address & 0x01FFFFFF) < self.rom_len {
                self.rom[address & 0x01FFFFFF]
            } else {
                unused_pattern!(address, 8) as u8
            }
            _ => { warn!("Unmapped read_u8 from {:08x}", address); 0 },
        }
    }

    fn io_read_u32(&self, address: usize) -> u32 {
        match address {
            _ => {
                (self.io_read_u16(address & 0xFFFFFFFC) as u32)
                    | ((self.io_read_u16((address & 0xFFFFFFFE) | 2) as u32) << 16)
            }
        }
    }
    
    fn io_read_u16(&self, address: usize) -> u16 {
        match address {
            KEYINPUT => self.keypad.as_register(),
            IE => self.irq.i_e,
            IF => self.irq.i_f,
            IME => self.irq.ime as u16,
            0x04000000..=0x04000056 => self.gpu.io_read_u16(address),
            0x04000060..=0x040000A8 => self.apu.io_read_u16(address),

            TM0CNT_L => self.timer[0].borrow().read_cnt_l(),
            TM0CNT_H => self.timer[0].borrow().read_cnt_h(),
            TM1CNT_L => self.timer[1].borrow().read_cnt_l(),
            TM1CNT_H => self.timer[1].borrow().read_cnt_h(),
            TM2CNT_L => self.timer[2].borrow().read_cnt_l(),
            TM2CNT_H => self.timer[2].borrow().read_cnt_h(),
            TM3CNT_L => self.timer[3].borrow().read_cnt_l(),
            TM3CNT_H => self.timer[3].borrow().read_cnt_h(),

            DMA0SAD_L => (self.dma[0].source_addr & 0xffff) as u16,
            DMA0SAD_H => (self.dma[0].source_addr >> 16) as u16,
            DMA0DAD_L => (self.dma[0].dest_addr & 0xffff) as u16,
            DMA0DAD_H => (self.dma[0].dest_addr >> 16) as u16,
            DMA0CNT_L => self.dma[0].word_count,
            DMA0CNT_H => self.dma[0].read_cnt_h(),
            
            DMA1SAD_L => (self.dma[1].source_addr & 0xffff) as u16,
            DMA1SAD_H => (self.dma[1].source_addr >> 16) as u16,
            DMA1DAD_L => (self.dma[1].dest_addr & 0xffff) as u16,
            DMA1DAD_H => (self.dma[1].dest_addr >> 16) as u16,
            DMA1CNT_L => self.dma[1].word_count,
            DMA1CNT_H => self.dma[1].read_cnt_h(),
            
            DMA2SAD_L => (self.dma[2].source_addr & 0xffff) as u16,
            DMA2SAD_H => (self.dma[2].source_addr >> 16) as u16,
            DMA2DAD_L => (self.dma[2].dest_addr & 0xffff) as u16,
            DMA2DAD_H => (self.dma[2].dest_addr >> 16) as u16,
            DMA2CNT_L => self.dma[2].word_count,
            DMA2CNT_H => self.dma[2].read_cnt_h(),
            
            DMA3SAD_L => (self.dma[0].source_addr & 0xffff) as u16,
            DMA3SAD_H => (self.dma[3].source_addr >> 16) as u16,
            DMA3DAD_L => (self.dma[3].dest_addr & 0xffff) as u16,
            DMA3DAD_H => (self.dma[3].dest_addr >> 16) as u16,
            DMA3CNT_L => self.dma[3].word_count,
            DMA3CNT_H => self.dma[3].read_cnt_h(),
            
            _ => { warn!("Unmapped read_u16 from {:08x} (IO)", address); 0 }
        }
    }
    
    fn io_read_u8(&self, address: usize) -> u8 {
        match address {
            POSTFLG => self.postflg,
            _ => (self.io_read_u16(address & 0xFFFFFFFE) >> ((address & 0x1) << 3)) as u8,
        }
    }

    pub fn write_u32(&mut self, address: usize, value: u32) {
        match address & 0x0F000000 {
            0x00000000 if address < 0x4000 => warn!("Ignored write to BIOS ({:08x})", address),
            0x02000000 => LittleEndian::write_u32(
                &mut self.eram[(address & 0x3ffff)..], value
            ),
            0x03000000 => LittleEndian::write_u32(
                &mut self.iram[(address & 0x7fff)..], value
            ),
            0x04000000 => self.io_write_u32(address, value),
            0x05000000 => self.gpu.pram_write_u32(address, value),
            0x06000000 => self.gpu.vram_write_u32(address, value),
            0x07000000 => self.gpu.oam_write_u32(address, value),

            // Ignore writes to GamePak
            0x08000000 |
            0x09000000 |
            0x0A000000 |
            0x0B000000 |
            0x0C000000 |
            0x0D000000 |
            0x0E000000 => { }
            _ => warn!("Unmapped write_u32 to {:08x} (value={:08x})", address, value)
        }
    }

    pub fn write_u16(&mut self, address: usize, value: u16) {
        match address & 0x0F000000 {
            0x00000000 if address < 0x4000 => warn!("Ignored write to BIOS ({:08x})", address),
            0x02000000 => LittleEndian::write_u16(
                &mut self.eram[(address & 0x3ffff)..], value
            ),
            0x03000000 => LittleEndian::write_u16(
                &mut self.iram[(address & 0x7fff)..], value
            ),
            0x04000000 => self.io_write_u16(address, value),
            0x05000000 => self.gpu.pram_write_u16(address, value),
            0x06000000 => self.gpu.vram_write_u16(address, value),
            0x07000000 => self.gpu.oam_write_u16(address, value),
            _ => warn!("Unmapped write_u16 to {:08x} (value={:04x})", address, value),
        }
    }
    
    pub fn write_u8(&mut self, address: usize, value: u8) {
        match address & 0x0F000000 {
            0x00000000 if address < 0x4000 => warn!("Ignored write to BIOS ({:08x})", address),
            0x02000000 => self.eram[address & 0x3ffff] = value,
            0x03000000 => self.iram[address & 0x7fff] = value,
            0x04000000 => self.io_write_u8(address, value),
            _ => warn!("Unmapped write_u8 to {:08x} (value={:02x})", address, value),
        }
    }

    fn io_write_u32(&mut self, address: usize, value: u32) {
        match address {
            // DMA writes are often 32bit so we handle them directly
            DMA0SAD_L => self.dma[0].source_addr = value,
            DMA0DAD_L => self.dma[0].dest_addr = value,
            DMA0CNT_L => self.dma[0].write_cnt(value),
            
            DMA1SAD_L => self.dma[1].source_addr = value,
            DMA1DAD_L => self.dma[1].dest_addr = value,
            DMA1CNT_L => self.dma[1].write_cnt(value),
            
            DMA2SAD_L => self.dma[2].source_addr = value,
            DMA2DAD_L => self.dma[2].dest_addr = value,
            DMA2CNT_L => self.dma[2].write_cnt(value),
            
            DMA3SAD_L => self.dma[3].source_addr = value,
            DMA3DAD_L => self.dma[3].dest_addr = value,
            DMA3CNT_L => self.dma[3].write_cnt(value),
            
            _ => {
                self.io_write_u16(address, value as u16);
                self.io_write_u16(address | 2, (value >> 16) as u16);
            }
        }
    }
    
    fn io_write_u16(&mut self, address: usize, value: u16) {
        self.io[(address & 0x3FF) >> 1] = value;
        
        match address {
            0x04000000..=0x04000056 => self.gpu.io_write_u16(address, value),
            0x04000060..=0x040000A8 => self.apu.io_write_u16(address, value),

            TM0CNT_L => self.timer[0].borrow_mut().write_cnt_l(value),
            TM0CNT_H => self.timer[0].borrow_mut().write_cnt_h(value),
            TM1CNT_L => self.timer[1].borrow_mut().write_cnt_l(value),
            TM1CNT_H => self.timer[1].borrow_mut().write_cnt_h(value),
            TM2CNT_L => self.timer[2].borrow_mut().write_cnt_l(value),
            TM2CNT_H => self.timer[2].borrow_mut().write_cnt_h(value),
            TM3CNT_L => self.timer[3].borrow_mut().write_cnt_l(value),
            TM3CNT_H => self.timer[3].borrow_mut().write_cnt_h(value),
            
            DMA0SAD_L =>
                self.dma[0].source_addr = (self.dma[0].source_addr & 0xffff0000) | (value as u32),
            DMA0SAD_H =>
                self.dma[0].source_addr = (self.dma[0].source_addr & 0xffff) | ((value as u32) << 16),
            DMA0DAD_L =>
                self.dma[0].dest_addr = (self.dma[0].dest_addr & 0xffff0000) | (value as u32),
            DMA0DAD_H =>
                self.dma[0].dest_addr = (self.dma[0].dest_addr & 0xffff) | ((value as u32) << 16),
            DMA0CNT_L => self.dma[0].word_count = value,
            DMA0CNT_H => self.dma[0].write_cnt_h(value),

            DMA1SAD_L =>
                self.dma[1].source_addr = (self.dma[1].source_addr & 0xffff0000) | (value as u32),
            DMA1SAD_H =>
                self.dma[1].source_addr = (self.dma[1].source_addr & 0xffff) | ((value as u32) << 16),
            DMA1DAD_L =>
                self.dma[1].dest_addr = (self.dma[1].dest_addr & 0xffff0000) | (value as u32),
            DMA1DAD_H =>
                self.dma[1].dest_addr = (self.dma[1].dest_addr & 0xffff) | ((value as u32) << 16),
            DMA1CNT_L => self.dma[1].word_count = value,
            DMA1CNT_H => self.dma[1].write_cnt_h(value),

            DMA2SAD_L =>
                self.dma[2].source_addr = (self.dma[2].source_addr & 0xffff0000) | (value as u32),
            DMA2SAD_H =>
                self.dma[2].source_addr = (self.dma[2].source_addr & 0xffff) | ((value as u32) << 16),
            DMA2DAD_L =>
                self.dma[2].dest_addr = (self.dma[2].dest_addr & 0xffff0000) | (value as u32),
            DMA2DAD_H =>
                self.dma[2].dest_addr = (self.dma[2].dest_addr & 0xffff) | ((value as u32) << 16),
            DMA2CNT_L => self.dma[2].word_count = value,
            DMA2CNT_H => self.dma[2].write_cnt_h(value),

            DMA3SAD_L =>
                self.dma[3].source_addr = (self.dma[3].source_addr & 0xffff0000) | (value as u32),
            DMA3SAD_H =>
                self.dma[3].source_addr = (self.dma[3].source_addr & 0xffff) | ((value as u32) << 16),
            DMA3DAD_L =>
                self.dma[3].dest_addr = (self.dma[3].dest_addr & 0xffff0000) | (value as u32),
            DMA3DAD_H =>
                self.dma[3].dest_addr = (self.dma[3].dest_addr & 0xffff) | ((value as u32) << 16),
            DMA3CNT_L => self.dma[3].word_count = value,
            DMA3CNT_H => self.dma[3].write_cnt_h(value),
            
            IE => self.irq.i_e = value,
            IF => self.irq.write_if(value),
            IME => self.irq.write_ime(value),
            _ => warn!("Unmapped write_u16 to {:08x} (IO, value={:04x})", address, value),
        }
    }
    
    fn io_write_u8(&mut self, address: usize, value: u8) {
        match address {
            HALTCNT => self.irq.halt = true,
            POSTFLG => self.postflg = value,
            _ => {
                let value16 = ((value as u16) << ((address & 1) << 3))
                    | (self.io[(address & 0x3FF) >> 1] & !(0xFF << ((address & 1) << 3)));
                
                self.io_write_u16(address & 0xFFFFFFFE, value16);
            }
        }
    }

    // IRQ
    #[inline]
    pub fn halt(&self) -> bool { self.irq.halt }

    // Frame
    #[inline]
    pub fn is_frame(&self) -> bool { self.gpu.is_frame() }
    pub fn ack_frame(&mut self) { self.gpu.ack_frame(); }
    
    pub fn load_bios(&mut self, filename: &str) -> Result<(), &'static str> {
        match File::open(filename) {
            Ok(mut file) => {    
                info!("BIOS file opened");
            
                if let Err(e) = file.read_to_end(&mut self.bios) {
                    error!("Error reading BIOS file : {}", e);
                    Err("Error reading BIOS")
                } else {
                    Ok(())
                }
            }

            Err(e) => {
                error!("Couldn't load BIOS : {}", e);
                Err("Error opening BIOS file")
            }
        }
    }

    pub fn load_rom(&mut self, filename: &str) -> bool {
        match File::open(filename) {
            Ok(mut file) => {
                info!("ROM file opened");

                if let Err(e) = file.read_to_end(&mut self.rom) {
                    error!("Error reading ROM file : {}", e);
                    false
                } else {
                    self.rom_len = self.rom.len();
                    true
                }
            }
            Err(e) => {
                error!("Couldn't open ROM file : {}", e);
                false
            }
        }
    }
}

const KEYINPUT: usize = 0x04000130;
const IE:       usize = 0x04000200;
const IF:       usize = 0x04000202;
const IME:      usize = 0x04000208;
const POSTFLG:  usize = 0x04000300;
const HALTCNT:  usize = 0x04000301;
