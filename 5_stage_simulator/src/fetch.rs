use crate::registers::{PC, IFID};

pub fn fetch_to_register(fetch_a: &mut PC, fetch_b: &mut IFID, mem: &[u8], program_len: &usize) {
    if fetch_a.pc < *program_len {
    fetch_b.instruction = fetch_instruction(mem);
    fetch_b.pc = fetch_a.pc;
    } else {
        fetch_b.instruction = 0x2000;
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