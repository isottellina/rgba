# arm_gen.py --- 
# 
# Filename: arm_gen.py
# Author: Louise <louise>
# Created: Sat Jan 13 17:25:38 2018 (+0100)
# Last-Updated: Mon Jan 15 12:14:49 2018 (+0100)
#           By: Louise <louise>
# 
import pathlib, sys

def write_branch(file, high, low):
    link = (high & 0x10) != 0
    file.write("\tlet offset = ((instr << 8) as i32) >> 6;\n")
    file.write("\tlet old_pc = _cpu.get_register(15);\n")
    
    if link:
        file.write("\t_cpu.set_register(14, old_pc);\n")

    file.write("\tlet new_pc = ((old_pc as i32) + offset) as u32;\n")
    file.write("\t_cpu.set_register(15, new_pc);\n")
    file.write("\t_cpu.advance_pipeline(_io);\n")

def write_op2_imm(file, high, low):
    s = (high & 0x01) != 0
    
    file.write("\tlet imm = instr & 0xFF;\n")
    file.write("\tlet rot = (instr & 0xF00) >> 7;\n")
    file.write("\tlet op2 = imm.rotate_right(rot);\n")
    if s:
        file.write("\tif rot != 0 { _cpu.carry = (op2 >> 31) != 0; }\n")

def write_op2_reg(file, high, low):
    s = (high & 0x01) != 0
    shift = (low & 0x6) >> 2
    
    file.write("\tlet rm = _cpu.get_register((instr & 0xF) as usize);\n")
    if (low & 1) == 0: # By immediate
        file.write("\tlet amount = (instr >> 7) & 0x1f;\n")
        if shift == 0:
            file.write("\tlet op2 = if amount == 0 { rm } else {")
            if s: file.write(" let tmp = rm << (amount - 1); _cpu.carry = (tmp >> 31) != 0; tmp << 1 };\n")
            else: file.write(" rm << amount };\n")
        elif shift == 1:
            file.write("\tlet op2 = if amount == 0 { ")
            if s:
                file.write("_cpu.carry = (rm & 0x80000000) != 0; 0 } else { let tmp = rm >> (amount - 1); ")
                file.write("_cpu.carry = (tmp & 1) != 0; tmp >> 1 };\n")
            else:
                file.write("0 } else { rm >> amount };\n")
        elif shift == 2:
            file.write("\tlet op2 = if amount == 0 { ")
            if s:
                file.write("_cpu.carry = (rm & 0x80000000) != 0; ((rm as i32) >> 31) as u32 } else { ")
                file.write("let tmp = ((rm as i32) >> (amount - 1)) as u32; _cpu.carry = tmp & 1 != 0; ")
                file.write("((tmp as i32) >> 1) as u32 };\n")
            else:
                file.write("((rm as i32) >> 31 } else { ((rm as i32) >> amount) as u32 };\n")
        elif shift == 3:
            file.write("\tlet op2 = if amount == 0 { ")
            if s:
                file.write("let tmp = (rm >> 1) | ((_cpu.carry as u32) << 31);  _cpu.carry = (rm & 1) != 0; ")
                file.write("tmp } else { let tmp = rm.rotate_right(amount); _cpu.carry = (tmp >> 31) != 0; ")
                file.write("tmp };\n")
            else:
                file.write("(rm >> 1) | ((_cpu.carry as u32) << 31) } else { rm.rotate_right(amount) };\n")
        
    else: # By register
        file.write("\tlet amount = _cpu.get_register(((instr >> 8) & 0xF) as usize) & 0xFF;\n")
        if shift == 0:
            if s:
                file.write("\tlet tmp = rm << (amount - 1); _cpu.carry = (tmp >> 31) != 0;\n")
                file.write("\tlet op2 = tmp << 1;\n")
            else:
                file.write("\tlet op2 = rm << amount;\n")
        elif shift == 1:
            if s:
                file.write("\tlet tmp = rm >> (amount - 1); _cpu.carry = (tmp & 1) != 0;\n")
                file.write("\tlet op2 = tmp >> 1 ;\n")
            else:
                file.write("\tlet op2 = rm >> amount;\n")
        elif shift == 2:
            if s:
                file.write("\tlet tmp = ((rm as i32) >> (amount - 1)) as u32; _cpu.carry = tmp & 1 != 0;\n")
                file.write("\tlet op2 = ((tmp as i32) >> 1) as u32;\n")
            else:
                file.write("\tlet op2 = ((rm as i32) >> amount) as u32;\n")
        elif shift == 3:
            file.write("\tlet op2 = rm.rotate_right(amount);\n")
            if s:
                file.write("\t_cpu.carry = (op2 >> 31) != 0;\n")
    
    
def write_alu(file, high, low):
    op = (high & 0x1e) >> 1
    s = (high & 0x01) != 0
    imm = (high & 0x20) != 0

    if op == 5 or op == 6 or op == 7:
        file.write("\tlet cf = _cpu.carry;\n")
    
    # Generation op2 code
    if imm:
        write_op2_imm(file, high, low)
    else:
        write_op2_reg(file, high, low)
        
    test = (op & 0xc == 0x8)
        
    if (op & 0xc == 0x8) and not s:
        file.write("\tpanic!(\"Generating bad ALU instruction (s)\");\n")
        return
    
    if op != 13 and op != 15:
        file.write("\tlet rn = _cpu.get_register(((instr >> 16) & 0xF) as usize);\n")

    if not test or s:
        file.write("\tlet rd = (instr >> 12) & 0xF;\n")
    
    if op == 8 or op == 0: # AND, TST
        file.write("\tlet res = rn & op2;\n")
    elif op == 9 or op == 1: # EOR, TEQ
        file.write("\tlet res = rn | op2;\n");
    elif op == 10 or op == 2: # SUB, CMP
        file.write("\tlet res = rn.wrapping_sub(op2);\n")
        if s:
            file.write("\tif rd != 15 {\n")
            file.write("\t\t_cpu.carry = rn >= op2;\n")
            file.write("\t\t_cpu.overflow = (rn ^ op2) & (rn ^ res) & 0x80000000 != 0;\n")
            file.write("\t}\n")
    elif op == 3: # RSB
        file.write("\tlet res = op2.wrapping_sub(rn);\n")
        if s:
            file.write("\tif rd != 15 {\n")
            file.write("\t\t_cpu.carry = op2 >= rn;\n")
            file.write("\t\t_cpu.overflow = (rn ^ op2) & (op2 ^ res) & 0x80000000 != 0;\n")
            file.write("\t}\n")
    elif op == 11 or op == 4: # ADD, CMN
        file.write("\tlet res = rn.wrapping_add(op2);\n")
        if s:
            file.write("\tif rd != 15 {\n")
            file.write("\t\t_cpu.carry = rn > res;\n")
            file.write("\t\t_cpu.overflow = !(rn ^ op2) & (rn ^ res) & 0x80000000 != 0;\n")
            file.write("\t}\n")
    elif op == 5: # ADC
        file.write("\tlet res = rn.wrapping_add(op2).wrapping_add(cf as u32);\n")
        if s:
            file.write("\tif rd != 15 {\n")
            file.write("\t\t_cpu.carry = if cf { rn >= res } else { rn > res };\n")
            file.write("\t\t_cpu.overflow = !(rn ^ op2) & (rn ^ res) & 0x80000000 != 0;\n")
            file.write("\t}\n")
    elif op == 6: # SBC
        file.write("\tlet res = rn.wrapping_sub(op2).wrapping_sub(cf as u32);\n")
        if s:
            file.write("\tif rd != 15 {\n")
            file.write("\t\t_cpu.carry = if cf { rn > op2 } else { rn >= op2 };\n")
            file.write("\t\t_cpu.overflow = (rn ^ op2) & (rn ^ res) & 0x80000000 != 0;\n")
            file.write("\t}\n")
    elif op == 7: # RSC
        file.write("\tlet res = op2.wrapping_sub(rn).wrapping_sub(cf as u32);\n")
        if s:
            file.write("\tif rd != 15 {\n")
            file.write("\t\t_cpu.carry = if cf { op2 > rn } else { op2 >= rn };\n")
            file.write("\t\t_cpu.overflow = (rn ^ op2) & (op2 ^ res) & 0x80000000 != 0;\n")
            file.write("\t}\n")
    elif op == 12: # ORR
        file.write("\tlet res = rn | op2;\n")
    elif op == 13: # MOV
        file.write("\tlet res = op2;\n")
    elif op == 14: # BIC
        file.write("\tlet res = rn & !op2;");
    elif op == 15: # MVN
        file.write("\tlet res = !op2;");
    else:
        file.write("\tlet res = 0;\n")
        file.write("\tunimplemented!(\"ALU instruction not implemented : {:08x}\", instr);\n")

    if s:
        file.write("\tif rd != 15 { _cpu.sign = (res as i32) < 0; _cpu.zero = res == 0; }\n")
    if not test:
        file.write("\t_cpu.set_register(rd as usize, res);\n")
        file.write("\tif rd == 15 { unimplemented!(\"Setting r15 via ALU\"); }")
    
def write_instruction(file, high, low):
    file.write(
        "fn arm_%03x(_cpu: &mut ARM7TDMI, _io: &mut Interconnect, instr: u32) {\n"
        % (high * 16 + low)
    )

    if (high & 0xE0) == 0xA0: # B/BL
        write_branch(file, high, low)
    elif (high & 0xC0) == 0x00: # ALU
        write_alu(file, high, low)
    else:
        file.write("\tunimplemented!(\"{:08x}\", instr);\n")

    file.write("}\n\n")

try:
    out = pathlib.Path(out_dir) / "arm_generated.rs"
    file = open(out, "w")
except:
    file = sys.stdout

for high in range(0x0, 0x100):
    for low in range(0x0, 0x10):
        write_instruction(file, high, low)

file.write("const ARM_INSTRUCTIONS: [fn(&mut ARM7TDMI, &mut Interconnect, u32); 4096] = ")
file.write("[" + ", ".join(["arm_%03x" % i for i in range(0x0, 0x1000)]) + "];\n")
