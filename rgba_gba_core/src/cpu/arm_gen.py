# arm_gen.py --- 
# 
# Filename: arm_gen.py
# Author: Louise <louise>
# Created: Sat Jan 13 17:25:38 2018 (+0100)
# Last-Updated: Thu Nov  5 23:15:30 2020 (+0100)
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
            print("fn arm_%03x(_cpu: &mut ARM7TDMI, _io: &mut Interconnect, instr: u32) {" % fn)
            print(self.array[fn], end = "")
            print("}\n")
    def print_functions(self):
        for fn in range(len(self.array)):
            self.print_function(fn)
    def print_array(self):
        print("const ARM_INSTRUCTIONS: [fn(&mut ARM7TDMI, &mut Interconnect, u32); 4096] = [", end = "\n\t")
        for n, i in enumerate(self.array):
            if not n % 16: print("// 0x%x" % n, end = "\n\t")
            
            if type(i) == str:
                print("arm_%03x, " % n, end = "" if (n + 1) % 4 else "\n\t")
            else:
                print("arm_%03x, " % i, end = "" if (n + 1) % 4 else "\n\t")
        print("\r];")

def write_branch(g, high, low):
    link = (high & 0x10) != 0
    g.write("let offset = ((instr << 8) as i32) >> 6;")
    g.write("let old_pc = _cpu.get_register(15);")
    
    if link:
        g.write("_cpu.set_register(14, old_pc - 4);")

    g.write("let new_pc = ((old_pc as i32) + offset) as u32;")
    g.write("_cpu.set_register(15, new_pc);")
    g.write("_cpu.branch(_io);")

def write_branch_exchange(g):
    g.write("let dest = _cpu.get_register((instr & 0xF) as usize);")
    g.write("if dest & 1 != 0 { _cpu.state = CpuState::Thumb; }")
    g.write("_cpu.registers[15] = dest & 0xFFFFFFFE;")
    g.write("_cpu.branch(_io);")

def write_swi(g):
    g.write("_cpu.raise_swi();")
    
def write_op2_imm(g, high, low):
    s = (high & 0x01) != 0
    
    g.write("let imm = instr & 0xFF;")
    g.write("let rot = (instr & 0xF00) >> 7;")
    g.write("let op2 = imm.rotate_right(rot);")
    if s:
        g.write("if rot != 0 { _cpu.carry = (op2 >> 31) != 0; }")

def write_op2_reg(g, low, s):
    shift = (low & 0x6) >> 1
    
    if (low & 1) == 0: # By immediate
        g.write("let rm = _cpu.get_register((instr & 0xF) as usize);")
        g.write("let amount = (instr >> 7) & 0x1f;")
        if shift == 0:
            g.write("let op2 = if amount == 0 { rm } else {", end = "")
            if s: g.write(" let tmp = rm << (amount - 1); _cpu.carry = (tmp >> 31) != 0; tmp << 1 };")
            else: g.write(" rm << amount };")
        elif shift == 1:
            g.write("let op2 = if amount == 0 { ", end = "")
            if s:
                g.write("_cpu.carry = (rm & 0x80000000) != 0; 0 } else { let tmp = rm >> (amount - 1); ",end="")
                g.write("_cpu.carry = (tmp & 1) != 0; tmp >> 1 };")
            else:
                g.write("0 } else { rm >> amount };")
        elif shift == 2:
            g.write("let op2 = if amount == 0 { ", end = "")
            if s:
                g.write("_cpu.carry = (rm & 0x80000000) != 0; ((rm as i32) >> 31) as u32 } else { ", end="")
                g.write("let tmp = ((rm as i32) >> (amount - 1)) as u32; _cpu.carry = tmp & 1 != 0; ", end="")
                g.write("((tmp as i32) >> 1) as u32 };")
            else:
                g.write("((rm as i32) >> 31) as u32 } else { ((rm as i32) >> amount) as u32 };")
        elif shift == 3:
            g.write("let op2 = if amount == 0 { ", end="")
            if s:
                g.write("let tmp = (rm >> 1) | ((_cpu.carry as u32) << 31);  _cpu.carry = (rm & 1) != 0; ")
                g.write("tmp } else { let tmp = rm.rotate_right(amount); _cpu.carry = (tmp >> 31) != 0; ")
                g.write("tmp };")
            else:
                g.write("(rm >> 1) | ((_cpu.carry as u32) << 31) } else { rm.rotate_right(amount) };")
        
    else: # By register
        # Shift by register is longer and PC is shifted more
        g.write("let rm = if (instr & 0xF) != 15 {")
        g.write("_cpu.get_register((instr & 0xF) as usize)", indent = 2)
        g.write("} else {")
        g.write("_cpu.get_register(15) + 4")
        g.write("};")
        
        g.write("let amount = _cpu.get_register(((instr >> 8) & 0xF) as usize) & 0xFF;")
        g.write("let op2 = if amount != 0 {")
        if shift == 0:
            if s:
                g.write("if amount < 32 {", indent = 2)
                g.write("let tmp = rm << (amount - 1); _cpu.carry = (tmp >> 31) != 0;", indent = 3)
                g.write("tmp << 1", indent = 3)
                g.write("} else if amount == 32 {", indent = 2)
                g.write("_cpu.carry = rm & 1 != 0; 0", indent = 3)
                g.write("} else {", indent = 2)
                g.write("_cpu.carry = false; 0", indent = 3)
                g.write("}")
            else:
                g.write("if amount < 32 { rm << amount } else { 0 }", indent = 2)
        elif shift == 1:
            if s:
                g.write("if amount < 32 {", indent = 2)
                g.write("let tmp = rm >> (amount - 1); _cpu.carry = (tmp & 1) != 0;", indent = 3)
                g.write("tmp >> 1", indent = 3)
                g.write("} else if amount == 32 {", indent = 2)
                g.write("_cpu.carry = rm & 0x80000000 != 0; 0", indent = 3)
                g.write("} else {", indent = 2)
                g.write("_cpu.carry = false; 0", indent = 3)
                g.write("}", indent = 2)
            else:
                g.write("if amount < 32 { rm >> amount } else { 0 }", indent = 2)
        elif shift == 2:
            if s:
                g.write("if amount < 32 {", indent = 2)
                g.write("let tmp = ((rm as i32) >> (amount - 1)) as u32; _cpu.carry = tmp & 1 != 0;",indent=3)
                g.write("((tmp as i32) >> 1) as u32", indent = 3)
                g.write("} else {", indent = 2)
                g.write("_cpu.carry = rm & 0x80000000 != 0; ((rm as i32) >> 31) as u32", indent = 3)
                g.write("}", indent = 2)
            else:
                g.write(
                    "if amount < 32 { ((rm as i32) >> amount) as u32 } else { ((rm as i32) >> 31) as u32 }",
                    indent = 2
                )
        elif shift == 3:
            g.write("let op2 = rm.rotate_right(amount);", indent = 2)
            if s:
                g.write("_cpu.carry = (op2 >> 31) != 0;", indent = 2)
            g.write("op2")
        g.write("} else {")
        g.write("rm", indent = 2)
        g.write("};")
    
    
def write_alu(g, high, low):
    op = (high & 0x1e) >> 1
    s = (high & 0x01) != 0
    imm = (high & 0x20) != 0

    if op == 5 or op == 6 or op == 7:
        g.write("let cf = _cpu.carry;")
    
    # Geration op2 code
    if imm:
        write_op2_imm(g, high, low)
    else:
        write_op2_reg(g, low, s)
        
    test = (op & 0xc == 0x8)
        
    if (op & 0xc == 0x8) and not s:
        g.write("panic!(\"Gerating bad ALU instruction ({:08x}))\", instr);")
        return
    
    if op != 13 and op != 15:
        g.write("let rn = _cpu.get_register(((instr >> 16) & 0xF) as usize);")

    if not test or s:
        g.write("let rd = (instr >> 12) & 0xF;")
    
    if op == 8 or op == 0: # AND, TST
        g.write("let res = rn & op2;")
    elif op == 9 or op == 1: # EOR, TEQ
        g.write("let res = rn ^ op2;");
    elif op == 10 or op == 2: # SUB, CMP
        g.write("let res = rn.wrapping_sub(op2);")
        if s:
            g.write("if rd != 15 {")
            g.write("_cpu.carry = rn >= op2;", indent = 2)
            g.write("_cpu.overflow = (rn ^ op2) & (rn ^ res) & 0x80000000 != 0;", indent = 2)
            g.write("}")
    elif op == 3: # RSB
        g.write("let res = op2.wrapping_sub(rn);")
        if s:
            g.write("if rd != 15 {")
            g.write("_cpu.carry = op2 >= rn;", indent = 2)
            g.write("_cpu.overflow = (rn ^ op2) & (op2 ^ res) & 0x80000000 != 0;", indent = 2)
            g.write("}")
    elif op == 11 or op == 4: # ADD, CMN
        g.write("\tlet res = rn.wrapping_add(op2);")
        if s:
            g.write("if rd != 15 {")
            g.write("_cpu.carry = rn > res;", indent = 2)
            g.write("_cpu.overflow = !(rn ^ op2) & (rn ^ res) & 0x80000000 != 0;", indent = 2)
            g.write("}")
    elif op == 5: # ADC
        g.write("let res = rn.wrapping_add(op2).wrapping_add(cf as u32);")
        if s:
            g.write("if rd != 15 {")
            g.write("_cpu.carry = if cf { rn >= res } else { rn > res };", indent = 2)
            g.write("_cpu.overflow = !(rn ^ op2) & (rn ^ res) & 0x80000000 != 0;", indent = 2)
            g.write("}")
    elif op == 6: # SBC
        g.write("let res = rn.wrapping_sub(op2).wrapping_sub(!cf as u32);")
        if s:
            g.write("if rd != 15 {")
            g.write("_cpu.carry = if cf { rn >= op2 } else { rn > op2 };", indent = 2)
            g.write("_cpu.overflow = (rn ^ op2) & (rn ^ res) & 0x80000000 != 0;", indent = 2)
            g.write("}")
    elif op == 7: # RSC
        g.write("let res = op2.wrapping_sub(rn).wrapping_sub(!cf as u32);")
        if s:
            g.write("if rd != 15 {")
            g.write("_cpu.carry = if cf { op2 > rn } else { op2 >= rn };", indent = 2)
            g.write("_cpu.overflow = (rn ^ op2) & (op2 ^ res) & 0x80000000 != 0;", indent = 2)
            g.write("}")
    elif op == 12: # ORR
        g.write("let res = rn | op2;")
    elif op == 13: # MOV
        g.write("let res = op2;")
    elif op == 14: # BIC
        g.write("let res = rn & !op2;");
    elif op == 15: # MVN
        g.write("let res = !op2;");
    else:
        g.write("let res = 0;")
        g.write("unimplemented!(\"ALU instruction not implemented : {:08x}\", instr);")

    if s:
        g.write("if rd != 15 { _cpu.sign = (res as i32) < 0; _cpu.zero = res == 0; }")
    if not test:
        g.write("_cpu.set_register(rd as usize, res);")
        g.write("if rd == 15 {")
        if s: g.write("let spsr = _cpu.spsr(); _cpu.set_cpsr(spsr);", indent = 2)
        g.write("_cpu.branch(_io);", indent = 2)
        g.write("}")

def write_swp(g, high):
    byte = (high & 0x04) != 0

    g.write("let rn = _cpu.get_register(((instr >> 16) & 0xF) as usize);")
    g.write("let rm = _cpu.get_register((instr & 0xF) as usize);")
    g.write("let rd = (instr >> 12) & 0xF;")

    if byte:
        g.write("let tmp = _cpu.read_u8(_io, rn as usize);")
        g.write("_cpu.write_u8(_io, rn as usize, rm as u8);")
        g.write("_cpu.set_register(rd as usize, tmp as u32);")
    else:
        g.write("let tmp = _cpu.read_u32(_io, (rn & 0xFFFFFFFC) as usize).rotate_right((rn & 3) << 3);")
        g.write("_cpu.write_u32(_io, (rn & 0xFFFFFFFC) as usize, rm);")
        g.write("_cpu.set_register(rd as usize, tmp);")
        
def write_psr(g, high, low):
    reg = "cpsr" if (high & 0x04 == 0) else "spsr"
    
    if high & 0x02 == 0x02:
        if high & 0x20 != 0: # Immediate value
            g.write("let val = (instr & 0xFF).rotate_right((instr & 0xF00) >> 7);")
        else: # Register
            g.write("let val = _cpu.get_register((instr & 0xF) as usize);")

        g.write("if instr & 0x000F0000 == 0x00080000 { _cpu.set_%s_flg(val); } else { _cpu.set_%s(val); }"
              % (reg, reg))
    else:
        g.write("let val = _cpu.%s();" % reg)
        g.write("let rd = (instr & 0xF000) >> 12;")
        g.write("_cpu.set_register(rd as usize, val);")
        
def write_sdt(g, high, low):
    pre = high & 0x10 != 0
    wb = high & 0x02 != 0
    
    g.write("let rd = (instr >> 12) & 0xF;")
    g.write("let rn = (instr >> 16) & 0xF;")
    
    if high & 0x20 == 0:
        g.write("let off = instr & 0xFFF;")
    else:
        write_op2_reg(g, low, False)
        g.write("let off = op2;")

    if pre:
        if high & 0x08 != 0:
            g.write("let addr = _cpu.get_register(rn as usize).wrapping_add(off);")
        else:
            g.write("let addr = _cpu.get_register(rn as usize).wrapping_sub(off);")
    else:
        g.write("let addr = _cpu.get_register(rn as usize);")
        
    if high & 0x01 == 0:
        g.write("let val = _cpu.get_register(rd as usize);")
        if high & 0x04 != 0: # Byte quantity
            g.write("_cpu.write_u8(_io, addr as usize, val as u8);")
        else: # Word quantity
            g.write("_cpu.write_u32(_io, addr as usize, val);")
    else:
        if high & 0x04 != 0: # Byte quantity
            g.write("let mut res = _cpu.read_u8(_io, addr as usize) as u32;")
        else: # Word quantity
            g.write("let mut res = if addr & 3 != 0 { ")
            g.write(
                "let rot = (addr & 3) << 3; _cpu.read_u32(_io, (addr & !3) as usize).rotate_right(rot)",
                indent = 2
            )
            g.write("} else {")
            g.write("_cpu.read_u32(_io, addr as usize)", indent = 2)
            g.write("};")
        g.write("_cpu.set_register(rd as usize, res);")
        g.write("if rd == 15 {")
        g.write("if res & 1 != 0 { _cpu.state = CpuState::Thumb; res &= !1; } else { res &= !3; }",indent = 2)
        g.write("_cpu.branch(_io);", indent = 2)
        g.write("}")

    g.write("if rn != rd {")
    if not pre:
        if high & 0x08 != 0:
            g.write("let wb = addr.wrapping_add(off);", indent = 2)
        else:
            g.write("let wb = addr.wrapping_sub(off);", indent = 2)
        if wb:
            g.write('unimplemented!("User-mode transfer ({:08x})", instr);', indent = 2)
            g.write("}")
            return
        g.write("_cpu.set_register(rn as usize, wb);", indent = 2)
    elif wb:
        g.write("_cpu.set_register(rn as usize, addr);", indent = 2)

    g.write("}")

def write_bdt(g, high, low):
    pre = high & 0x10 != 0
    up = high & 0x08 != 0
    psr = high & 0x04 != 0
    wb = high & 0x02 != 0
    load = high & 0x01 != 0

    g.write('let rn = (instr >> 16) & 0xF;')
    g.write('let list = instr & 0xFFFF;')

    if wb: # Determine WB behavior
        g.write('let wbmode = if list & (1 << rn) != 0 {')
        if load:
            g.write("0", indent = 2)
        else:
            g.write("if list & ((1 << rn) - 1) == 0 { 2 } else { 1 }", indent = 2)
        g.write("} else {")
        g.write("1", indent = 2)
        g.write("};")
        
        if not load:
            g.write("let oldrn = _cpu.get_register(rn as usize);")

    if not up:
        g.write("let mut addr = (_cpu.get_register(rn as usize) - (list.count_ones() << 2)) as usize;")
        if wb: g.write("let lowestrn = addr as u32;")
        pre = not pre
    else:
        g.write("let mut addr = _cpu.get_register(rn as usize) as usize;")

    if psr:
        if load:
            g.write("let userbnk = (instr & 0x8000) == 0;")
        else:
            g.write("let userbnk = true;")
        g.write("let oldmode = _cpu.mode;")
        g.write("if userbnk { _cpu.mode = CpuMode::User; }")

    for i in range(16):
        g.write("if list & 0x%04x != 0 {" % (1 << i))
        if pre: g.write("addr += 4;", indent = 2)

        if load:
            g.write("let val = _cpu.read_u32(_io, addr);", indent = 2)
            g.write("_cpu.set_register(%d, val);" % i, indent = 2)
            if i == 15:
                if psr:
                    g.write("let spsr = _cpu.spsr();", indent = 2)
                    g.write("_cpu.set_cpsr(spsr);", indent = 2)
                g.write("_cpu.branch(_io);", indent = 2)
        else:
            if wb:
                g.write("let val = if (rn == %d) && (wbmode == 1) {" % i, indent = 2)
                if up:
                    g.write("oldrn + (list.count_ones() << 2)", indent = 3)
                else:
                    g.write("lowestrn", indent = 3)
                g.write("} else {", indent = 2)
                g.write("_cpu.get_register(%d)" % i, indent = 3)
                g.write("};", indent = 2)
            else:
                g.write("let val = _cpu.get_register(%d);" % i)
            g.write("_cpu.write_u32(_io, addr, val);", indent = 3)
        
        if not pre: g.write("addr += 4;", indent = 2)
        g.write("}")

    if wb:
        g.write("if wbmode != 0 {")
        if up:
            g.write("_cpu.set_register(rn as usize, addr as u32);", indent = 2)
        else:
            g.write("_cpu.set_register(rn as usize, lowestrn);", indent = 2)
        g.write("}")

    if psr:
        g.write("if userbnk { _cpu.mode = oldmode; }")

def write_mul(g, high):
    a = ((high >> 1) & 1) != 0
    s = (high & 1) != 0

    g.write("let rd = (instr >> 16) & 0xF;")
    g.write("let rm = _cpu.get_register((instr & 0xF) as usize);")
    g.write("let rs = _cpu.get_register(((instr >> 8) & 0xF) as usize);")

    g.write("if (rs & 0xFFFFFF00 == 0) || (!rs & 0xFFFFFF00 == 0) {")
    g.write("_io.delay(%d);" % (2 if a else 1), indent = 2)
    g.write("} else if (rs & 0xFFFF0000 == 0) || (!rs & 0xFFFF0000 == 0) {")
    g.write("_io.delay(%d);" % (3 if a else 2), indent = 2)
    g.write("} else if (rs & 0xFF000000 == 0) || (!rs & 0xFF000000 == 0) {")
    g.write("_io.delay(%d);" % (4 if a else 3), indent = 2)
    g.write("} else {")
    g.write("_io.delay(%d);" % (5 if a else 4), indent = 2)
    g.write("}")
    
    if a: # MLA
        g.write("let rn = _cpu.get_register(((instr >> 12) & 0xF) as usize);")
        g.write("let val = rm.wrapping_mul(rs).wrapping_add(rn);")
    else:
        g.write("let val = rm.wrapping_mul(rs);")

    if s:
        g.write("_cpu.zero = val == 0;")
        g.write("_cpu.sign = (val as i32) < 0;")
        g.write("_cpu.carry = true;")

    g.write("_cpu.set_register(rd as usize, val);")

def write_mull(g, high):
    u = ((high >> 2) & 1) != 0
    a = ((high >> 1) & 1) != 0
    s = (high & 1) != 0

    g.write("let rd_hi = ((instr >> 16) & 0xF) as usize;")
    g.write("let rd_lo = ((instr >> 12) & 0xF) as usize;")

    if u:
        g.write("let rm = ((_cpu.get_register((instr & 0xF) as usize) as i32) as i64) as u64;")
        g.write("let rs = ((_cpu.get_register(((instr >> 8) & 0xF) as usize) as i32) as i64) as u64;")
    else:
        g.write("let rm = _cpu.get_register((instr & 0xF) as usize) as u64;")
        g.write("let rs = _cpu.get_register(((instr >> 8) & 0xF) as usize) as u64;")

    g.write("if (rs & 0xFFFFFF00 == 0) || (!rs & 0xFFFFFF00 == 0) {")
    g.write("_io.delay(%d);" % (3 if a else 2), indent = 2)
    g.write("} else if (rs & 0xFFFF0000 == 0) || (!rs & 0xFFFF0000 == 0) {")
    g.write("_io.delay(%d);" % (4 if a else 3), indent = 2)
    g.write("} else if (rs & 0xFF000000 == 0) || (!rs & 0xFF000000 == 0) {")
    g.write("_io.delay(%d);" % (5 if a else 4), indent = 2)
    g.write("} else {")
    g.write("_io.delay(%d);" % (6 if a else 5), indent = 2)
    g.write("}")
    
    if a: # MLAL, UMLAL
        g.write("let acc = ((_cpu.get_register(rd_hi) as u64) << 32) | (_cpu.get_register(rd_lo) as u64);")
        g.write("let val = rm.wrapping_mul(rs).wrapping_add(acc);")
    else: # MULL, UMULL
        g.write("let val = rm.wrapping_mul(rs);")

    if s:
        g.write("_cpu.zero = val == 0;")
        g.write("_cpu.sign = (val as i64) < 0;")
        g.write("_cpu.carry = true;")

    g.write("_cpu.set_register(rd_hi, (val >> 32) as u32);")
    g.write("_cpu.set_register(rd_lo, val as u32);")
        
def write_half(g, high, low):
    pre = ((high >> 4) & 1) != 0
    up  = ((high >> 3) & 1) != 0
    imm = ((high >> 2) & 1) != 0
    wb  = (((high >> 1) & 1) != 0) or not pre
    load = (high & 1) != 0

    op = (low >> 1) & 3

    g.write("let rd = (instr >> 12) & 0xF;")
    g.write("let rn = (instr >> 16) & 0xF;")
    
    if imm:
        g.write("let off = (((instr & 0xF00) >> 4) | (instr & 0xF)) as u32;")
    else:
        g.write("let off = _cpu.get_register((instr & 0xF) as usize);")

    if pre:
        if up:
            g.write("let addr = _cpu.get_register(rn as usize).wrapping_add(off) as usize;")
        else:
            g.write("let addr = _cpu.get_register(rn as usize).wrapping_sub(off) as usize;")
    else:
        g.write("let addr = _cpu.get_register(rn as usize) as usize;")

    if op == 1: # LDRH/STRH
        if load:
            g.write("let val = if addr & 1 == 0 {")
            g.write("_cpu.read_u16(_io, addr)", indent = 2)
            g.write("} else {")
            g.write("_cpu.read_u16(_io, addr).swap_bytes()", indent = 2)
            g.write("};")
            g.write("_cpu.set_register(rd as usize, val as u32);")
        else:
            g.write("let val = _cpu.get_register(rd as usize) as u16;")
            g.write("_cpu.write_u16(_io, addr, val);")
    elif op == 2: # LDRSB
        if load:
            g.write("let val = _cpu.read_u8(_io, addr) as i8;")
            g.write("_cpu.set_register(rd as usize, (val as i32) as u32);")
        else:
            g.write('panic!("Tried to store signed byte");')
    elif op == 3: # LDRSH
        if load:
            g.write("let val = _cpu.read_u16(_io, addr) as i16;")
            g.write("_cpu.set_register(rd as usize, (val as i32) as u32);")
        else:
            g.write('panic!("Tried to store signed halfword");')

    if wb:
        g.write("if rn != rd {")
        if not pre:
            if up:
                g.write("let wb_val = addr.wrapping_add(off as usize);", indent = 2)
            else:
                g.write("let wb_val = addr.wrapping_sub(off as usize);", indent = 2)
        else:
            g.write("let wb_val = addr;", indent = 2)
            
        g.write("_cpu.set_register(rn as usize, wb_val as u32);", indent = 2)
        g.write("}")
        
        
def write_instruction(g, high, low):
    g.set_current(high * 16 + low)

    if (high & 0xE0) == 0xA0: # B/BL
        write_branch(g, high, low)
    elif high == 0x12 and low == 1: # BX
        write_branch_exchange(g)
    elif high & 0xF0 == 0xF0: # SWI
        write_swi(g)
    elif ((high & 0xFB) == 0x32) or ((high & 0xF9 == 0x10) and low == 0): # PSR transfer
        write_psr(g, high, low)
    elif (high & 0xC0) == 0x00 and not ((high & 0x20 == 0) and (low & 9 == 9)): # ALU
        write_alu(g, high, low)
    elif (high & 0xE0) == 0x00 and (low & 0x9) == 0x9 and (low != 9): # Halfword
        write_half(g, high, low)
    elif (high & 0xF8) == 0x00 and (low == 9): # Multiply
        write_mul(g, high)
    elif (high & 0xF8) == 0x08 and (low == 9): # Multiply long
        write_mull(g, high)
    elif (high & 0xFB) == 0x10 and (low == 9): # SWP
        write_swp(g, high)
    elif (high & 0xC0) == 0x40: # SDT
        write_sdt(g, high, low)
    elif (high & 0xE0) == 0x80: # BDT
        write_bdt(g, high, low)
    else:
        g.write("unimplemented!(\"{:08x} {:08x}\", instr, _cpu.pc);")

g = Generator(0x1000)
    
for high in range(0x0, 0x100):
    for low in range(0x0, 0x10):
        write_instruction(g, high, low)

g.optimize()
g.print_functions()
g.print_array()
