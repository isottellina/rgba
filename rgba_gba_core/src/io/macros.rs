// macros.rs --- 
// 
// Filename: macros.rs
// Author: Louise <louise>
// Created: Mon Jan 22 16:58:00 2018 (+0100)
// Last-Updated: Mon Jan 22 17:01:57 2018 (+0100)
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
