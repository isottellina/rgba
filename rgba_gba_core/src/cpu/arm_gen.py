# arm_gen.py --- 
# 
# Filename: arm_gen.py
# Author: Louise <louise>
# Created: Sat Jan 13 17:25:38 2018 (+0100)
# Last-Updated: Sat Jan 13 18:14:27 2018 (+0100)
#           By: Louise <louise>
# 
import pathlib, sys

def write_instruction(file, high, low):
    file.write(
        """
    fn arm_%03x(_cpu: &mut ARM7TDMI, _io: &mut Interconnect, instr: u32) { 
        unimplemented!("{:08x}", instr);
    }
        """
        % (high * 16 + low))

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
