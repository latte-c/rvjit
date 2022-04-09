use crate::types::*;

pub fn mul(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000001, rs2, rs1, 000, rd, 0b0110011)
}

pub fn mulh(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000001, rs2, rs1, 001, rd, 0b0110011)
}

pub fn mulhsu(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000001, rs2, rs1, 010, rd, 0b0110011)
}

pub fn mulhu(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000001, rs2, rs1, 011, rd, 0b0110011)
}

pub fn div(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000001, rs2, rs1, 100, rd, 0b0110011)
}

pub fn divu(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000001, rs2, rs1, 101, rd, 0b0110011)
}

pub fn rem(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000001, rs2, rs1, 110, rd, 0b0110011)
}

pub fn remu(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000001, rs2, rs1, 111, rd, 0b0110011)
}
