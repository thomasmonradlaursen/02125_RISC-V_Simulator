use crate::engine::components::{control::Control, registers::{IFID, IDEX}};

#[derive(Debug, Clone, Copy)]
pub struct Decoding {
    pub opcode: i32,
    pub funct3: i32,
    pub funct7: i32,
    pub rs1: i32,
    pub rs2: i32,
    pub imm3112: i32,
    pub imm110: i32,
    pub shamt: i32,
    pub s_offset: i32,
    pub sb_offset: i32,
    pub uj_offset: i32,
}

impl Default for Decoding {
    fn default() -> Self {
        Self {
            opcode: 0,
            funct3: 0,
            funct7: 0,
            rs1: 0,
            rs2: 0,
            imm3112: 0,
            imm110: 0,
            shamt: 0,
            s_offset: 0,
            sb_offset: 0,
            uj_offset: 0,
        }
    }
}

pub fn update_for_execution(decode: &mut IDEX, execute: &mut IDEX, reg: &[i32; 32]) {
    execute.instruction = decode.instruction;
    execute.pc = decode.pc;
    execute.rd = decode.rd;
    execute.rs1 = decode.rs1;
    execute.rs2 = decode.rs2;
    execute.control = decode.control;
    execute.decoding = decode.decoding;
    execute.decoding.rs1 = reg[decode.rs1];
    execute.decoding.rs2 = reg[decode.rs2];
}

pub fn decode_to_register(decode_a: &mut IFID, decode_b: &mut IDEX, reg: &[i32; 32]) {
    decode_b.instruction = decode_a.instruction;
    decode_b.pc = decode_a.pc;
    decode_b.rd = ((decode_a.instruction >> 7) & 0x01f) as usize;
    decode_b.rs1 = ((decode_a.instruction >> 15) & 0x01f) as usize;
    decode_b.rs2 = ((decode_a.instruction >> 20) & 0x01f) as usize;
    decode_b.decoding = decode_instruction(&decode_a.instruction, &decode_b.rs1, &decode_b.rs2, reg);
    decode_b.control = Control::compute_control(&decode_b.decoding.opcode);
}

pub fn decode_instruction(
    instruction: &i32,
    rs1_address: &usize,
    rs2_address: &usize,
    reg: &[i32; 32],
) -> Decoding {
    Decoding {
        opcode: instruction & 0x7f,
        funct3: (instruction >> 12) & 0x07,
        funct7: instruction >> 25,
        rs1: reg[*rs1_address],
        rs2: reg[*rs2_address],
        imm3112: (instruction >> 12) << 12,
        imm110: instruction >> 20,
        shamt: (instruction >> 20) & 0x01f,
        s_offset: s_offset(&instruction),
        sb_offset: sb_offset(&instruction),
        uj_offset: uj_offset(&instruction),
    }
}

fn uj_offset(instruction: &i32) -> i32 {
    let bit20: i32 = (instruction >> 31) << 20; // 20
    let bit101: i32 = ((instruction >> 21) & 0x3ff) << 1; // 10 9 8 7 6 5 4 3 2 1
    let bit1912: i32 = instruction & 0xff000; // 19 18 17 16 15 14 13 12
    let bit11: i32 = (((instruction & 0x100000) as u32) >> 9) as i32; // 11
    ((bit101 | bit1912) | bit11) | bit20
}

fn sb_offset(instruction: &i32) -> i32 {
    let bit11 = (instruction & 0x80) << 4; // 11
    let bit12 = (instruction >> 31) << 12; // 12
    let bit41 = ((instruction >> 8) & 0x0f) << 1; // 4 3 2 1
    let bit105 = ((instruction >> 25) & 0x3f) << 5; // 10 9 8 7 6 5
    ((bit41 | bit105) | bit11) | bit12
}

fn s_offset(instruction: &i32) -> i32 {
    let bit40 = (instruction >> 7) & 0x1f;
    let bit115 = (instruction >> 25) << 5;
    bit40 | bit115
}
