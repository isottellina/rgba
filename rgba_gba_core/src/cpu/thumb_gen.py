# thumb_gen.py --- 
# 
# Filename: thumb_gen.py
# Author: Louise <louise>
# Created: Tue Jan 16 19:57:01 2018 (+0100)
# Last-Updated: Wed Jan 17 00:36:33 2018 (+0100)
#           By: Louise <louise>
# 
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

def write_instruction(high):
    print("#[allow(unreachable_code, unused_variables)]")
    print(
        "fn thumb_%02x(_cpu: &mut ARM7TDMI, _io: &mut Interconnect, instr: u16) {"
        % high
    )

    if high & 0xE0 == 0x20:
        write_f3(high)
    else:
        print("\tunimplemented!(\"{:04x}\", instr);")

    print("}\n")

for high in range(0x0, 0x100):
    write_instruction(high)

print("const THUMB_INSTRUCTIONS: [fn(&mut ARM7TDMI, &mut Interconnect, u16); 256] = ")
print("[" + ", ".join(["thumb_%02x" % i for i in range(0x0, 0x100)]) + "];")
