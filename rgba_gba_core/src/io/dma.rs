// dma.rs --- 
// 
// Filename: dma.rs
// Author: Louise <louise>
// Created: Thu Feb  1 16:51:05 2018 (+0100)
// Last-Updated: Thu Nov  5 21:11:57 2020 (+0100)
//           By: Louise <louise>
// 

#[derive(Default)]
pub struct DmaChannel {
    pub channel: u32,
    
    pub source_addr: u32,
    pub dest_addr: u32,
    pub word_count: u16,
    
    pub source_mode: u16,
    pub dest_mode: u16,
    pub repeat: bool,
    pub word_size: bool,
    pub drq: bool,
    pub start_timing: u16,
    pub irq_en: bool,
    pub enable: bool
}

impl DmaChannel {
    pub fn new(channel: u32) -> DmaChannel {
        let mut dma = DmaChannel::default();
        dma.channel = channel;

        dma
    }

    #[inline]
    pub fn read_cnt_h(&self) -> u16 {
        return (self.dest_mode << 5)
            | (self.source_mode << 7)
            | ((self.repeat as u16) << 9)
            | ((self.word_size as u16) << 10)
            | ((self.drq as u16) << 11)
            | (self.start_timing << 12)
            | ((self.irq_en as u16) << 14)
            | ((self.enable as u16) << 15);
    }
    
    #[inline]
    pub fn write_cnt(&mut self, cnt: u32) {
        self.word_count = cnt as u16;
        self.write_cnt_h((cnt >> 16) as u16);
    }

    #[inline]
    pub fn write_cnt_h(&mut self, cnt: u16) {
        self.dest_mode = (cnt >> 5) & 3;
        self.source_mode = (cnt >> 7) & 3;
        self.repeat = ((cnt >> 9) & 1) != 0;
        self.word_size = ((cnt >> 10) & 1) != 0;
        self.drq = ((cnt >> 11) & 1) != 0;
        self.start_timing = (cnt >> 12) & 3;
        self.irq_en = ((cnt >> 14) & 1) != 0;
        self.enable = ((cnt >> 15) & 1) != 0;

        if self.enable {
            warn!("Writing to DMA{}CNT_H with value {:04x} (from {:08x} to {:08x})", self.channel, cnt, self.source_addr, self.dest_addr);
        }
    }
}

// Channel 0 I/O ports
pub const DMA0SAD_L: usize = 0x040000B0;
pub const DMA0SAD_H: usize = 0x040000B2;
pub const DMA0DAD_L: usize = 0x040000B4;
pub const DMA0DAD_H: usize = 0x040000B6;
pub const DMA0CNT_L: usize = 0x040000B8;
pub const DMA0CNT_H: usize = 0x040000BA;

// Channel 1 I/O ports
pub const DMA1SAD_L: usize = 0x040000BC;
pub const DMA1SAD_H: usize = 0x040000BE;
pub const DMA1DAD_L: usize = 0x040000C0;
pub const DMA1DAD_H: usize = 0x040000C2;
pub const DMA1CNT_L: usize = 0x040000C4;
pub const DMA1CNT_H: usize = 0x040000C6;

// Channel 2 I/O ports
pub const DMA2SAD_L: usize = 0x040000C8;
pub const DMA2SAD_H: usize = 0x040000CA;
pub const DMA2DAD_L: usize = 0x040000CC;
pub const DMA2DAD_H: usize = 0x040000CE;
pub const DMA2CNT_L: usize = 0x040000D0;
pub const DMA2CNT_H: usize = 0x040000D2;

// Channel 3 I/O ports
pub const DMA3SAD_L: usize = 0x040000D4;
pub const DMA3SAD_H: usize = 0x040000D6;
pub const DMA3DAD_L: usize = 0x040000D8;
pub const DMA3DAD_H: usize = 0x040000DA;
pub const DMA3CNT_L: usize = 0x040000DC;
pub const DMA3CNT_H: usize = 0x040000DE;
