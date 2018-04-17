// macros.rs --- 
// 
// Filename: macros.rs
// Author: Louise <louise>
// Created: Mon Jan 22 16:58:00 2018 (+0100)
// Last-Updated: Tue Apr 17 14:56:19 2018 (+0200)
//           By: Louise <louise>
// 

macro_rules! unused_pattern {
    ($addr:expr, 8) => {{
        if $addr & 1 == 0 {
            unused_pattern!($addr, 16) >> 8
        } else {
            unused_pattern!($addr, 16)
        }
    }};
    ($addr:expr, 16) => {{
        ($addr >> 1)
    }};
    ($addr:expr, 32) => {{
        (unused_pattern!($addr, 16) << 16) | unused_pattern!($addr + 2, 16)
    }};
}

macro_rules! handle_dma {
    ($io:expr, $channel:expr) => {{
        let mut source_addr = $channel.source_addr as usize;
        let mut dest_addr = $channel.dest_addr as usize;
        let size = if $channel.word_size { 4 } else { 2 };
        
        for _ in 0..$channel.word_count {
            // Actual transfer
            if $channel.word_size {
                let v = $io.read_u32(source_addr);
                $io.write_u32(dest_addr, v);
            } else {
                let v = $io.read_u16(source_addr);
                $io.write_u16(dest_addr, v);
            };
            
            // Incrementing source
            if $channel.source_mode == 0 {
                source_addr += size;
            } else if $channel.source_mode == 1 {
                source_addr -= size;
            }

            // Incrementing dest
            if $channel.dest_mode == 0 {
                dest_addr += size;
            } else if $channel.dest_mode == 1 {
                dest_addr -= size;
            }
        }
        
        $channel.enable = false;
    }};
}
