use crate::registers::{MEMWB};

pub fn writeback(mem_wb: &MEMWB, reg: &mut [i32; 32], running: &mut bool) {
    if mem_wb.mem_result.alu_result == -1 {
        *running = false;
    }
    if mem_wb.control.reg_write {
        reg[mem_wb.rd] = mem_wb.mem_result.alu_result;
    }
}
