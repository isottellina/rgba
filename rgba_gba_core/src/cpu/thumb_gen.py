# thumb_gen.py --- 
# 
# Filename: thumb_gen.py
# Author: Louise <louise>
# Created: Tue Jan 16 19:57:01 2018 (+0100)
# Last-Updated: Sun Feb 25 19:02:48 2018 (+0100)
#           By: Louise <louise>
#
class Generator:
    def __init__(self, size):
        self.current = 0
        self.array = [None] * size
    def set_current(self, current):
        self.current = current
        self.array[current] = ""
    def write(self, s, end = "\n", indent = 1):
        self.array[self.current] += "\t" * indent
        self.array[self.current] += s
        self.array[self.current] += end
    def optimize(self):
        for n1, el in enumerate(self.array):
            if type(el) == str:
                for n2, i in enumerate(self.array[n1 + 1:], start=(n1 + 1)):
                    if i == el: self.array[n2] = n1
    def print_function(self, fn):
        if type(self.array[fn]) == str:
            print("#[allow(unreachable_code, unused_variables, unused_assignments)]")
            print("fn thumb_%02x(_cpu: &mut ARM7TDMI, _io: &mut Interconnect, instr: u16) {" % fn)
            print(self.array[fn], end = "")
            print("}\n")
    def print_functions(self):
        for fn in range(len(self.array)):
            self.print_function(fn)
    def print_array(self):
        print("const THUMB_INSTRUCTIONS: [fn(&mut ARM7TDMI, &mut Interconnect, u16); 256] = [", end = "\n\t")
        for n, i in enumerate(self.array):
            if not n % 16: print("// 0x%x" % n, end = "\n\t")
            
            if type(i) == str:
                print("thumb_%02x, " % n, end = "" if (n + 1) % 4 else "\n\t")
            else:
                print("thumb_%02x, " % i, end = "" if (n + 1) % 4 else "\n\t")
        print("\r];")

def write_f1(g, high):
    op = (high >> 3)
    g.write("let rd = instr & 7;")
    g.write("let rs = _cpu.registers[((instr >> 3) & 7) as usize];")
    g.write("let shift = (instr >> 6) & 0x1f;")

    if op == 0:
        g.write("if shift != 0 { _cpu.carry = ((rs << (shift - 1)) & 0x80000000) != 0; }")
        g.write("let res = rs << shift;")
    elif op == 1:
        g.write("if shift != 0 { _cpu.carry = ((rs >> (shift - 1)) & 1) != 0; }")
        g.write("let res = rs >> shift;")
    elif op == 2:
        g.write("if shift != 0 { _cpu.carry = (((rs as i32) >> (shift - 1)) & 1) != 0; }")
        g.write("let res = ((rs as i32) >> shift) as u32;")

    g.write("_cpu.registers[rd as usize] = res;")
    g.write("_cpu.zero = res == 0;")
    g.write("_cpu.sign = (res as i32) < 0;")

def write_f2(g, high):
    imm = (high & 0x04) != 0
    op = (high & 0x02) != 0

    g.write("let rd = instr & 7;")
    g.write("let op1 = _cpu.registers[((instr >> 3) & 7) as usize];")

    if imm:
        g.write("let op2 = ((instr >> 6) & 7) as u32;")
    else:
        g.write("let op2 = _cpu.registers[((instr >> 6) & 7) as usize];")
    
    if op: # SUB
        g.write("let res = op1.wrapping_sub(op2);")
        g.write("_cpu.carry = op1 >= op2;")
        g.write("_cpu.overflow = (op1 ^ op2) & (op1 ^ op2) & 0x80000000 != 0;")
    else: # ADD
        g.write("let res = op1.wrapping_add(op2);")
        g.write("_cpu.carry = res < op1;")
        g.write("_cpu.overflow = !(op1 ^ op2) & (op1 ^ res) & 0x80000000 != 0;")

    g.write("_cpu.zero = res == 0;")
    g.write("_cpu.sign = (res as i32) < 0;")
    g.write("_cpu.registers[rd as usize] = res")
    
def write_f3(g, high):
    op = (high >> 3) & 0x3;

    g.write("let rd = (instr >> 8) & 7;")
    g.write("let val = (instr & 0xFF) as u32;")
    if op == 0: # MOV
        g.write("let res = val;")
    elif op == 1 or op == 3: # CMP and SUB
        g.write("let op1 = _cpu.registers[rd as usize];")
        g.write("let res = op1.wrapping_sub(val as u32);")
        g.write("_cpu.carry = op1 >= val;")
        g.write("_cpu.overflow = (op1 ^ val) & (op1 ^ res) & 0x80000000 != 0;")
    elif op == 2: # ADD
        g.write("let op1 = _cpu.registers[rd as usize];")
        g.write("let res = op1.wrapping_add(val as u32);")
        g.write("_cpu.carry = res < op1;")
        g.write("_cpu.overflow = !(op1 ^ val) & (op1 ^ res) & 0x80000000 != 0;")

    g.write("_cpu.zero = res == 0;")
    g.write("_cpu.sign = (res as i32) < 0;")
    if op != 1:
        g.write("_cpu.registers[rd as usize] = res;")

def write_f4(g, high):
    g.write("let op = (instr >> 6) & 0xf;")
    g.write("let function = THUMB_ALU[op as usize];")
    g.write("function(_cpu, instr);")
        
def write_f5(g, high):
    op = high & 3

    g.write("let rd = (instr & 7) | ((instr & 0x80) >> 4);")
    g.write("let rs = _cpu.get_register(((instr >> 3) & 0xF) as usize);")
    
    if op == 0: # ADD
        g.write("let op1 = _cpu.get_register(rd as usize);")
        g.write("let res = op1.wrapping_add(rs);")
        g.write("_cpu.set_register(rd as usize, res);")
        g.write("if rd == 15 { _cpu.registers[15] &= 0xFFFFFFFE; _cpu.branch(_io); }")
    elif op == 1: # CMP
        g.write("let op1 = _cpu.get_register(rd as usize);")
        g.write("let res = op1.wrapping_sub(rs);")
        g.write("_cpu.sign = (res as i32) < 0;")
        g.write("_cpu.zero = res == 0;")
        g.write("_cpu.carry = op1 >= res;")
        g.write("_cpu.overflow = (op1 ^ rs) & (op1 ^ res) & 0x80000000 != 0;")
        g.write("if rd == 15 { _cpu.registers[15] &= 0xFFFFFFFE; _cpu.branch(_io); }")
    elif op == 2: # MOV
        g.write("_cpu.set_register(rd as usize, rs);")
        g.write("if rd == 15 { _cpu.registers[15] &= 0xFFFFFFFE; _cpu.branch(_io); }")
    elif op == 3: # BX
        g.write("if rs & 1 == 0 {")
        g.write("_cpu.state = CpuState::ARM; _cpu.registers[15] = rs & 0xFFFFFFFC;", indent = 2)
        g.write("} else {")
        g.write("_cpu.registers[15] = rs & 0xFFFFFFFE;", indent = 2)
        g.write("}")
        g.write("_cpu.branch(_io);")
        
def write_f6(g, high):
    rd = high & 7
    
    g.write("let off = ((instr & 0xFF) as u32) << 2;")
    g.write("let addr = (_cpu.registers[15] & 0xFFFFFFFC) + off;")
    g.write("_cpu.registers[%s] = _cpu.read_u32(_io, addr as usize);" % rd)

def write_f7(g, high):
    load = high & 0x08 != 0
    byte = high & 0x04 != 0

    g.write("let rb = (instr >> 3) & 7;")
    g.write("let ro = (instr >> 6) & 7;")
    g.write("let rd = instr & 7;")
    g.write("let addr = _cpu.registers[rb as usize].wrapping_add(_cpu.registers[ro as usize]);")
    if load:
        if byte:
            g.write("let val = _cpu.read_u8(_io, addr as usize) as u32;")
        else:
            g.write("let val = _cpu.read_u32(_io, addr as usize) as u32;")
        g.write("_cpu.registers[rd as usize] = val;")
    else:
        if byte:
            g.write("_cpu.write_u8(_io, addr as usize, _cpu.registers[rd as usize] as u8);")
        else:
            g.write("_cpu.write_u32(_io, addr as usize, _cpu.registers[rd as usize]);")

def write_f8(g, high):
    op = (high >> 2) & 3

    g.write("let rb = _cpu.registers[((instr >> 3) & 7) as usize];")
    g.write("let ro = _cpu.registers[((instr >> 6) & 7) as usize];")
    g.write("let addr = rb.wrapping_add(ro) as usize;")
    g.write("let rd = instr & 7;")

    if op == 0: # STRH
        g.write("let val = _cpu.registers[rd as usize] as u16;")
        g.write("_cpu.write_u16(_io, addr, val);")
    elif op == 1: # LDSB
        g.write("let val = (_cpu.read_u8(_io, addr) as i8) as i32;")
        g.write("_cpu.registers[rd as usize] = val as u32;")
    elif op == 2: # LDRH
        g.write("let val = _cpu.read_u16(_io, addr) as u32;")
        g.write("_cpu.registers[rd as usize] = val;")
    elif op == 3: # LDSH
        g.write("let val = (_cpu.read_u16(_io, addr) as i16) as i32;")
        g.write("_cpu.registers[rd as usize] = val as u32;")
            
def write_f9(g, high):
    op = (high >> 3) & 3

    g.write("let offset = ((instr >> 6) & 0x1F) as u32;")
    g.write("let rb = _cpu.registers[((instr >> 3) & 7) as usize];")
    g.write("let rd = instr & 7;")
    
    if op == 0: # STR
        g.write("let addr = rb + (offset << 2);")
        g.write("let val = _cpu.registers[rd as usize];")
        g.write("_cpu.write_u32(_io, addr as usize, val);")
    elif op == 1: # LDR
        g.write("let addr = rb + (offset << 2);")
        g.write("_cpu.registers[rd as usize] = _cpu.read_u32(_io, addr as usize);")
    elif op == 2: # STRB
        g.write("let addr = rb + offset;")
        g.write("let val = _cpu.registers[rd as usize];")
        g.write("_cpu.write_u8(_io, addr as usize, val as u8);")
    elif op == 3: # LDRB
        g.write("let addr = rb + offset;")
        g.write("_cpu.registers[rd as usize] = _cpu.read_u8(_io, addr as usize) as u32;")
            
def write_f10(g, high):
    g.write("let rb = _cpu.registers[((instr >> 3) & 7) as usize];")
    g.write("let rd = instr & 7;")
    g.write("let offset = ((instr >> 6) & 0x1f) << 1;")
    g.write("let addr = rb + (offset as u32);")
    
    if high & 0x08 == 0: # STRH
        g.write("let val = _cpu.registers[rd as usize];")
        g.write("_cpu.write_u16(_io, addr as usize, val as u16);")
    else:
        g.write("_cpu.registers[rd as usize] = _cpu.read_u16(_io, addr as usize) as u32;")
            
def write_f11(g, high):
    load = high & 0x08 != 0
    rd = high & 0x7
    g.write("let off = ((instr & 0xFF) as usize) << 2;")
    g.write("let sp = _cpu.get_register(13) as usize;")
    g.write("let addr = off.wrapping_add(sp);")
    
    if load:
        g.write("_cpu.registers[%s] = _cpu.read_u32(_io, addr);" % rd)
    else:
        g.write("let reg = _cpu.registers[%s];" % rd)
        g.write("_cpu.write_u32(_io, addr, reg)")

def write_f12(g, high):
    rd = high & 7

    g.write("let val = ((instr & 0xFF) as u32) << 2;")
    
    if high & 0x08 == 0:
        g.write("_cpu.registers[%d] = _cpu.registers[15].wrapping_add(val);" % rd)
    else:
        g.write("let sp = _cpu.get_register(13);")
        g.write("_cpu.registers[%d] = sp.wrapping_add(val);" % rd)

def write_f13(g, high):
    g.write("let off = ((instr & 0x7f) as u32) << 2;")
    g.write("let sp = _cpu.get_register(13);")
    g.write("if instr & 0x80 == 0 {")
    g.write("_cpu.set_register(13, sp + off);", indent = 2)
    g.write("} else {")
    g.write("_cpu.set_register(13, sp - off);", indent = 2)
    g.write("}")
            
def write_f14(g, high):
    pop = high & 0x08 != 0

    if pop:
        g.write("let mut sp = _cpu.get_register(13) as usize;")
    else:
        g.write("let count = (instr & 0x1FF).count_ones();")
        g.write("let mut sp = (_cpu.get_register(13) - (count << 2)) as usize;")
        g.write("_cpu.set_register(13, sp as u32);")
    
    for i in range(9):
        g.write("if instr & (1 << %s) != 0 {" % i)
        
        if i == 8:
            i = 15 if pop else 14
        
        if pop:
            if i == 15:
                g.write("_cpu.registers[15] = _cpu.read_u32(_io, sp) & 0xFFFFFFFE;", indent = 2)
                g.write("_cpu.branch(_io);", indent = 2)
            else:
                g.write("_cpu.registers[%s] = _cpu.read_u32(_io, sp);" % i, indent = 2)
        else:
            if i == 14:
                g.write("let reg = _cpu.get_register(14);", indent = 2)
            else:
                g.write("let reg = _cpu.registers[%s];" % i, indent = 2)
            g.write("_cpu.write_u32(_io, sp, reg);", indent = 2)

        g.write("sp += 4;", indent = 2)
        g.write("}")
        
    if pop:
        g.write("_cpu.set_register(13, sp as u32);")

def write_f15(g, high):
    rb = high & 7
    pop = high & 0x08 != 0

    # Empty list
    g.write('if instr & 0xFF == 0 { unimplemented!("Empty list in LDM/STM"); }')
    g.write("let mut sp = _cpu.registers[%d] as usize;" % rb)
    
    for i in range(8):
        g.write("if instr & (1 << %s) != 0 {" % i)
        
        if pop:
            g.write("_cpu.registers[%s] = _cpu.read_u32(_io, sp);" % i, indent = 2)
        else:
            g.write("let reg = _cpu.registers[%s];" % i, indent = 2)
            g.write("_cpu.write_u32(_io, sp, reg);", indent = 2)

        g.write("sp += 4;", indent = 2)
        g.write("}")
        
    g.write("_cpu.registers[%d] = sp as u32;" % rb)
            
def write_f16(g, high):
    conditions = [
        "_cpu.zero", "!_cpu.zero",
        "_cpu.carry", "!_cpu.carry",
        "_cpu.sign", "!_cpu.sign",
        "_cpu.overflow", "!_cpu.overflow",
        "_cpu.carry && !_cpu.zero", "!_cpu.carry || _cpu.zero",
        "_cpu.sign == _cpu.overflow", "_cpu.sign != _cpu.overflow",
        "!_cpu.zero && (_cpu.sign == _cpu.overflow)", "_cpu.zero || (_cpu.sign != _cpu.overflow)",
        "true", "false"
    ]
    
    g.write("let off = (((instr & 0xFF) as i8) as i32) << 1;")
    g.write("if %s {" % conditions[high & 0xF])
    g.write("_cpu.registers[15] = (_cpu.registers[15] as i32).wrapping_add(off) as u32;", indent = 2)
    g.write("_cpu.branch(_io);", indent = 2)
    g.write("}")

def write_f17(g, high):
    g.write("_cpu.raise_swi();")
    
def write_f18(g, high):
    g.write("let offset = (((instr << 5) as i16) >> 4) as i32;")
    g.write("_cpu.registers[15] = ((_cpu.registers[15] as i32) + offset) as u32;")
    g.write("_cpu.branch(_io);")
    
def write_f19(g, high):
    stage = (high & 0x08) >> 3

    if stage == 0:
        g.write("let pc = _cpu.registers[15] as i32;")
        g.write("let new_lr = pc + (((((instr & 0x7FF) as u32) << 21) as i32) >> 9);")
        g.write("_cpu.set_register(14, new_lr as u32);")
    else:
        g.write("let tmp = (_cpu.registers[15] - 2) | 1;")
        g.write("_cpu.registers[15] = _cpu.get_register(14).wrapping_add(((instr & 0x7FF) as u32) << 1);")
        g.write("_cpu.set_register(14, tmp);")
        g.write("_cpu.branch(_io);")
    
def write_instruction(g, high):
    g.set_current(high)
    
    if high & 0xF8 == 0x18:
        write_f2(g, high)
    elif high & 0xE0 == 0x00:
        write_f1(g, high)
    elif high & 0xE0 == 0x20:
        write_f3(g, high)
    elif high & 0xFC == 0x40:
        write_f4(g, high)
    elif high & 0xFC == 0x44:
        write_f5(g, high)
    elif high & 0xF8 == 0x48:
        write_f6(g, high)
    elif high & 0xF2 == 0x50:
        write_f7(g, high)
    elif high & 0xF2 == 0x52:
        write_f8(g, high)
    elif high & 0xE0 == 0x60:
        write_f9(g, high)
    elif high & 0xF0 == 0x80:
        write_f10(g, high)
    elif high & 0xF0 == 0x90:
        write_f11(g, high)
    elif high & 0xF0 == 0xA0:
        write_f12(g, high)
    elif high & 0xFF == 0xB0:
        write_f13(g, high)
    elif high & 0xF6 == 0xB4:
        write_f14(g, high)
    elif high & 0xF0 == 0xC0:
        write_f15(g, high)
    elif high & 0xF0 == 0xD0 and high & 0xF != 0xF:
        write_f16(g, high)
    elif high == 0xDF:
        write_f17(g, high)
    elif high & 0xF8 == 0xE0:
        write_f18(g, high)
    elif high & 0xF0 == 0xF0:
        write_f19(g, high)
    else:
        g.write("unimplemented!(\"{:04x} {:08x}\", instr, _cpu.pc);")

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
        print("\tlet res = op1 ^ rs;");
    elif op == 2: # LSL
        print("\tlet shift = rs & 0xFF;")
        print("\tlet res = op1 << shift;")
        print("\tif shift != 0 { _cpu.carry = ((op1 << (shift - 1)) & 0x80000000) != 0; }")
    elif op == 3: # LSR
        print("\tlet shift = rs & 0xFF;")
        print("\tlet res = op1 >> shift;")
        print("\tif shift != 0 { _cpu.carry = ((op1 >> (shift - 1)) & 1) != 0; }")
    elif op == 4: # ASR
        print("\tlet shift = rs & 0xFF;")
        print("\tlet res = ((op1 as i32) >> shift) as u32;")
        print("\tif shift != 0 { _cpu.carry = (((op1 as i32) >> (shift - 1)) & 1) != 0; }")
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

for opcode in range(0x10):
    write_alu(opcode)

print("const THUMB_ALU: [fn(&mut ARM7TDMI, u16); 16] = ", end = "")
print("[" + ", ".join(["thumb_alu_%01x" % i for i in range(0x0, 0x10)]) + "];")

g = Generator(0x100)

for high in range(0x100):
    write_instruction(g, high)

g.optimize()
g.print_functions()
g.print_array()
