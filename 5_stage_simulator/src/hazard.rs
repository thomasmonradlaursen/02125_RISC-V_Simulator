use crate::registers::{EXMEM, IDEX, IFID};

pub fn ex_hazard(decode: &IFID, execute: &IDEX, stall: &mut bool) {
    println!("Execution hazards:");
    if execute.control.reg_write && (execute.rd != 0) {
        if execute.rd == decode.rs1 {
            println!("EX/MEM rd: {} = ID/EX rs1: {}", execute.rd, decode.rs1);
            *stall = true;
        }
        if execute.rd == decode.rs2 {
            println!("EX/MEM rd: {} = ID/EX rs2: {}", execute.rd, decode.rs2);
            *stall = true;
        }
    }
}

pub fn mem_hazard(decode: &IFID, mem: &EXMEM, stall: &mut bool) {
    println!("Memory hazards:");
    if mem.control.reg_write  && (mem.rd != 0) {
        if mem.rd == decode.rs1 {
            println!("MEM/WB rd: {} = ID/EX rs1: {}", mem.rd, decode.rs1);
            *stall = true;
        }
        if mem.rd == decode.rs2 {
            println!("MEM/WB rd: {} = ID/EX rs2: {}", mem.rd, decode.rs2);
            *stall = true;
        }
    }
}

pub fn load_use_hazard(decode: &IFID, execute: &IDEX, stall: &mut bool) {
    println!("Load-use hazards:");
    if execute.control.mem_read {
        if execute.rd == decode.rs1 {
            println!("ID/EX rd: {} = IF/ID rs1: {}", execute.rd, decode.rs1);
            *stall = true;
        }
        if execute.rd == decode.rs2 {
            println!("ID/EX rd: {} = IF/ID rs2: {}", execute.rd, decode.rs2);
            *stall = true;
        }
    }
}

pub fn load_use_hazard_extended(decode: &IFID, mem: &EXMEM, stall: &mut bool) {
    println!("Load-use hazards without forwarding:");
    
    if mem.control.mem_read {
        if mem.rd == decode.rs1 {
            println!("EX/MEM rd: {} = IF/ID rs1: {}", mem.rd, decode.rs1);
            *stall = true;
        }
        if mem.rd == decode.rs2 {
            println!("EX/MEM rd: {} = IF/ID rs2: {}", mem.rd, decode.rs2);
            *stall = true;
        }
    }
}

pub fn control_hazard(fetch: &mut IFID, decode: &mut IDEX, branch: &bool) {
    println!("Control hazard:");
    if *branch {
        println!("Flushing fetch and decode");
        *fetch = IFID {
            ..Default::default()
        };
        fetch.instruction = 0x1000;
        *decode = IDEX {
            ..Default::default()
        };
        decode.instruction = 0x1000;
    }
}
