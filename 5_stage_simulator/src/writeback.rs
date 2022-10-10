use crate::registers::{MEMWB};

pub fn writeback(wb: &MEMWB, reg: &mut [i32; 32], running: &mut bool) {
    if wb.mem_result.alu_result == -1 {
        *running = false;
    }
    if wb.control.reg_write {
        reg[wb.rd] = wb.mem_result.alu_result;
    }
}
