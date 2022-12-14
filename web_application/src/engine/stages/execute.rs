use crate::engine::components::registers::{IDEX, EXMEM};
use gloo_dialogs;

#[derive(Debug, Clone, Copy)]
pub struct Computation {
    pub result: i32,
    pub carry: i32,
    pub mem_opcode: i32,
    pub mem_funct3: i32,
}

impl Default for Computation {
    fn default() -> Self {
        Self { result: 0, carry: 0, mem_opcode: 0, mem_funct3: 0 }
    }
}

pub fn update_for_memory(execute: &mut EXMEM, mem: &mut EXMEM) {
    mem.instruction = execute.instruction;
    mem.pc = execute.pc;
    mem.rd = execute.rd;
    mem.rs1 = execute.rs1;
    mem.rs2 = execute.rs2;
    mem.control = execute.control;
    mem.computation = execute.computation;
}

pub fn execute_to_register(execute_a: &mut IDEX, execute_b: &mut EXMEM, pc_src: &mut usize, branch: &mut bool, running: &mut bool) {
    execute_b.instruction = execute_a.instruction;
    execute_b.pc = execute_a.pc;
    execute_b.rd = execute_a.rd;
    execute_b.rs1 = execute_a.rs1;
    execute_b.rs2 = execute_a.rs2;
    execute_b.computation = execute_instruction(pc_src, execute_a, branch, running);
    execute_b.control = execute_a.control;
}

pub fn execute_instruction(pc_src: &mut usize, execute_a: &IDEX, branch: &mut bool, running: &mut bool) -> Computation {
    let mut computation = Computation{result: 0, carry: 0, mem_opcode: 0, mem_funct3: 0};
    match execute_a.decoding.opcode {
        0x00 => match execute_a.decoding.funct3 {
            0x02 => {
                computation.result = -1;
            }
            _ => (),
        },
        0x03 => match execute_a.decoding.funct3 {
            0x00 => {
                // LB - Load byte
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.imm110;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;

            }
            0x01 => {
                // LH - Load halfword
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.imm110;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;

            }
            0x02 => {
                // LW - Load word
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.imm110;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;

            }
            0x04 => {
                // LHU - Load halfword unsigned
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.imm110;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;

            }
            0x05 => {
                // LWU - Load word unsigned
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.imm110;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;

            }
            _ => (),
        },
        0x13 => match execute_a.decoding.funct3 {
            0x00 => {
                // ADDI
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.imm110;

            }
            0x01 => {
                computation.result = execute_a.decoding.rs1 << execute_a.decoding.shamt;

            }
            0x02 => {
                computation.result = (execute_a.decoding.rs1 < execute_a.decoding.imm110) as i32;

            }
            0x03 => {
                computation.result = ((execute_a.decoding.rs1 as u32) < (execute_a.decoding.imm110 as u32)) as i32;

            }
            0x04 => {
                computation.result = execute_a.decoding.rs1 ^ execute_a.decoding.imm110;

            }
            0x05 => match execute_a.decoding.funct7 {
                0x00 => {
                    computation.result = ((execute_a.decoding.rs1 as u32) >> (execute_a.decoding.shamt as u32)) as i32;
    
                }
                0x20 => {
                    computation.result = execute_a.decoding.rs1 >> execute_a.decoding.shamt;
    
                }
                _ => (),
            },
            0x06 => {
                computation.result = execute_a.decoding.rs1 | execute_a.decoding.imm110;

            }
            0x07 => {
                computation.result = execute_a.decoding.rs1 & execute_a.decoding.imm110;

            }
            _ => (),
        },
        0x17 => {
            // AUIPC
            computation.result = execute_a.pc as i32 + execute_a.decoding.imm3112;
        }
        0x23 => match execute_a.decoding.funct3 {
            0x00 => {
                // SB - Store byte
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.s_offset;
                computation.carry = execute_a.decoding.rs2;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;
            }
            0x01 => {
                // SH - Store halfword
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.s_offset;
                computation.carry = execute_a.decoding.rs2;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;
            }
            0x02 => {
                // SW - Store word
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.s_offset;
                computation.carry = execute_a.decoding.rs2;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;
            }
            _ => (),
        },
        0x33 => match execute_a.decoding.funct3 {
            0x00 => match execute_a.decoding.funct7 {
                0x00 => {
                    computation.result = execute_a.decoding.rs1 + execute_a.decoding.rs2;
    
                }
                0x20 => {
                    computation.result = execute_a.decoding.rs1 - execute_a.decoding.rs2;
    
                }
                _ => (),
            },
            0x01 => {
                computation.result = execute_a.decoding.rs1 << execute_a.decoding.rs2;

            }
            0x02 => {
                computation.result = (execute_a.decoding.rs1 < execute_a.decoding.rs2) as i32;

            }
            0x03 => {
                computation.result = ((execute_a.decoding.rs1 as u32) < (execute_a.decoding.rs2 as u32)) as i32;

            }
            0x04 => {
                computation.result = execute_a.decoding.rs1 ^ execute_a.decoding.rs2;

            }
            0x05 => match execute_a.decoding.funct7 {
                0x00 => {
                    computation.result = ((execute_a.decoding.rs1 as u32) >> (execute_a.decoding.rs2 as u32)) as i32;
    
                }
                0x20 => {
                    computation.result = execute_a.decoding.rs1 >> execute_a.decoding.rs2;
    
                }
                _ => (),
            },
            0x06 => {
                computation.result = execute_a.decoding.rs1 | execute_a.decoding.rs2;

            }
            0x07 => {
                computation.result = execute_a.decoding.rs1 & execute_a.decoding.rs2;

            }
            _ => (),
        },
        0x37 => {
            // LUI
            computation.result = execute_a.decoding.imm3112;
        }
        0x63 => match execute_a.decoding.funct3 {
            0x00 => {
                if execute_a.decoding.rs1 == execute_a.decoding.rs2 {
                    instruction_misalignemnt(&execute_a.decoding.sb_offset, running);
                    *pc_src = execute_a.pc + execute_a.decoding.sb_offset as usize;
                    *branch = true;
                }
            }
            0x01 => {
                if execute_a.decoding.rs1 != execute_a.decoding.rs2 {
                    instruction_misalignemnt(&execute_a.decoding.sb_offset, running);
                    *pc_src = execute_a.pc + execute_a.decoding.sb_offset as usize;
                    *branch = true;
                }
            }
            0x04 => {
                if execute_a.decoding.rs1 < execute_a.decoding.rs2 {
                    instruction_misalignemnt(&execute_a.decoding.sb_offset, running);
                    *pc_src = execute_a.pc + execute_a.decoding.sb_offset as usize;
                    *branch = true;
                }
            }
            0x05 => {
                if execute_a.decoding.rs1 >= execute_a.decoding.rs2 {
                    instruction_misalignemnt(&execute_a.decoding.sb_offset, running);
                    *pc_src = execute_a.pc + execute_a.decoding.sb_offset as usize;
                    *branch = true;
                }
            }
            0x06 => {
                if (execute_a.decoding.rs1 as u32) < (execute_a.decoding.rs2 as u32) {
                    instruction_misalignemnt(&execute_a.decoding.sb_offset, running);
                    *pc_src = execute_a.pc + execute_a.decoding.sb_offset as usize;
                    *branch = true;
                }
            }
            0x07 => {
                if (execute_a.decoding.rs1 as u32) >= (execute_a.decoding.rs2 as u32) {
                    instruction_misalignemnt(&execute_a.decoding.sb_offset, running);
                    *pc_src = execute_a.pc + execute_a.decoding.sb_offset as usize;
                    *branch = true;
                }
            }
            _ => (),
        },
        0x67 => match execute_a.decoding.funct3 {
            0x00 => {
                instruction_misalignemnt(&execute_a.decoding.imm110, running);
                computation.result = execute_a.pc as i32 + 4;
                *pc_src = (execute_a.decoding.rs1 + execute_a.decoding.imm110) as usize;
                *branch = true;

            }
            _ => (),
        },
        0x6F => {
            instruction_misalignemnt(&execute_a.decoding.uj_offset, running);
            computation.result = execute_a.pc as i32 + 4;
            *pc_src = execute_a.pc + execute_a.decoding.uj_offset as usize;
            *branch = true;
        }
        _ => (),
    }

    // Hardwired 0 in x0 needs to be preserved.
    if execute_a.rd == 0 {
        if execute_a.decoding.opcode != 0x23 || execute_a.decoding.opcode != 0x63 {
            computation.result = 0;
        }
    }
    computation
}

fn instruction_misalignemnt(offset: &i32, running: &mut bool) {
    if offset % 4 != 0 {
        gloo_dialogs::alert(&format!("WARNING: Instruction-misalignment-expection. Stopping simulation. {} % 4 != 0.", offset)[..]);
        *running = false;
    }
}