use crate::{
    registers::{EXMEM, IDEX, MEMWB},
};

pub fn detect_hazard(id_ex: &mut IDEX, ex_mem: &EXMEM, mem_wb: &MEMWB, stall: &mut bool) {
    println!("Hazard detection:");
    if ex_mem.rd != 0 {
        if ex_mem.rd == id_ex.rs1 {
            println!(
                "Memory access - rd: {}, Execute - rs1: {}",
                ex_mem.rd, id_ex.rs1
            );
            clear_decode(id_ex, stall);
        }
        if ex_mem.rd == id_ex.rs2 {
            println!(
                "Memory access - rd: {}, Execute - rs2: {}",
                ex_mem.rd, id_ex.rs2
            );
            clear_decode(id_ex, stall);
        }
    }
    if mem_wb.rd != 0 {
        if mem_wb.rd == id_ex.rs1 {
            println!(
                "Writeback - rd: {}, Execute - rs1: {}",
                mem_wb.rd, id_ex.rs1
            );
            clear_decode(id_ex, stall);
        }
        if mem_wb.rd == id_ex.rs2 {
            println!(
                "Writeback - rd: {}, Execute - rs2: {}",
                ex_mem.rd, id_ex.rs2
            );
            clear_decode(id_ex, stall);
        }
    }
    println!();
}

pub fn clear_decode(id_ex: &mut IDEX, stall: &mut bool) {
    id_ex.instruction = 0x3000;
    *stall = true;
}
