use crate::registers::{EXMEM, IDEX, IFID, MEMWB};

pub fn ex_hazard(decode: &IDEX, execute: &EXMEM, stall: &mut bool) {
    println!("Execution hazards:");
    if execute.control.reg_write && (execute.rd != 0) {
        if execute.rd == decode.rs1 {
            println!(
                "EX/MEM Destination register: {} = ID/EX Source Register 1: {}",
                execute.rd, decode.rs1
            );
            *stall = true;
        }
        if execute.rd == decode.rs2 {
            println!(
                "EX/MEM Destination register: {} = ID/EX Source Register 2: {}",
                execute.rd, decode.rs2
            );
            *stall = true;
        }
    }
}

pub fn mem_hazard(decode: &IDEX, execute: &EXMEM, mem: &MEMWB, stall: &mut bool) {
    println!("Memory hazards:");
    if mem.control.reg_write
        && (mem.rd != 0)
        && !(execute.control.reg_write && (execute.rd != 0) && (execute.rd != decode.rs1))
        && (mem.rd == decode.rs1)
    {
        println!(
            "EX/MEM Destination register: {} = ID/EX Source Register 1: {}",
            execute.rd, decode.rs1
        );
        println!(
            "MEM/WB Destination register: {} = ID/EX Source Register 1: {}",
            mem.rd, decode.rs1
        );
        *stall = true;
    }
    if mem.control.reg_write
        && (mem.rd != 0)
        && !(execute.control.reg_write && (execute.rd != 0) && (execute.rd != decode.rs2))
        && (mem.rd == decode.rs2)
    {
        println!(
            "EX/MEM Destination register: {} = ID/EX Source Register 2: {}",
            execute.rd, decode.rs2
        );
        println!(
            "MEM/WB Destination register: {} = ID/EX Source Register 2: {}",
            mem.rd, decode.rs2
        );
        *stall = true;
    }
}

pub fn load_use_hazard(decode: &IFID, execute: &IDEX, stall: &mut bool) {
    println!("Hazards due to load-use condition:");
    if execute.control.mem_read && (execute.rd == decode.rs1) {
        println!("ID/EX Destination register: {} = IF/ID Source Register 1: {}", execute.rd, decode.rs1);
        *stall = true;
    }
    if execute.control.mem_read && (execute.rd == decode.rs2) {
        println!("ID/EX Destination register: {} = IF/ID Source Register 2: {}", execute.rd, decode.rs2);
        *stall = true;
    }
}

pub fn load_use_hazard_extended(decode: &IFID, mem: &EXMEM, stall: &mut bool) {
    println!("Hazards due to load-use condition AND no forwarding:");
    if mem.control.mem_read && (mem.rd == decode.rs1) {
        println!("EX/MEM Destination register: {} = IF/ID Source Register 1: {}", mem.rd, decode.rs1);
        *stall = true;
    }
    if mem.control.mem_read && (mem.rd == decode.rs2) {
        println!("EX/MEM Destination register: {} = IF/ID Source Register 2: {}", mem.rd, decode.rs2);
        *stall = true;
    }
}