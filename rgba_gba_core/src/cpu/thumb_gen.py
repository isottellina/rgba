# thumb_gen.py --- 
# 
# Filename: thumb_gen.py
# Author: Louise <louise>
# Created: Tue Jan 16 19:57:01 2018 (+0100)
# Last-Updated: Thu Jan 18 13:45:29 2018 (+0100)
#           By: Louise <louise>
#
def write_f2(high):
    imm = (high & 0x04) != 0
    op = (high & 0x02) != 0

    print("\tlet rd = instr & 7;")
    print("\tlet op1 = _cpu.registers[((instr >> 3) & 7) as usize];")

    if imm:
        print("\tlet op2 = ((instr >> 6) & 7) as u32;")
    else:
        print("\tlet op2 = _cpu.registers[((instr >> 6) & 7) as usize];")
    
    if op: # SUB
        print("\tlet res = op1.wrapping_sub(op2);")
        print("\t_cpu.carry = op1 >= op2;")
        print("\t_cpu.overflow = (op1 ^ op2) & (op1 ^ op2) & 0x80000000 != 0;")
    else: # ADD
        print("\tlet res = op1.wrapping_add(op2);")
        print("\t_cpu.carry = res < op1;")
        print("\t_cpu.overflow = !(op1 ^ op2) & (op1 ^ res) & 0x80000000 != 0;")

    print("\t_cpu.zero = res == 0;")
    print("\t_cpu.sign = (res as i32) < 0;")
    print("\t_cpu.registers[rd as usize] = res")
    
def write_f3(high):
    op = (high >> 3) & 0x3;

    print("\tlet rd = (instr >> 8) & 7;")
    print("\tlet val = (instr & 0xFF) as u32;")
    if op == 0: # MOV
        print("\tlet res = val;")
    elif op == 1 or op == 3: # CMP and SUB
        print("\tlet op1 = _cpu.registers[rd as usize];")
        print("\tlet res = op1.wrapping_sub(val as u32);")
        print("\t_cpu.carry = op1 >= val;")
        print("\t_cpu.overflow = (op1 ^ val) & (op1 ^ res) & 0x80000000 != 0;")
    elif op == 2: # ADD
        print("\tlet op1 = _cpu.registers[rd as usize];")
        print("\tlet res = op1.wrapping_add(val as u32);")
        print("\t_cpu.carry = res < op1;")
        print("\t_cpu.overflow = !(op1 ^ val) & (op1 ^ res) & 0x80000000 != 0;")

    print("\t_cpu.zero = res == 0;")
    print("\t_cpu.sign = (res as i32) < 0;")
    if op != 1:
        print("\t_cpu.registers[rd as usize] = res;")

def write_f4(high):
    print("\tlet op = (instr >> 6) & 0xf;")
    print("\tlet function = THUMB_ALU[op as usize];")
    print("\tfunction(_cpu, instr);")
        
def write_f5(high):
    op = high & 3

    print("\tlet rd = (instr & 7) | ((instr & 0x80) >> 4);")
    print("\tlet rs = _cpu.get_register(((instr >> 3) & 0xF) as usize);")
    
    if op == 0: # ADD
        print("\tlet op1 = _cpu.get_register(rd as usize);")
        print("\tlet res = op1.wrapping_add(rs);")
        print("\t_cpu.set_register(rd as usize, res);")
        print("\tif rd == 15 { _cpu.registers[15] &= 0xFFFFFFFE; _cpu.advance_pipeline(_io); }")
    elif op == 1: # CMP
        print("\tlet op1 = _cpu.get_register(rd as usize);")
        print("\tlet res = op1.wrapping_sub(rs);")
        print("\t_cpu.sign = (res as i32) < 0;")
        print("\t_cpu.zero = res == 0;")
        print("\t_cpu.carry = op1 >= res;")
        print("\t_cpu.overflow = (op1 ^ rs) & (op1 ^ res) & 0x80000000 != 0;")
        print("\tif rd == 15 { _cpu.registers[15] &= 0xFFFFFFFE; _cpu.advance_pipeline(_io); }")
    elif op == 2: # MOV
        print("\t_cpu.set_register(rd as usize, rs);")
        print("\tif rd == 15 { _cpu.registers[15] &= 0xFFFFFFFE; _cpu.advance_pipeline(_io); }")
    elif op == 3: # BX
        print("\tif rs & 1 == 0 {")
        print("\t\t_cpu.state = CpuState::ARM; _cpu.registers[15] = rs & 0xFFFFFFFC;")
        print("\t} else {")
        print("\t\t_cpu.registers[15] = rs & 0xFFFFFFFE;")
        print("\t}")
        print("\t_cpu.advance_pipeline(_io);")
        
def write_f6(high):
    rd = high & 7
    
    print("\tlet off = ((instr & 0xFF) as u32) << 2;")
    print("\tlet addr = (_cpu.registers[15] & 0xFFFFFFFC) + off;")
    print("\t_cpu.registers[%s] = _cpu.read_u32(_io, addr as usize);" % rd)

def write_f7(high):
    load = high & 0x08 != 0
    byte = high & 0x04 != 0

    print("\tlet rb = (instr >> 3) & 7;")
    print("\tlet ro = (instr >> 6) & 7;")
    print("\tlet rd = instr & 7;")
    print("\tlet addr = _cpu.registers[rb as usize].wrapping_add(_cpu.registers[ro as usize]);")
    if load:
        if byte:
            print("\tlet val = _cpu.read_u8(_io, addr as usize) as u32;")
        else:
            print("\tlet val = _cpu.read_u32(_io, addr as usize) as u32;")
        print("\t_cpu.registers[rd as usize] = val;")
    else:
        if byte:
            print("\t_cpu.write_u8(_io, addr as usize, _cpu.registers[rd as usize] as u8);")
        else:
            print("\t_cpu.write_u32(_io, addr as usize, _cpu.registers[rd as usize]);")

def write_f11(high):
    load = high & 0x08 != 0
    rd = high & 0x7
    print("\tlet off = ((instr & 0xFF) as usize) << 2;")
    print("\tlet sp = _cpu.get_register(13) as usize;")
    print("\tlet addr = off.wrapping_add(sp);")
    
    if load:
        print("\t_cpu.registers[%s] = _cpu.read_u32(_io, addr);" % rd)
    else:
        print("\tlet reg = _cpu.registers[%s];" % rd)
        print("\t_cpu.write_u32(_io, addr, reg)")
            
def write_f13(high):
    print("\tlet off = ((instr & 0x7f) as u32) << 2;")
    print("\tlet sp = _cpu.get_register(13);")
    print("\tif instr & 0x80 == 0 {")
    print("\t\t_cpu.set_register(13, sp + off);")
    print("\t} else {")
    print("\t\t_cpu.set_register(13, sp - off);")
    print("\t}")
            
def write_f14(high):
    pop = high & 0x08 != 0

    if pop:
        print("\tlet mut sp = _cpu.get_register(13) as usize;")
    else:
        print("\tlet count = (instr & 0x1FF).count_ones();")
        print("\tlet mut sp = (_cpu.get_register(13) - (count << 2)) as usize;")
        print("\t_cpu.set_register(13, sp as u32);")
    
    for i in range(9):
        print("\tif instr & (1 << %s) != 0 {" % i)
        
        if i == 8:
            i = 15 if pop else 14
        
        if pop:
            if i == 15:
                print("\t\t_cpu.registers[15] = _cpu.read_u32(_io, sp) & 0xFFFFFFFE;")
                print("\t\t_cpu.advance_pipeline(_io);")
            else:
                print("\t\t_cpu.registers[%s] = _cpu.read_u32(_io, sp);" % i)
        else:
            if i == 14:
                print("\t\tlet reg = _cpu.get_register(14);")
            else:
                print("\t\tlet reg = _cpu.registers[%s];" % i)
            print("\t\t_cpu.write_u32(_io, sp, reg);")

        print("\t\tsp += 4;")
        print("\t}")
        
    if pop:
        print("\t_cpu.set_register(13, sp as u32);")
        
            
def write_f16(high):
    conditions = [
        "_cpu.zero", "!_cpu.zero",
        "_cpu.carry", "!_cpu.carry",
        "_cpu.sign", "!_cpu.sign",
        "_cpu.overflow", "!_cpu.overflow",
        "_cpu.carry && !_cpu.zero", "!_cpu.carry && _cpu.zero",
        "_cpu.sign == _cpu.overflow", "_cpu.sign != _cpu.overflow",
        "_cpu.zero && (_cpu.sign == _cpu.overflow)", "!_cpu.zero || (_cpu.sign != _cpu.overflow)",
        "false", "false"
    ]
    
    print("\tlet off = (((instr & 0xFF) as i8) as i32) << 1;")
    print("\tif %s {" % conditions[high & 0xF])
    print("\t\t_cpu.registers[15] = (_cpu.registers[15] as i32).wrapping_add(off) as u32;")
    print("\t\t_cpu.advance_pipeline(_io);")
    print("\t}")

def write_f19(high):
    stage = (high & 0x08) >> 3

    if stage == 0:
        print("\tlet pc = _cpu.registers[15] as i32;")
        print("\tlet new_lr = pc + (((((instr & 0x7FF) as u32) << 21) as i32) >> 9);")
        print("\t_cpu.set_register(14, new_lr as u32);")
    else:
        print("\tlet tmp = (_cpu.registers[15] - 2) | 1;")
        print("\t_cpu.registers[15] = _cpu.get_register(14) + (((instr & 0x7FF) as u32) << 1);")
        print("\t_cpu.set_register(14, tmp);")
        print("\t_cpu.advance_pipeline(_io);")
    
def write_instruction(high):
    print("#[allow(unreachable_code, unused_variables, unused_assignments)]")
    print(
        "fn thumb_%02x(_cpu: &mut ARM7TDMI, _io: &mut Interconnect, instr: u16) {"
        % high
    )

    if high & 0xF8 == 0x18:
        write_f2(high)
    elif high & 0xE0 == 0x20:
        write_f3(high)
    elif high & 0xFC == 0x40:
        write_f4(high)
    elif high & 0xFC == 0x44:
        write_f5(high)
    elif high & 0xF8 == 0x48:
        write_f6(high)
    elif high & 0xF2 == 0x50:
        write_f7(high)
    elif high & 0xF0 == 0x90:
        write_f11(high)
    elif high & 0xFF == 0xB0:
        write_f13(high)
    elif high & 0xF6 == 0xB4:
        write_f14(high)
    elif high & 0xF0 == 0xD0:
        write_f16(high)
    elif high & 0xF0 == 0xF0:
        write_f19(high)
    else:
        print("\tunimplemented!(\"{:04x}\", instr);")

    print("}\n")

def write_alu(op):
    print("#[allow(unreachable_code, unused_variables, unused_assignments)]")
    print(
        "fn thumb_alu_%01x(_cpu: &mut ARM7TDMI, instr: u16) {"
        % op
    )

    test = op in [8, 10, 11]

    print("\tlet rd = instr & 0x7;")
    print("\tlet op1 = _cpu.registers[rd as usize];")
    print("\tlet rs = _cpu.registers[((instr >> 3) & 7) as usize];")
    
    if op == 8 or op == 0: # AND, TST
        print("\tlet res = op1 & rs;")
    elif op == 1: # EOR
        print("\tlet res = op1 | rs;");
    elif op == 2: # LSL
        print("\tlet shift = rs & 0xFF;")
        print("\tlet res = op1 << shift;")
        print("\tif shift != 0 { _cpu.carry = ((res << (shift - 1)) & 0x80000000) != 0; }")
    elif op == 3: # LSR
        print("\tlet shift = rs & 0xFF;")
        print("\tlet res = op1 >> shift;")
        print("\tif shift != 0 { _cpu.carry = ((res >> (shift - 1)) & 1) != 0; }")
    elif op == 4: # ASR
        print("\tlet shift = rs & 0xFF;")
        print("\tlet res = ((op1 as i32) >> shift) as u32;")
        print("\tif shift != 0 { _cpu.carry = (((res as i32) >> (shift - 1)) & 1) != 0; }")
    elif op == 5: # ADC
        print("\tlet res = op1.wrapping_add(rs).wrapping_add(_cpu.carry as u32);")
        print("\t_cpu.carry = if _cpu.carry { op1 >= res } else { op1 > res };")
        print("\t_cpu.overflow = !(op1 ^ rs) & (op1 ^ res) & 0x80000000 != 0;")
    elif op == 6: # SBC
        print("\tlet res = op1.wrapping_sub(rs).wrapping_sub(_cpu.carry as u32);")
        print("\t_cpu.carry = if _cpu.carry { op1 > rs } else { op1 >= rs };")
        print("\t_cpu.overflow = (op1 ^ rs) & (op1 ^ res) & 0x80000000 != 0;")
    elif op == 7: # ROR
        print("\tlet shift = rs & 0xFF;")
        print("\tlet res = op1.rotate_right(shift & 0x1f);")
        print("\tif shift != 0 { _cpu.carry = (res & 0x80000000) != 0; }")
    elif op == 9: # NEG
        print("\tlet res = 0_u32.wrapping_sub(rs);")
        print("\t_cpu.carry = 0 >= res;")
        print("\t_cpu.overflow = (rs & res & 0x80000000) != 0;")
    elif op == 10: # CMP
        print("\tlet res = op1.wrapping_sub(rs);")
        print("\t_cpu.carry = op1 >= rs;")
        print("\t_cpu.overflow = (op1 ^ rs) & (op1 ^ res) & 0x80000000 != 0;")
    elif op == 11: # CMN
        print("\tlet res = op1.wrapping_add(rs);")
        print("\t_cpu.carry = op1 > res;")
        print("\t_cpu.overflow = !(op1 ^ rs) & (op1 ^ res) & 0x80000000 != 0;")
    elif op == 12: # ORR
        print("\tlet res = op1 | rs;")
    elif op == 13: # MUL
        print("\tlet res = op1.wrapping_mul(rs);")
        print("\t_cpu.carry = false;")
    elif op == 14: # BIC
        print("\tlet res = op1 & !rs;");
    elif op == 15: # MVN
        print("\tlet res = !rs;");
        
    print("\t_cpu.sign = (res as i32) < 0;")
    print("\t_cpu.zero = res == 0;")

    if not test:
        print("\t_cpu.set_register(rd as usize, res);")

    print("}\n")
    
for high in range(0x100):
    write_instruction(high)

for opcode in range(0x10):
    write_alu(opcode)
    
print("const THUMB_INSTRUCTIONS: [fn(&mut ARM7TDMI, &mut Interconnect, u16); 256] = ", end = "")
print("[" + ", ".join(["thumb_%02x" % i for i in range(0x0, 0x100)]) + "];")

print("const THUMB_ALU: [fn(&mut ARM7TDMI, u16); 16] = ", end = "")
print("[" + ", ".join(["thumb_alu_%01x" % i for i in range(0x0, 0x10)]) + "];")
