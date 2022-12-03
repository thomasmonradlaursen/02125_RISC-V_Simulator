use crate::engine::components::registers::MEMWB;

pub fn writeback(wb: &MEMWB, reg: &mut [i32; 32], running: &mut bool, program_len: &usize) {
    if !(wb.pc < *program_len) {
        *running = false;
    }
    if wb.control.mem_to_reg && wb.rd != 0 {
        reg[wb.rd] = wb.mem_result.read_mem;
    }
    if wb.control.reg_write && wb.rd != 0 {
        reg[wb.rd] = wb.mem_result.alu_result;
    }
}
