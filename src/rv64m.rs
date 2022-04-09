use crate::types::*;

pub fn mulw(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000001, rs2, rs1, 000, rd, 0b0111011)
}

pub fn divw(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000001, rs2, rs1, 100, rd, 0b0111011)
}

pub fn divuw(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000001, rs2, rs1, 101, rd, 0b0111011)
}

pub fn remw(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000001, rs2, rs1, 110, rd, 0b0111011)
}

pub fn remuw(rd: u8, rs1: u8, rs2: u8) -> u32 {
    rv_rtype(0b0000001, rs2, rs1, 111, rd, 0b0111011)
}
