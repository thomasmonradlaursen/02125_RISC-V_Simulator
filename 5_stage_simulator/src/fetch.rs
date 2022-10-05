use crate::registers::{IFID, PC};

pub fn update_register(pc: &mut PC, if_id: &mut IFID, mem: &[u8], branch: &bool, program_len: &usize) {
    if_id.instruction = fetch_instruction(mem, if_id);
    if_id.pc = pc.pc;
    if !*branch && pc.pc < *program_len {
        pc.pc += 4;
    }
}

pub fn fetch_instruction(mem: &[u8], if_id: &mut IFID) -> i32 {
    let bytes: [u8; 4] = [mem[0], mem[1], mem[2], mem[3]];
    i32::from_le_bytes(bytes)
}