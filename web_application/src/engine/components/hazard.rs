use crate::engine::components::registers::{EXMEM, IDEX, IFID};

pub fn ex_hazard(decode: &IFID, execute: &IDEX, stall: &mut bool) {
    if execute.control.reg_write && (execute.rd != 0) {
        if execute.rd == decode.rs1 {
            *stall = true;
        }
        if execute.rd == decode.rs2 {
            *stall = true;
        }
    }
}

pub fn mem_hazard(decode: &IFID, mem: &EXMEM, stall: &mut bool) {
    if mem.control.reg_write  && (mem.rd != 0) {
        if mem.rd == decode.rs1 {
            *stall = true;
        }
        if mem.rd == decode.rs2 {
            *stall = true;
        }
    }
}

pub fn load_use_hazard(decode: &IFID, execute: &IDEX, stall: &mut bool) {
    if execute.control.mem_read {
        if execute.rd == decode.rs1 {
            *stall = true;
        }
        if execute.rd == decode.rs2 {
            *stall = true;
        }
    }
}

pub fn load_use_hazard_extended(decode: &IFID, mem: &EXMEM, stall: &mut bool) {    
    if mem.control.mem_read {
        if mem.rd == decode.rs1 {
            *stall = true;
        }
        if mem.rd == decode.rs2 {
            *stall = true;
        }
    }
}

pub fn control_hazard(fetch: &mut IFID, decode: &mut IDEX, branch: &bool) {
    if *branch {
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
