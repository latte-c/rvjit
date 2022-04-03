import os


def gen_rv32i_inst(encode: str) -> str:
    splited = encode.strip().split(' ')
    inst_name = splited[-1]
    inst_type = splited[-2]
    opcode = '0b' + splited[-3]

    if inst_name in ['csrrw', 'csrrs', 'csrrc', 'csrrwi', 'csrrsi', 'csrrci']:
        return ""

    if inst_name in ['fence', 'fence.i']:
        return ""

    if inst_name in ['ecall', 'ebreak']:
        funct3 = '0b' + splited[2]
        return f'pub fn {inst_name}() -> u32 {{\n  rv_itype({"1" if inst_name == "ebreak" else "0"}, 0, {funct3}, 0, {opcode})\n}}'

    if inst_type == 'U':
        # generate U type
        func_signature = f'pub fn {inst_name}(rd: u8, imm: u32)'
        func_body = f'rv_utype(imm, rd, {opcode})'
    elif inst_type == 'J':
        # generate J type
        func_signature = f'pub fn {inst_name}(rd: u8, imm: u32)'
        func_body = f'rv_jtype(imm, rd, {opcode})'
    elif inst_type == 'I':
        # generate I type
        funct3 = '0b' + splited[2]
        # handle sr and sl separately
        if inst_name in ['slli', 'srli', 'srai']:
            funct7 = '0b' + splited[0]
            funct3 = '0b' + splited[3]
            func_signature = f'pub fn {inst_name}(rd: u8, rs1: u8, shamt: u8)'
            func_body = f'rv_rtype({funct7}, shamt, rs1, {funct3}, rd, {opcode})'
        else:
            func_signature = f'pub fn {inst_name}(rd: u8, rs1: u8, imm: u32)'
            func_body = f'rv_itype(imm, rs1, {funct3}, rd, {opcode})'
    elif inst_type == 'B':
        # generate B type
        funct3 = '0b' + splited[3]
        func_signature = f'pub fn {inst_name}(imm: u32, rs1: u8, rs2: u8)'
        func_body = f'rv_btype(imm, rs2, rs1, {funct3}, {opcode})'
    elif inst_type == 'R':
        # generate R type
        funct3 = '0b' + splited[3]
        funct7 = '0b' + splited[0]
        func_signature = f'pub fn {inst_name}(rd: u8, rs1: u8, rs2: u8)'
        func_body = f'rv_rtype({funct7}, rs2, rs1, {funct3}, rd, {opcode})'
    elif inst_type == 'S':
        funct3 = '0b' + splited[3]
        func_signature = f'pub fn {inst_name}(rs1: u8, rs2: u8, imm: u32)'
        func_body = f'rv_stype(imm, rs2, rs1, {funct3}, {opcode})'
    else:
        raise Exception(f'unknown instruction type {inst_type}')

    return f'{func_signature} -> u32 {{\n   {func_body}\n}}'


with open('rv32i.txt', 'r') as f:
    with open('../src/rv32i.rs', 'w') as of:
        of.write('use crate::types::*;\n\n')
        for l in f.readlines():
            inst = gen_rv32i_inst(l)
            of.write(inst + '\n\n')

os.system('rustfmt ../src/rv32i.rs')
