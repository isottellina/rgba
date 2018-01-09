// disasm.rs --- 
// 
// Filename: disasm.rs
// Author: Louise <louise>
// Created: Wed Dec 13 17:39:26 2017 (+0100)
// Last-Updated: Tue Jan  9 12:56:11 2018 (+0100)
//           By: Louise <louise>
// 
use io::Interconnect;

static CONDITIONS: [&str; 4] = ["NZ", "Z", "NC", "C"];
static ALU: [&str; 8] = ["ADD A,", "ADC A,", "SUB", "SBC",
                         "AND", "XOR", "OR", "CP"];
static ROT: [&str; 8] = ["RLC", "RRC", "RL", "RR", "SLA", "SRA", "SWAP", "SRL"];
static R: [&str; 8] = ["B", "C", "D", "E", "H", "L", "(HL)", "A"];
static RP: [&str; 4] = ["BC", "DE", "HL", "SP"];
static RP2: [&str; 4] = ["BC", "DE", "HL", "AF"];

// Returns an address relative to the instruction
fn rel_address(io: &Interconnect, addr: usize) -> String {
    let diff = io.read_u8(addr) as i8;
    let address = addr.wrapping_add(diff as usize);

    format!("(0x{:04X})", address + 1)
}

// Returns an address
fn address(io: &Interconnect, addr: usize) -> String {
    let address = io.read_u16(addr);

    format!("(0x{:04X})", address)
}

// Returns an unsigned byte
fn d8(io: &Interconnect, addr: usize) -> String {
    format!("0x{:02X}", io.read_u8(addr))
}

// Returns a signed byte
fn s8(io: &Interconnect, addr: usize) -> i8 { io.read_u8(addr) as i8 }

// Returns an unsigned word
fn d16(io: &Interconnect, addr: usize) -> String {
    format!("0x{:04X}", io.read_u16(addr))
}

pub fn disasm(io: &Interconnect, addr: usize) -> String {
    let opcode = io.read_u8(addr);
    
    if opcode != 0xCB {
        let x = opcode >> 6;
        let y = (opcode >> 3) & 0x7;
        let z = opcode & 0x7;

        match (x, y, z) {
            // X = 0 and Z = 0
            (0, 0, 0) => "NOP".to_owned(),
            (0, 1, 0) => format!("LD {}, SP", address(io, addr + 1)),
            (0, 2, 0) => "STOP".to_owned(),
            (0, 3, 0) => format!("JR {}", rel_address(io, addr + 1)),
            (0, 4...7, 0) => format!("JR {}, {}",
                                     CONDITIONS[(y - 4) as usize],
                                     rel_address(io, addr + 1)
            ),
            
            // X = 0 and Z = 1
            (0, _, 1) => match y & 1 {
                0 => format!("LD {}, {}",
                             RP[(y >> 1) as usize], d16(io, addr + 1)),
                1 => format!("ADD HL, {}", RP[(y >> 1) as usize]),
                _ => unreachable!()
            },

            // X = 0 and Z = 2
            (0, 0, 2) => "LD (BC), A".to_owned(),
            (0, 1, 2) => "LD A, (BC)".to_owned(),
            (0, 2, 2) => "LD (DE), A".to_owned(),
            (0, 3, 2) => "LD A, (DE)".to_owned(),
            (0, 4, 2) => "LD (HL+), A".to_owned(),
            (0, 5, 2) => "LD A, (HL+)".to_owned(),
            (0, 6, 2) => "LD (HL-), A".to_owned(),
            (0, 7, 2) => "LD A, (HL-)".to_owned(),

            // X = 0 and Z = 3
            (0, _, 3) => match y & 1 {
                0 => format!("INC {}", RP[(y >> 1) as usize]),
                1 => format!("DEC {}", RP[(y >> 1) as usize]),
                _ => unreachable!(),
            },

            // X = 0 and Z = 4
            (0, _, 4) => format!("INC {}", R[y as usize]),
            // X = 0 and Z = 5
            (0, _, 5) => format!("DEC {}", R[y as usize]),
            // X = 0 and Z = 6
            (0, _, 6) => format!("LD {}, {}", R[y as usize], d8(io, addr + 1)),

            // X = 0 and Z = 7
            (0, 0, 7) => "RLCA".to_owned(),
            (0, 1, 7) => "RRCA".to_owned(),
            (0, 2, 7) => "RLA".to_owned(),
            (0, 3, 7) => "RRA".to_owned(),
            (0, 4, 7) => "DAA".to_owned(),
            (0, 5, 7) => "CPL".to_owned(),
            (0, 6, 7) => "SCF".to_owned(),
            (0, 7, 7) => "CCF".to_owned(),

            // X = 1
            (1, 6, 6) => "HALT".to_owned(),
            (1, _, _) => format!("LD {}, {}", R[y as usize], R[z as usize]),

            // X = 2
            (2, _, _) => format!("{} {}", ALU[y as usize], R[z as usize]),

            // X = 3 and Z = 0
            (3, 0...3, 0) => format!("RET {}", CONDITIONS[y as usize]),
            (3, 4, 0) => format!("LD (0xFF{:02X}), A", io.read_u8(addr + 1)),
            (3, 5, 0) => format!("ADD SP, {:02X}", s8(io, addr + 1)),
            (3, 6, 0) => format!("LD A, (0xFF{:02X})", io.read_u8(addr + 1)),
            (3, 7, 0) => format!("LD HL, SP + {}", s8(io, addr + 1)),

            // X = 3 and Z = 1
            (3, _, 1) => match (y >> 1, y & 1) {
                (p, 0) => format!("POP {}", RP2[p as usize]),
                (0, 1) => "RET".to_owned(),
                (1, 1) => "RETI".to_owned(),
                (2, 1) => "JP (HL)".to_owned(),
                (3, 1) => "LD SP, HL".to_owned(),
                _ => unreachable!(),
            },

            // X = 3 and Z = 2
            (3, 0...3, 2) => format!("JP {}, {}",
                                     CONDITIONS[y as usize],
                                     address(io, addr + 1)),
            (3, 4, 2) => "LD (C), A".to_owned(),
            (3, 5, 2) => format!("LD {}, A", address(io, addr + 1)),
            (3, 6, 2) => "LD A, (C)".to_owned(),
            (3, 7, 2) => format!("LD A, {}", address(io, addr + 1)),
            
            // X = 3 and Z = 3
            (3, 0, 3) => format!("JP {}", address(io, addr + 1)),
            (3, 6, 3) => "DI".to_owned(),
            (3, 7, 3) => "EI".to_owned(),

            // X = 3 and Z = 4
            (3, 0...3, 4) => format!("CALL {}, {}",
                                     CONDITIONS[y as usize],
                                     address(io, addr + 1)),

            // X = 3 and Z = 5
            (3, 0, 5) | (3, 2, 5) | (3, 4, 5) | (3, 6, 5) =>
                format!("PUSH {}", RP2[(y >> 1) as usize]),
            (3, 1, 5) => format!("CALL {}", address(io, addr + 1)),

            // X = 3 and Z = 6
            (3, _, 6) => format!("{} {}", ALU[y as usize], d8(io, addr + 1)),

            // X = 3 and Z = 7
            (3, _, 7) => format!("RST 0x{:02X}", y << 3),
            
            _ => "Unknown instruction".to_string(),
        }
    } else {
        let opcode = io.read_u8(addr + 1);
        
        let x = opcode >> 6;
        let y = (opcode >> 3) & 0x7;
        let z = opcode & 0x7;

        match x {
            0 => format!("{} {}", ROT[y as usize], R[z as usize]),
            1 => format!("BIT {}, {}", y, R[z as usize]),
            2 => format!("RES {}, {}", y, R[z as usize]),
            3 => format!("SET {}, {}", y, R[z as usize]),

            _ => unreachable!()
        }
    }
}
