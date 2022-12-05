use crate::engine::components::registers::{EXMEM, IDEX, MEMWB};

pub fn ex_forward(decode: &IDEX, execute: &EXMEM, forward_a: &mut u8, forward_b: &mut u8) {
    if execute.control.reg_write && (execute.rd != 0) {
        if execute.rd == decode.rs1 {
            *forward_a = 1;
        }
        if execute.rd == decode.rs2 {
            *forward_b = 1;
        }
    }
}

pub fn mem_forward(decode: &IDEX, mem: &MEMWB, forward_a: &mut u8, forward_b: &mut u8) {
    if mem.control.reg_write && (mem.rd != 0) {
        if (mem.rd == decode.rs1) && (*forward_a != 1) {
            *forward_a = 2;
        }
        // NOTE: THIS NEEDS TO BE ADDRESSED! In terms of using rs2 in instruction which do not use rs2
        // A case for this can be seen in test_sb, where the last LB x6 -23(x9) has both rs1 and rs2 being 9.
        // (The imm110 for -23 in binary is 111111101001, where the lower 5 bits, representing rs2, is binary for 9)
        // Therefore, forward_b will be overwritten to 2, and therefore, the right value will not be forwarded
        // to rs1. Currently, when casting to usize, this will create an out of bound expection when loading.
        if (mem.rd == decode.rs2) && (*forward_b != 1) {
            *forward_b = 2;
        }
    } 
}

pub fn load_forward(decode: &IDEX, mem: &MEMWB, forward_a: &mut u8, forward_b: &mut u8) {
    if mem.control.mem_read && (mem.rd != 0) {
        if (mem.rd == decode.rs1) && *forward_a != 1{
            *forward_a = 3;
        }
        if (mem.rd == decode.rs2) && *forward_b != 1 {
            *forward_b = 3;
        }
    }
}

pub fn update_forward_a(destination: &mut IDEX, source_ex: &EXMEM, source_mem: &MEMWB, forward_a: &u8) {
    if *forward_a == 0 {
        ()
    }
    if *forward_a == 1 {
        destination.decoding.rs1 = source_ex.computation.result;
        ()
    }
    if *forward_a == 2 {
        destination.decoding.rs1 = source_mem.mem_result.alu_result;
        ()
    }
    if *forward_a == 3 {
        destination.decoding.rs1 = source_mem.mem_result.read_mem;
        ()
    }
}

pub fn update_forward_b(destination: &mut IDEX, source_ex: &EXMEM, source_mem: &MEMWB, forward_b: &u8) {
    if *forward_b == 0 {
        ()
    }
    if *forward_b == 1 {
        destination.decoding.rs2 = source_ex.computation.result;
        ()
    }
    if *forward_b == 2 {
        destination.decoding.rs2 = source_mem.mem_result.alu_result;
        ()
    }
    if *forward_b == 3 {
        destination.decoding.rs2 = source_mem.mem_result.read_mem;
        ()
    }
}

pub fn reset_multiplexers(forward_a: &mut u8, forward_b: &mut u8) {
    *forward_a = 0;
    *forward_b = 0;
}
