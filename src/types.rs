// generate and encode different types of instruction

use crate::immediate::*;

pub(crate) fn rv_rtype(funct7: u8, rs2: u8, rs1: u8, funct3: u8, rd: u8, opcode: u8) -> u32 {
    (funct7 as u32) << 25
        | (rs2 as u32) << 20
        | (rs1 as u32) << 15
        | (funct3 as u32) << 12
        | (rd as u32) << 7
        | (opcode as u32)
}

pub(crate) fn rv_itype(imm: u32, rs1: u8, funct3: u8, rd: u8, opcode: u8) -> u32 {
    immediate_i(imm)
        | (rs1 as u32) << 15
        | (funct3 as u32) << 12
        | (rd as u32) << 7
        | (opcode as u32)
}

pub(crate) fn rv_stype(imm: u32, rs2: u8, rs1: u8, funct3: u8, opcode: u8) -> u32 {
    immediate_s(imm)
        | (rs2 as u32) << 20
        | (rs1 as u32) << 15
        | (funct3 as u32) << 12
        | (opcode as u32)
}

pub(crate) fn rv_btype(imm: u32, rs2: u8, rs1: u8, funct3: u8, opcode: u8) -> u32 {
    immediate_b(imm)
        | (rs2 as u32) << 20
        | (rs1 as u32) << 15
        | (funct3 as u32) << 12
        | (opcode as u32)
}

pub(crate) fn rv_utype(imm: u32, rd: u8, opcode: u8) -> u32 {
    immediate_u(imm) | (rd as u32) << 7 | (opcode as u32)
}

pub(crate) fn rv_jtype(imm: u32, rd: u8, opcode: u8) -> u32 {
    immediate_j(imm) | (rd as u32) << 7 | (opcode as u32)
}
