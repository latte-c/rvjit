use crate::types::*;

pub fn lui(rd: u8, imm: u32) -> u32 {
    rv_utype(imm, rd, 0b0110111)
}

pub fn auipc(rd: u8, imm: u32) -> u32 {
    rv_utype(imm, rd, 0b0010111)
}

pub fn jal(rd: u8, imm: u32) -> u32 {
    rv_jtype(imm, rd, 0b1101111)
}

pub fn jalr(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b000, rd, 0b1100111)
}

pub fn beq(imm: u32, rs1: u8, rs2: u8) -> u32 {
    rv_btype(imm, rs2, rs1, 0b000, 0b1100011)
}

pub fn bne(imm: u32, rs1: u8, rs2: u8) -> u32 {
    rv_btype(imm, rs2, rs1, 0b001, 0b1100011)
}

pub fn blt(imm: u32, rs1: u8, rs2: u8) -> u32 {
    rv_btype(imm, rs2, rs1, 0b100, 0b1100011)
}

pub fn bge(imm: u32, rs1: u8, rs2: u8) -> u32 {
    rv_btype(imm, rs2, rs1, 0b101, 0b1100011)
}

pub fn bltu(imm: u32, rs1: u8, rs2: u8) -> u32 {
    rv_btype(imm, rs2, rs1, 0b110, 0b1100011)
}

pub fn bgeu(imm: u32, rs1: u8, rs2: u8) -> u32 {
    rv_btype(imm, rs2, rs1, 0b111, 0b1100011)
}

pub fn lb(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b000, rd, 0b0000011)
}

pub fn lh(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b001, rd, 0b0000011)
}

pub fn lw(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b010, rd, 0b0000011)
}

pub fn lbu(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b100, rd, 0b0000011)
}

pub fn lhu(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b101, rd, 0b0000011)
}

pub fn sb(rs1: u8, rs2: u8, imm: u32) -> u32 {
    rv_stype(imm, rs2, rs1, 0b000, 0b0100011)
}

pub fn sh(rs1: u8, rs2: u8, imm: u32) -> u32 {
    rv_stype(imm, rs2, rs1, 0b001, 0b0100011)
}

pub fn sw(rs1: u8, rs2: u8, imm: u32) -> u32 {
    rv_stype(imm, rs2, rs1, 0b010, 0b0100011)
}

pub fn addi(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b000, rd, 0b0010011)
}

pub fn slti(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b010, rd, 0b0010011)
}

pub fn sltiu(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b011, rd, 0b0010011)
}

pub fn xori(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b100, rd, 0b0010011)
}

pub fn ori(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b110, rd, 0b0010011)
}

pub fn andi(rd: u8, rs1: u8, imm: u32) -> u32 {
    rv_itype(imm, rs1, 0b111, rd, 0b0010011)
}

pub fn slli32(rd: u8, rs1: u8, shamt: u8) -> u32 {
    rv_rtype(0b0000000, shamt, rs1, 0b001, rd, 0b0010011)
}

pub fn srli32(rd: u8, rs1: u8, shamt: u8) -> u32 {
    rv_rtype(0b0000000, shamt, rs1, 0b101, rd, 0b0010011)
}

pub fn srai32(rd: u8, rs1: u8, shamt: u8) -> u32 {
    rv_rtype(0b0100000, shamt, rs1, 0b101, rd, 0b0010011)
}

pub fn add(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b000, rd, 0b0110011)
}

pub fn sub(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0100000, rs2, rs1, 0b000, rd, 0b0110011)
}

pub fn sll(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b001, rd, 0b0110011)
}

pub fn slt(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b010, rd, 0b0110011)
}

pub fn sltu(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b011, rd, 0b0110011)
}

pub fn xor(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b100, rd, 0b0110011)
}

pub fn srl(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b101, rd, 0b0110011)
}

pub fn sra(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0100000, rs2, rs1, 0b101, rd, 0b0110011)
}

pub fn or(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b110, rd, 0b0110011)
}

pub fn and(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000000, rs2, rs1, 0b111, rd, 0b0110011)
}

pub fn ecall() -> u32 {
    rv_itype(0, 0, 0b000, 0, 0b1110011)
}

pub fn ebreak() -> u32 {
    rv_itype(1, 0, 0b000, 0, 0b1110011)
}
