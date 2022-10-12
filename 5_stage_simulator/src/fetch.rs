use crate::registers::{IFID};

pub fn fetch_to_register(pc: &mut usize, fetch: &mut IFID, mem: &[u8], program_len: &usize) {
    if *pc <= *program_len {
    fetch.instruction = fetch_instruction(mem);
    fetch.pc = *pc;
    } else {
        fetch.instruction = 0x2000;
    }
}

pub fn update_for_decode(fetch: &mut IFID, decode: &mut IFID) {
    decode.instruction = fetch.instruction;
    decode.pc = fetch.pc;
}

pub fn fetch_instruction(mem: &[u8]) -> i32 {
    let bytes: [u8; 4] = [mem[0], mem[1], mem[2], mem[3]];
    i32::from_le_bytes(bytes)
}