use crate::{registers::{IDEX, EXMEM, IFID}};

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

pub fn execute_to_register(execute_a: &mut IDEX, execute_b: &mut EXMEM, pc_src: &mut usize, branch: &mut bool, fetch: &mut IFID, decode: &mut IDEX) {
    execute_b.instruction = execute_a.instruction;
    execute_b.pc = execute_a.pc;
    execute_b.rd = execute_a.rd;
    execute_b.rs1 = execute_a.rs1;
    execute_b.rs2 = execute_a.rs2;
    execute_b.computation = execute_instruction(pc_src, execute_a, branch, fetch, decode);
    execute_b.control = execute_a.control;
}

pub fn execute_instruction(pc_src: &mut usize, execute_a: &IDEX, branch: &mut bool, fetch: &mut IFID, decode: &mut IDEX) -> Computation {
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
                //self.reg_write = true;
            }
            0x01 => {
                // LH - Load halfword
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.imm110;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;
                //self.reg_write = true;
            }
            0x02 => {
                // LW - Load word
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.imm110;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;
                //self.reg_write = true;
            }
            0x04 => {
                // LHU - Load halfword unsigned
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.imm110;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;
                //self.reg_write = true;
            }
            0x05 => {
                // LWU - Load word unsigned
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.imm110;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;
                //self.reg_write = true;
            }
            _ => (),
        },
        0x13 => match execute_a.decoding.funct3 {
            0x00 => {
                // ADDI
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.imm110;
                //self.reg_write = true;
            }
            0x01 => {
                computation.result = execute_a.decoding.rs1 << execute_a.decoding.shamt;
                //self.reg_write = true;
            }
            0x02 => {
                computation.result = (execute_a.decoding.rs1 < execute_a.decoding.imm110) as i32;
                //self.reg_write = true;
            }
            0x03 => {
                computation.result = ((execute_a.decoding.rs1 as u32) < (execute_a.decoding.imm110 as u32)) as i32;
                //self.reg_write = true;
            }
            0x04 => {
                computation.result = execute_a.decoding.rs1 ^ execute_a.decoding.imm110;
                //self.reg_write = true;
            }
            0x05 => match execute_a.decoding.funct7 {
                0x00 => {
                    computation.result = ((execute_a.decoding.rs1 as u32) >> (execute_a.decoding.shamt as u32)) as i32;
                    //self.reg_write = true;
                }
                0x20 => {
                    computation.result = execute_a.decoding.rs1 >> execute_a.decoding.shamt;
                    //self.reg_write = true;
                }
                _ => (),
            },
            0x06 => {
                computation.result = execute_a.decoding.rs1 | execute_a.decoding.imm110;
                //self.reg_write = true;
            }
            0x07 => {
                computation.result = execute_a.decoding.rs1 & execute_a.decoding.imm110;
                //self.reg_write = true;
            }
            _ => (),
        },
        0x17 => {
            // AUIPC
            computation.result = execute_a.pc as i32 + execute_a.decoding.imm3112;
            //self.reg_write = true;
        }
        0x23 => match execute_a.decoding.funct3 {
            0x00 => {
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.s_offset;
                computation.carry = execute_a.decoding.rs2;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;
                //self.reg_write = false;
            }
            0x01 => {
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.s_offset;
                computation.result = execute_a.decoding.rs2;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;
                //self.reg_write = false;
            }
            0x02 => {
                computation.result = execute_a.decoding.rs1 + execute_a.decoding.s_offset;
                computation.result = execute_a.decoding.rs2;
                computation.mem_opcode = execute_a.decoding.opcode;
                computation.mem_funct3 = execute_a.decoding.funct3;
                //self.reg_write = false;
            }
            _ => (),
        },
        0x33 => match execute_a.decoding.funct3 {
            0x00 => match execute_a.decoding.funct7 {
                0x00 => {
                    computation.result = execute_a.decoding.rs1 + execute_a.decoding.rs2;
                    //self.reg_write = true;
                }
                0x20 => {
                    computation.result = execute_a.decoding.rs1 - execute_a.decoding.rs2;
                    //self.reg_write = true;
                }
                _ => (),
            },
            0x01 => {
                computation.result = execute_a.decoding.rs1 << execute_a.decoding.rs2;
                //self.reg_write = true;
            }
            0x02 => {
                computation.result = (execute_a.decoding.rs1 < execute_a.decoding.rs2) as i32;
                //self.reg_write = true;
            }
            0x03 => {
                computation.result = ((execute_a.decoding.rs1 as u32) < (execute_a.decoding.rs2 as u32)) as i32;
                //self.reg_write = true;
            }
            0x04 => {
                computation.result = execute_a.decoding.rs1 ^ execute_a.decoding.rs2;
                //self.reg_write = true;
            }
            0x05 => match execute_a.decoding.funct7 {
                0x00 => {
                    computation.result = ((execute_a.decoding.rs1 as u32) >> (execute_a.decoding.rs2 as u32)) as i32;
                    //self.reg_write = true;
                }
                0x20 => {
                    computation.result = execute_a.decoding.rs1 >> execute_a.decoding.rs2;
                    //self.reg_write = true;
                }
                _ => (),
            },
            0x06 => {
                computation.result = execute_a.decoding.rs1 | execute_a.decoding.rs2;
                //self.reg_write = true;
            }
            0x07 => {
                computation.result = execute_a.decoding.rs1 & execute_a.decoding.rs2;
                //self.reg_write = true;
            }
            _ => (),
        },
        0x37 => {
            // LUI
            computation.result = execute_a.decoding.imm3112;
            //self.reg_write = true;
        }
        0x63 => match execute_a.decoding.funct3 {
            0x00 => {
                if execute_a.decoding.rs1 == execute_a.decoding.rs2 {
                    *pc_src = execute_a.pc + execute_a.decoding.sb_offset as usize;
                    flush_and_branch(fetch, decode, branch);
                }
                //self.reg_write = false;
            }
            0x01 => {
                if execute_a.decoding.rs1 != execute_a.decoding.rs2 {
                    *pc_src = execute_a.pc + execute_a.decoding.sb_offset as usize;
                    flush_and_branch(fetch, decode, branch);
                }
                //self.reg_write = false;
            }
            0x04 => {
                if execute_a.decoding.rs1 < execute_a.decoding.rs2 {
                    *pc_src = execute_a.pc + execute_a.decoding.sb_offset as usize;
                    flush_and_branch(fetch, decode, branch);
                }
                //self.reg_write = false;
            }
            0x05 => {
                if execute_a.decoding.rs1 >= execute_a.decoding.rs2 {
                    *pc_src = execute_a.pc + execute_a.decoding.sb_offset as usize;
                    flush_and_branch(fetch, decode, branch);
                }
                //self.reg_write = false;
            }
            0x06 => {
                if (execute_a.decoding.rs1 as u32) < (execute_a.decoding.rs2 as u32) {
                    *pc_src = execute_a.pc + execute_a.decoding.sb_offset as usize;
                    flush_and_branch(fetch, decode, branch);
                }
                //self.reg_write = false;
            }
            0x07 => {
                if (execute_a.decoding.rs1 as u32) >= (execute_a.decoding.rs2 as u32) {
                    *pc_src = execute_a.pc + execute_a.decoding.sb_offset as usize;
                    flush_and_branch(fetch, decode, branch);
                }
                //self.reg_write = false;
            }
            _ => (),
        },
        0x67 => match execute_a.decoding.funct3 {
            0x00 => {
                computation.result = execute_a.pc as i32 + 4;
               * pc_src = (execute_a.decoding.rs1 + execute_a.decoding.imm110) as usize;
                flush_and_branch(fetch, decode, branch);
                //self.reg_write = true;
            }
            _ => (),
        },
        0x6F => {
            computation.result = execute_a.pc as i32 + 4;
            *pc_src += execute_a.decoding.uj_offset as usize;
            flush_and_branch(fetch, decode, branch);
            //self.reg_write = true;
        }
        0x73 => {
            /*if reg[17] == 10 {
                break;
            }*/
        }
        _ => (),
    }
    computation
}

fn flush_and_branch(fetch: &mut IFID, decode: &mut IDEX, branch: &mut bool) {
    *fetch = IFID {..Default::default()};
    fetch.instruction = 0x1000;
    *decode = IDEX {..Default::default()};
    decode.instruction = 0x1000;
    *branch = true;
}
