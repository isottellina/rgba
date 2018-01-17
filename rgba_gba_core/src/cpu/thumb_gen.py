# thumb_gen.py --- 
# 
# Filename: thumb_gen.py
# Author: Louise <louise>
# Created: Tue Jan 16 19:57:01 2018 (+0100)
# Last-Updated: Wed Jan 17 12:20:08 2018 (+0100)
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

def write_instruction(high):
    print("#[allow(unreachable_code, unused_variables)]")
    print(
        "fn thumb_%02x(_cpu: &mut ARM7TDMI, _io: &mut Interconnect, instr: u16) {"
        % high
    )

    if high & 0xF8 == 0x18:
        write_f2(high)
    elif high & 0xE0 == 0x20:
        write_f3(high)
    elif high & 0xF8 == 0x48:
        write_f6(high)
    elif high & 0xF2 == 0x50:
        write_f7(high)
    elif high & 0xF0 == 0xD0:
        write_f16(high)
    else:
        print("\tunimplemented!(\"{:04x}\", instr);")

    print("}\n")

for high in range(0x0, 0x100):
    write_instruction(high)

print("const THUMB_INSTRUCTIONS: [fn(&mut ARM7TDMI, &mut Interconnect, u16); 256] = ")
print("[" + ", ".join(["thumb_%02x" % i for i in range(0x0, 0x100)]) + "];")
