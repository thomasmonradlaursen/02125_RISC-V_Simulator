use crate::registers::{EXMEM, IDEX, IFID, MEMWB};

pub fn ex_forward(decode: &IDEX, execute: &EXMEM, forward_a: &mut u8) {
    println!("Fowarding from execution:");
    *forward_a = 0;
    if execute.control.reg_write && (execute.rd != 0) {
        if execute.rd == decode.rs1 {
            println!("EX/MEM rd: {} = ID/EX rs1: {}", execute.rd, decode.rs1);
            println!("Forwarding ALU result from EX/MEM {}", execute.computation.result);
            *forward_a = 1;
        }
        if execute.rd == decode.rs2 {
            println!("EX/MEM rd: {} = ID/EX rs2: {}", execute.rd, decode.rs2);
            println!("Forwarding ALU result from EX/MEM {}", execute.computation.result);
            *forward_a = 2;
        }
    }
}

pub fn mem_hazard(decode: &IDEX, mem: &MEMWB, forward_a: &u8, forward_b: &mut u8) {
    *forward_b = 0;
    println!("Forwarding from memory access:");
    if mem.control.reg_write && (mem.rd != 0) {
        if (mem.rd == decode.rs1) && (*forward_a != 1) {
            println!("MEM/WB rd: {} = ID/EX rs1: {}", mem.rd, decode.rs1);
            println!("Forwarding ALU result from MEM/WB {}", mem.mem_result.alu_result);
            *forward_b = 1;
        }
        // NOTE: THIS NEEDS TO BE ADDRESSED! In terms of using rs2 in instruction which do not use rs2
        // A case for this can be seen in test_sb, where the last LB x6 -23(x9) has both rs1 and rs2 being 9.
        // (The imm110 for -23 in binary is 111111101001, where the lower 5 bits, representing rs2, is binary for 9)
        // Therefore, forward_b will be overwritten to 2, and therefore, the right value will not be forwarded
        // to rs1. Currently, when casting to usize, this will create an out of bound expection when loading.
    } 
    if mem.control.reg_write && (mem.rd != 0) {
        if (mem.rd == decode.rs2) && (*forward_a != 2) {
            println!("MEM/WB rd: {} = ID/EX rs2: {}", mem.rd, decode.rs2);
            println!("Forwarding ALU result from MEM/WB {}", mem.mem_result.alu_result);
            *forward_b = 2;
        }
    }
}

pub fn load_use_forward(decode: &IFID, mem: &EXMEM, forward_c: &mut u8) {
    *forward_c = 0;
    println!("Forwarding from memory access due to load-use:");
    if mem.control.mem_read && (mem.rd == decode.rs1) {
        println!("EX/MEM rd: {} = IF/ID rs1: {}", mem.rd, decode.rs1);
        println!("Forwarding memory read from MEM/WB");
        *forward_c = 1;
    }
    if mem.control.mem_read && (mem.rd == decode.rs2) {
        println!("EX/MEM rd: {} = IF/ID rs2: {}", mem.rd, decode.rs2);
        println!("Forwarding memory read from MEM/WB");
        *forward_c = 2;
    }
}

pub fn update_forward_a(destination: &mut IDEX, source: &EXMEM, forward_a: &u8) {
    if *forward_a == 0 {
        ()
    }
    if *forward_a == 1 {
        destination.decoding.rs1 = source.computation.result;
        ()
    }
    if *forward_a == 2 {
        destination.decoding.rs2 = source.computation.result;
        ()
    }
}

pub fn update_forward_b(destination: &mut IDEX, source: &MEMWB, forward_b: &u8) {
    if *forward_b == 0 {
        ()
    }
    if *forward_b == 1 {
        destination.decoding.rs1 = source.mem_result.alu_result;
        ()
    }
    if *forward_b == 2 {
        destination.decoding.rs2 = source.mem_result.alu_result;
        ()
    }
}

pub fn update_forward_c(destination: &mut IDEX, source: &MEMWB, forward_c: &u8) {
    if *forward_c == 0 {
        ()
    }
    if *forward_c == 1 {
        destination.decoding.rs1 = source.mem_result.read_mem;
        ()
    }
    if *forward_c == 2 {
        destination.decoding.rs2 = source.mem_result.read_mem;
        ()
    }
}

