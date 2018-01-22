// disasm.rs --- 
// 
// Filename: disasm.rs
// Author: Louise <louise>
// Created: Mon Jan  8 14:49:33 2018 (+0100)
// Last-Updated: Mon Jan 22 15:15:43 2018 (+0100)
//           By: Louise <louise>
// 
use io::Interconnect;

const CONDITIONS: [&str; 16] = [
    "eq", "ne", "cs", "cc",
    "mi", "pl", "vs", "vc",
    "hi", "ls", "ge", "lt",
    "gt", "le", "",   "nv"
];

const SHIFTS: [&str; 5] = [
    "lsl", "lsr", "asr", "ror", "rrx"
];

const REGS: [&str; 16] = [
    "r0", "r1", "r2", "r3",
    "r4", "r5", "r6", "r7",
    "r8", "r9", "r10", "r11",
    "r12", "sp", "lr", "pc"
];

const ARM_INSTRS: [(u32, u32, &str); 29] = [
    // Branches
    (0x0F000000, 0x0A000000, "b%c %o"),
    (0x0F000000, 0x0B000000, "bl%c %o"),
    (0x0FFFFFF0, 0x012FFF10, "bx%c %r0"),
    // PSR Transfer
    (0x0FBF0FFF, 0x010F0000, "msr%c %r3, %p"),
    (0x0DB0F000, 0x0120F000, "msr%c %p, %i"),
    // Multiply
    (0x0FE000F0, 0x00000090, "mul%s%c %r4, %r0, %r2"),
    (0x0FE000F0, 0x00200090, "mla%s%c %r4, %r0, %r2, %r3"),
    // Multiply long
    (0x0FA000F0, 0x00800090, "%umull%s%c %r3, %r4, %r0, %r2"),
    (0x0FA000F0, 0x00A00090, "%umlal%s%c %r3, %r4, %r0, %r2"),
    // Load/Store instructions
    (0x0C100000, 0x04000000, "str%b%t%c %r3, %a"),
    (0x0C100000, 0x04100000, "ldr%b%t%c %r3, %a"),
    // STM/LDM
    (0x0E100000, 0x08000000, "stm%m%c %r4%!, %l"),
    (0x0E100000, 0x08100000, "ldm%m%c %r4%!, %l"),
    // ALU
    (0x0DE00000, 0x00000000, "and%s%c %r3, %r4, %i"),
    (0x0DE00000, 0x00200000, "eor%s%c %r3, %r4, %i"),
    (0x0DE00000, 0x00400000, "sub%s%c %r3, %r4, %i"),
    (0x0DE00000, 0x00600000, "rsb%s%c %r3, %r4, %i"),
    (0x0DE00000, 0x00800000, "add%s%c %r3, %r4, %i"),
    (0x0DE00000, 0x00A00000, "adc%s%c %r3, %r4, %i"),
    (0x0DE00000, 0x00C00000, "sbc%s%c %r3, %r4, %i"),
    (0x0DE00000, 0x00E00000, "rsc%s%c %r3, %r4, %i"),
    (0x0DE00000, 0x01000000, "tst%c %r4, %i"),
    (0x0DE00000, 0x01200000, "teq%c %r4, %i"),
    (0x0DE00000, 0x01400000, "cmp%c %r4, %i"),
    (0x0DE00000, 0x01600000, "cmn%c %r4, %i"),
    (0x0DE00000, 0x01800000, "orr%s%c %r3, %r4, %i"),
    (0x0DE00000, 0x01A00000, "mov%s%c %r3, %i"),
    (0x0DE00000, 0x01C00000, "bic%s%c %r3, %r4, %i"),
    (0x0DE00000, 0x01E00000, "mvn%s%c %r3, %i"),
];

pub fn disasm_arm(io: &Interconnect, offset: u32) -> String {
    let instr = io.read_u32(offset as usize);
    let mut dis = String::new();
    
    for &(mask, res, disasm) in ARM_INSTRS.iter() {
        if instr & mask == res {
            let mut it = disasm.chars();

            while let Some(c) = it.next() {
                if c == '%' {
                    match it.next() {
                        Some('c') =>
                            dis.push_str(CONDITIONS[(instr >> 28) as usize]),
                        Some('!') =>
                            if instr & 0x00200000 != 0 { dis.push('!'); },
                        Some('b') =>
                            if instr & 0x00400000 != 0 { dis.push('b'); },
                        Some('t') =>
                            if instr & 0x01200000 == 0x00200000 { dis.push('t'); },
                        Some('p') => {
                            dis.push_str(if instr & 0x400000 != 0 { "spsr" } else { "cpsr" });

                            if instr & 0x00010000 == 0 {
                                dis.push_str("_flg");
                            }
                        },
                        Some('r') => {
                            let shifted = instr >> (it.next().unwrap().to_digit(10).unwrap() << 2);
                            
                            dis.push_str(REGS[(shifted & 0xF) as usize])
                        }
                        Some('s') => if instr & 0x100000 != 0 { dis.push('s'); },
                        Some('u') => if instr & 0x400000 != 0 { dis.push('u'); },
                        Some('i') => {
                            if instr & 0x02000000 != 0 {
                                let imm = instr & 0xFF;
                                let rot = (instr & 0xF00) >> 7;

                                dis.push_str(&format!("0x{:08x}", imm.rotate_right(rot)));
                            } else {
                                let rm = instr & 0xF;
                                let mut shift = (instr & 0x60) >> 5;

                                dis.push_str(REGS[rm as usize]);
                                
                                if instr & 0x10 != 0 {
                                    dis.push(' ');
                                    dis.push_str(SHIFTS[shift as usize]);
                                    dis.push(' ');
                                    dis.push_str(REGS[((instr & 0xF00) >> 8) as usize]);
                                } else {
                                    let mut amount = (instr & 0xF80) >> 7;

                                    if amount == 0 && shift == 3 { shift = 4 }
                                    if amount == 0 { amount = 32; }

                                    if amount != 32 || shift != 0 {
                                        dis.push(' ');
                                        dis.push_str(SHIFTS[shift as usize]);
                                        dis.push_str(&format!(" #{}", amount));
                                    }
                                }
                            }
                        }
                        Some('a') => {
                            fn push_op(dis: &mut String, instr: u32) {
                                let dir = (instr & 0x00800000) != 0;
                                let imm = (instr & 0x02000000) == 0;

                                dis.push(if dir { '+' } else { '-' });

                                if imm {
                                    let d = instr & 0xFFF;
                                    dis.push_str(&format!("0x{:x}", d));
                                } else {
                                    let mut shift = (instr & 0x60) >> 5;
                                    let mut amount = (instr & 0xF00) >> 8;
                                    let rm = instr & 0xF;

                                    if shift == 3 && amount == 0 { shift = 4; }
                                    if amount == 0 { amount = 32; }

                                    dis.push(' ');
                                    dis.push_str(REGS[rm as usize]);
                                    
                                    if amount != 32 || shift != 0 {
                                        dis.push(' ');
                                        dis.push_str(SHIFTS[shift as usize]);
                                        dis.push_str(&format!(" #{}", amount));
                                    }
                                }
                            }
                                
                            let rn = (instr >> 16) & 0xF;
                            let pre = (instr & 0x01000000) != 0;

                            if rn == 15 {
                                push_op(&mut dis, instr);
                            } else if pre {
                                dis.push_str(&format!("[{}, ", REGS[rn as usize]));
                                push_op(&mut dis, instr);
                                dis.push_str(&format!("]"));

                                if (instr & 0x00200000) != 0 {
                                    dis.push('!');
                                }
                            } else {
                                dis.push_str(&format!("[{}], ", REGS[rn as usize]));
                                push_op(&mut dis, instr);
                            }
                        }
                        Some('l') => {
			    let mut msk = 0;
			    let mut not_first = false;

                            dis.push('{');
                            
                            while msk < 16 {
				if (instr & (1 << msk)) != 0 {
				    let fr = msk;
				    while (instr & (1 << msk)) != 0 && msk < 16 { msk += 1; }
				    let to = msk - 1;
				    if not_first { dis.push(','); }

                                    dis.push_str(REGS[fr as usize]);
                                    
				    if fr != to {
					if fr == to-1 { dis.push(','); }
					else { dis.push('-'); }
					dis.push_str(REGS[to as usize]);
				    }
                                    
				    not_first = true;
				} else {
				    msk += 1;
				}
                            }

                            dis.push('}');
                        }
                        Some('m') => {
                            dis.push(if instr &  0x800000 != 0 { 'i' } else { 'd' });
                            dis.push(if instr & 0x1000000 != 0 { 'b' } else { 'a' });
                        }
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

            break;
        }
    }

    if dis.len() == 0 {
        "Couldn't disassemble this instruction".to_owned()
    } else {
        dis
    }
}

const THUMB_INSTRS: [(u16, u16, &str); 59] = [
    // Format 1 (move shifted register)
    (0xF800, 0x0000, "lsl %r0, %r3, %s"),
    (0xF800, 0x0800, "lsr %r0, %r3, %s"),
    (0xF800, 0x1000, "asr %r0, %r3, %s"),
    // Format 2 (add/substract)
    (0xFA00, 0x1800, "add %r0, %r3, %I"),
    (0xFA00, 0x1A00, "sub %r0, %r3, %I"),
    // Format 3 (move/cmp/add/sub immediate)
    (0xF800, 0x2000, "mov %r8, %i"),
    (0xF800, 0x2800, "cmp %r8, %i"),
    (0xF800, 0x3000, "add %r8, %i"),
    (0xF800, 0x3800, "sub %r8, %i"),
    // Format 4 (ALU operations)
    (0xFFC0, 0x4000, "and %r0, %r3"),
    (0xFFC0, 0x4040, "eor %r0, %r3"),
    (0xFFC0, 0x4080, "lsl %r0, %r3"),
    (0xFFC0, 0x40C0, "lsr %r0, %r3"),
    (0xFFC0, 0x4100, "asr %r0, %r3"),
    (0xFFC0, 0x4140, "adc %r0, %r3"),
    (0xFFC0, 0x4180, "sbc %r0, %r3"),
    (0xFFC0, 0x41C0, "ror %r0, %r3"),
    (0xFFC0, 0x4200, "tst %r0, %r3"),
    (0xFFC0, 0x4240, "neg %r0, %r3"),
    (0xFFC0, 0x4280, "cmp %r0, %r3"),
    (0xFFC0, 0x42C0, "cmn %r0, %r3"),
    (0xFFC0, 0x4300, "orr %r0, %r3"),
    (0xFFC0, 0x4340, "mul %r0, %r3"),
    (0xFFC0, 0x4380, "bic %r0, %r3"),
    (0xFFC0, 0x43C0, "mvn %r0, %r3"),
    // Format 5 (Hi register operations)
    (0xFF00, 0x4400, "add %h07, %h36"),
    (0xFF00, 0x4500, "cmp %h07, %h36"),
    (0xFF00, 0x4600, "mov %h07, %h36"),
    (0xFF00, 0x4700, "bx %h36"),
    // Format 6 (PC-relative load)
    (0xF800, 0x4800, "ldr %r8, [%p]"),
    // Format 7 (Load/Store with register offset)
    (0xFA00, 0x5000, "str%ba %r0, [%r3, %r6]"),
    (0xFA00, 0x5800, "ldr%ba %r0, [%r3, %r6]"),
    // Format 8 (Load/Store Sign-Extended byte/Halfword)
    (0xFE00, 0x5200, "strh %r0, [%r3, %r6]"),
    (0xFE00, 0x5A00, "ldrh %r0, [%r3, %r6]"),
    (0xFE00, 0x5600, "ldsb %r0, [%r3, %r6]"),
    (0xFE00, 0x5E00, "ldsh %r0, [%r3, %r6]"),
    // Format 9 (Load/Store with immediate offset)
    (0xE800, 0x6000, "str%bc %r0, [%r3, %s]"),
    (0xE800, 0x6800, "ldr%bc %r0, [%r3, %s]"),
    // Format 10 (load/store halfword)
    (0xF800, 0x8000, "strh %r0, [%r3, %s]"),
    (0xF800, 0x8800, "ldrh %r0, [%r3, %s]"),
    // Format 11 (load/store with SP base)
    (0xF800, 0x9000, "str %r8, [sp, %f]"),
    (0xF800, 0x9800, "ldr %r8, [sp, %f]"),
    // Format 12 (load address)
    (0xF800, 0xA000, "add %r8, pc, %f"),
    (0xF800, 0xA800, "add %r8, sp, %f"),
    // Format 13 (add offset to SP)
    (0xFF80, 0xB000, "add sp, %m"),
    (0xFF80, 0xB080, "sub sp, %m"),
    // Format 14
    (0xFFFF, 0xB500, "push {pc}"),
    (0xFF00, 0xB500, "push {%l,pc}"),
    (0xFF00, 0xB400, "push {%l}"),
    (0xFFFF, 0xBD00, "pop {pc}"),
    (0xFF00, 0xBD00, "pop {%l,pc}"),
    (0xFF00, 0xBC00, "pop {%l}"),
    // Format 15
    (0xF800, 0xC000, "stmia %r8!, {%l}"),
    (0xF800, 0xC800, "ldmia %r8!, {%l}"),
    // Format 17 (SWI)
    (0xFF00, 0xDF00, "swi %w"),
    // Format 16 (Conditionnal branch)
    (0xF000, 0xD000, "b%c %o"),
    // Format 18 (Unconditionnal branch)
    (0xF800, 0xE000, "b %u"),
    // Format 19 (Long branch)
    (0xF800, 0xF000, "bl %Lh"),
    (0xF800, 0xF800, "bl %Ll"),
];

pub fn disasm_thumb(io: &Interconnect, offset: u32) -> String {
    let instr = io.read_u16(offset as usize);
    let mut dis = String::new();
    
    for &(mask, res, disasm) in THUMB_INSTRS.iter() {
        if instr & mask == res {
            let mut it = disasm.chars();

            while let Some(c) = it.next() {
                if c == '%' {
                    match it.next() {
                        Some('r') => {
                            let shifted = instr >> it.next().unwrap().to_digit(10).unwrap();
                            
                            dis.push_str(REGS[(shifted & 0x7) as usize])
                        }
                        Some('h') => {
                            let r = (instr >> it.next().unwrap().to_digit(10).unwrap()) & 7;
                            let h = (instr & (1 << it.next().unwrap().to_digit(10).unwrap())) != 0;

                            dis.push_str(REGS[(if h { r + 8 } else { r }) as usize]);
                        }
                        Some('m') => dis.push_str(&format!("0x{:x}", (instr & 0x7f) << 2)),
                        Some('f') => dis.push_str(&format!("0x{:x}", (instr & 0xff) << 2)),
                        Some('b') => if instr & (1 << it.next().unwrap().to_digit(16).unwrap()) != 0 {
                            dis.push('b');
                        },
                        Some('c') => dis.push_str(CONDITIONS[((instr >> 8) & 0xF) as usize]),
                        Some('p') => dis.push_str(
                            &format!("0x{:08x}", (offset & !3) + (((instr & 0xFF) as u32) << 2) + 4)
                        ),
                        Some('s') => dis.push_str(&format!("#{}", (instr >> 6) & 0x1f)),
                        Some('u') => dis.push_str(
                            &format!("0x{:08x}", offset + (((instr & 0x7ff) << 1) as u32) + 4)
                        ),
                        Some('i') => dis.push_str(&format!("0x{:02x}", instr & 0xFF)),
                        Some('I') => {
                            if instr & 0x0400 != 0 {
                                dis.push_str(&format!("#{}", (instr >> 6) & 0x7));
                            } else {
                                dis.push_str(REGS[((instr >> 6) & 0x7) as usize]);
                            }
                        }
                        Some('o') => {
                            let off = (((instr & 0xFF) as i8) as i32) << 1;
                            
                            dis.push_str(
                                &format!("0x{:x}",
                                         offset as i32 + off as i32 + 4
                                )
                            )
                        }
                        Some('l') => {
			    let mut msk = 0;
			    let mut not_first = false;
                            
                            while msk < 8 {
				if (instr & (1 << msk)) != 0 {
				    let fr = msk;
				    while (instr & (1 << msk)) != 0 && msk < 8 { msk += 1; }
				    let to = msk - 1;
				    if not_first { dis.push(','); }

                                    dis.push_str(REGS[fr as usize]);
                                    
				    if fr != to {
					if fr == to-1 { dis.push(','); }
					else { dis.push('-'); }
					dis.push_str(REGS[to as usize]);
				    }
                                    
				    not_first = true;
				} else {
				    msk += 1;
				}
                            }
                        }
                        Some('L') => {
                            let (pc, instr, next_instr) = match it.next().unwrap() {
                                'h' => (offset, instr, io.read_u16((offset + 2) as usize)),
                                'l' => (offset - 2, io.read_u16((offset - 2) as usize), instr),
                                _ => panic!("Bad disasm data"),
                            };
                            
                            let offset_high = ((((instr & 0x7FF) as u32) << 21) as i32) >> 9;
                            let offset_low = (next_instr & 0x7FF) as i32;
                            let addr = (pc as i32) + 4 + offset_high + (offset_low << 1);

                            dis.push_str(&format!("0x{:08x}", addr));
                        }
                        _ => panic!("Bad disasm data"),
                    }
                } else {
                    dis.push(c);
                }
            }

            break;
        }
    }

    if dis.len() == 0 {
        "Couldn't disassemble this instruction".to_owned()
    } else {
        dis
    }
}
