# thumb_gen.py --- 
# 
# Filename: thumb_gen.py
# Author: Louise <louise>
# Created: Tue Jan 16 19:57:01 2018 (+0100)
# Last-Updated: Tue Jan 16 20:04:34 2018 (+0100)
#           By: Louise <louise>
# 

def write_instruction(instr_code):
    print("#[allow(unreachable_code, unused_variables)]")
    print(
        "fn thumb_%02x(_cpu: &mut ARM7TDMI, _io: &mut Interconnect, instr: u16) {"
        % (instr_code)
    )

    print("\tunimplemented!(\"{:04x}\", instr);")

    print("}\n")

for high in range(0x0, 0x100):
    write_instruction(high)

print("const THUMB_INSTRUCTIONS: [fn(&mut ARM7TDMI, &mut Interconnect, u16); 256] = ")
print("[" + ", ".join(["thumb_%02x" % i for i in range(0x0, 0x100)]) + "];")
