// disasm.rs --- 
// 
// Filename: disasm.rs
// Author: Louise <louise>
// Created: Mon Jan  8 14:49:33 2018 (+0100)
// Last-Updated: Mon Jan  8 16:51:37 2018 (+0100)
//           By: Louise <louise>
// 

const CONDITIONS: [&str; 16] = [
    "eq", "ne", "cs", "cc",
    "mi", "pl", "vs", "vc",
    "hi", "ls", "ge", "lt",
    "gt", "le", "",   "nv"
];

const ARM_INSTRS: [(u32, u32, &str); 3] = [
    // Branches
    (0x0F000000, 0x0A000000, "b%c %o"),
    (0x0F000000, 0x0B000000, "bl%c %o"),
    (0x0FFFFFF0, 0x012FFF10, "bx%c %r")
];

pub fn disasm_arm(offset: u32, instr: u32) -> String {
    let mut dis = String::new();
    
    for &(mask, res, disasm) in ARM_INSTRS.iter() {
        if instr & mask == res {
            let mut it = disasm.chars();

            while let Some(c) = it.next() {
                if c == '%' {
                    match it.next() {
                        Some('c') =>
                            dis.push_str(CONDITIONS[(instr >> 28) as usize]),
                        Some('r') =>
                            dis.push_str(&format!("r{}", instr & 0xF)),
                        Some('o') => {
                            let mut off = instr & 0xFFFFFF;
                            
			    if off & 0x800000 != 0 {
				off |= 0xff000000;
			    }
                            
                            off <<= 2;
                            
                            dis.push_str(
                                &format!("0x{:x}",
                                         offset as i32 + off as i32 + 8
                                )
                            )
                        },
                        Some(e) => println!("{}", e),
                        _ => panic!()
                    }
                } else {
                    dis.push(c);
                }
            }
        }
    }

    dis
}

pub fn disasm_thumb(instr: u16) -> String {
    format!("")
}
