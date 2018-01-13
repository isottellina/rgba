# arm_gen.py --- 
# 
# Filename: arm_gen.py
# Author: Louise <louise>
# Created: Sat Jan 13 17:25:38 2018 (+0100)
# Last-Updated: Sat Jan 13 22:33:41 2018 (+0100)
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

def write_instruction(file, high, low):
    file.write(
        "fn arm_%03x(_cpu: &mut ARM7TDMI, _io: &mut Interconnect, instr: u32) {\n"
        % (high * 16 + low)
    )

    if (high & 0xE0) == 0xA0: # B/BL
        write_branch(file, high, low)
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
