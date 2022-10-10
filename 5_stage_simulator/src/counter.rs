use crate::registers::PC;

pub fn update_for_fetch(pc_a: &mut PC, pc_b: &mut PC) {
    pc_b.pc = pc_a.pc;
}

pub fn increment_program_counter(pc: &mut PC, amount: &usize, stall: &bool) {
    if !*stall {
       pc.pc += amount;
    }
}
