# arm_gen.py --- 
# 
# Filename: arm_gen.py
# Author: Louise <louise>
# Created: Sat Jan 13 17:25:38 2018 (+0100)
# Last-Updated: Sun Jan 14 23:38:56 2018 (+0100)
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
    pass
    
def write_alu(file, high, low):
    op = (high & 0x1e) >> 1
    s = (high & 0x01) != 0
    imm = (high & 0x20) != 0

    # Generation op2 code
    if imm:
        write_op2_imm(file, high, low)
    else:
        file.write("\tlet op2 = 0;")
        file.write("\tunimplemented!(\"Reg op2 unimplemented\");\n")

    test = (op & 0xc == 0x8)
        
    if (op & 0xc == 0x8) and not s:
        file.write("\tunimplemented!(\"Generating bad ALU instruction (s)\");\n")
        return

    if op != 13 and op != 15:
        file.write("\tlet rn = _cpu.get_register(((instr >> 16) & 0xF) as usize);\n")
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
    elif op == 13: # MOV
        file.write("\tlet res = op2;\n")
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
