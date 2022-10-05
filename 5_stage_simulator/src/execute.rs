use crate::{registers::{IDEX, EXMEM, PC, IFID}};

pub struct Computation {
    pub result: i32,
    pub carry: i32,
    pub mem_opcode: i32,
    pub mem_funct3: i32,
}

pub fn update_register(id_ex: &IDEX, ex_mem: &mut EXMEM, computation: Computation) {
    ex_mem.instruction = id_ex.instruction;
    ex_mem.pc = id_ex.pc;
    ex_mem.rd = id_ex.rd;
    ex_mem.rs1 = id_ex.rs1;
    ex_mem.rs2 = id_ex.rs2;
    ex_mem.control = id_ex.control;
    ex_mem.computation = computation; 
}

pub fn execute_instruction(pc: &mut PC, if_id: &mut IFID, id_ex: &IDEX, branch: &mut bool) -> Computation {
    let mut computation = Computation{result: 0, carry: 0, mem_opcode: 0, mem_funct3: 0};
    match id_ex.decoding.opcode {
        0x00 => match id_ex.decoding.funct3 {
            0x02 => {
                computation.result = -1;
            }
            _ => (),
        },
        0x03 => match id_ex.decoding.funct3 {
            0x00 => {
                // LB - Load byte
                computation.result = id_ex.decoding.rs1 + id_ex.decoding.imm110;
                computation.mem_opcode = id_ex.decoding.opcode;
                computation.mem_funct3 = id_ex.decoding.funct3;
                //self.reg_write = true;
            }
            0x01 => {
                // LH - Load halfword
                computation.result = id_ex.decoding.rs1 + id_ex.decoding.imm110;
                computation.mem_opcode = id_ex.decoding.opcode;
                computation.mem_funct3 = id_ex.decoding.funct3;
                //self.reg_write = true;
            }
            0x02 => {
                // LW - Load word
                computation.result = id_ex.decoding.rs1 + id_ex.decoding.imm110;
                computation.mem_opcode = id_ex.decoding.opcode;
                computation.mem_funct3 = id_ex.decoding.funct3;
                //self.reg_write = true;
            }
            0x04 => {
                // LHU - Load halfword unsigned
                computation.result = id_ex.decoding.rs1 + id_ex.decoding.imm110;
                computation.mem_opcode = id_ex.decoding.opcode;
                computation.mem_funct3 = id_ex.decoding.funct3;
                //self.reg_write = true;
            }
            0x05 => {
                // LWU - Load word unsigned
                computation.result = id_ex.decoding.rs1 + id_ex.decoding.imm110;
                computation.mem_opcode = id_ex.decoding.opcode;
                computation.mem_funct3 = id_ex.decoding.funct3;
                //self.reg_write = true;
            }
            _ => (),
        },
        0x13 => match id_ex.decoding.funct3 {
            0x00 => {
                // ADDI
                computation.result = id_ex.decoding.rs1 + id_ex.decoding.imm110;
                //self.reg_write = true;
            }
            0x01 => {
                computation.result = id_ex.decoding.rs1 << id_ex.decoding.shamt;
                //self.reg_write = true;
            }
            0x02 => {
                computation.result = (id_ex.decoding.rs1 < id_ex.decoding.imm110) as i32;
                //self.reg_write = true;
            }
            0x03 => {
                computation.result = ((id_ex.decoding.rs1 as u32) < (id_ex.decoding.imm110 as u32)) as i32;
                //self.reg_write = true;
            }
            0x04 => {
                computation.result = id_ex.decoding.rs1 ^ id_ex.decoding.imm110;
                //self.reg_write = true;
            }
            0x05 => match id_ex.decoding.funct7 {
                0x00 => {
                    computation.result = ((id_ex.decoding.rs1 as u32) >> (id_ex.decoding.shamt as u32)) as i32;
                    //self.reg_write = true;
                }
                0x20 => {
                    computation.result = id_ex.decoding.rs1 >> id_ex.decoding.shamt;
                    //self.reg_write = true;
                }
                _ => (),
            },
            0x06 => {
                computation.result = id_ex.decoding.rs1 | id_ex.decoding.imm110;
                //self.reg_write = true;
            }
            0x07 => {
                computation.result = id_ex.decoding.rs1 & id_ex.decoding.imm110;
                //self.reg_write = true;
            }
            _ => (),
        },
        0x17 => {
            // AUIPC
            computation.result = id_ex.pc as i32 + id_ex.decoding.imm3112;
            //self.reg_write = true;
        }
        0x23 => match id_ex.decoding.funct3 {
            0x00 => {
                computation.result = id_ex.decoding.rs1 + id_ex.decoding.s_offset;
                computation.carry = id_ex.decoding.rs2;
                computation.mem_opcode = id_ex.decoding.opcode;
                computation.mem_funct3 = id_ex.decoding.funct3;
                //self.reg_write = false;
            }
            0x01 => {
                computation.result = id_ex.decoding.rs1 + id_ex.decoding.s_offset;
                computation.result = id_ex.decoding.rs2;
                computation.mem_opcode = id_ex.decoding.opcode;
                computation.mem_funct3 = id_ex.decoding.funct3;
                //self.reg_write = false;
            }
            0x02 => {
                computation.result = id_ex.decoding.rs1 + id_ex.decoding.s_offset;
                computation.result = id_ex.decoding.rs2;
                computation.mem_opcode = id_ex.decoding.opcode;
                computation.mem_funct3 = id_ex.decoding.funct3;
                //self.reg_write = false;
            }
            _ => (),
        },
        0x33 => match id_ex.decoding.funct3 {
            0x00 => match id_ex.decoding.funct7 {
                0x00 => {
                    computation.result = id_ex.decoding.rs1 + id_ex.decoding.rs2;
                    //self.reg_write = true;
                }
                0x20 => {
                    computation.result = id_ex.decoding.rs1 - id_ex.decoding.rs2;
                    //self.reg_write = true;
                }
                _ => (),
            },
            0x01 => {
                computation.result = id_ex.decoding.rs1 << id_ex.decoding.rs2;
                //self.reg_write = true;
            }
            0x02 => {
                computation.result = (id_ex.decoding.rs1 < id_ex.decoding.rs2) as i32;
                //self.reg_write = true;
            }
            0x03 => {
                computation.result = ((id_ex.decoding.rs1 as u32) < (id_ex.decoding.rs2 as u32)) as i32;
                //self.reg_write = true;
            }
            0x04 => {
                computation.result = id_ex.decoding.rs1 ^ id_ex.decoding.rs2;
                //self.reg_write = true;
            }
            0x05 => match id_ex.decoding.funct7 {
                0x00 => {
                    computation.result = ((id_ex.decoding.rs1 as u32) >> (id_ex.decoding.rs2 as u32)) as i32;
                    //self.reg_write = true;
                }
                0x20 => {
                    computation.result = id_ex.decoding.rs1 >> id_ex.decoding.rs2;
                    //self.reg_write = true;
                }
                _ => (),
            },
            0x06 => {
                computation.result = id_ex.decoding.rs1 | id_ex.decoding.rs2;
                //self.reg_write = true;
            }
            0x07 => {
                computation.result = id_ex.decoding.rs1 & id_ex.decoding.rs2;
                //self.reg_write = true;
            }
            _ => (),
        },
        0x37 => {
            // LUI
            computation.result = id_ex.decoding.imm3112;
            //self.reg_write = true;
        }
        0x63 => match id_ex.decoding.funct3 {
            0x00 => {
                if id_ex.decoding.rs1 == id_ex.decoding.rs2 {
                    pc.pc = id_ex.pc + id_ex.decoding.sb_offset as usize;
                    flush_and_branch(pc, if_id, branch);
                }
                //self.reg_write = false;
            }
            0x01 => {
                if id_ex.decoding.rs1 != id_ex.decoding.rs2 {
                    pc.pc = id_ex.pc + id_ex.decoding.sb_offset as usize;
                    flush_and_branch(pc, if_id, branch);
                }
                //self.reg_write = false;
            }
            0x04 => {
                if id_ex.decoding.rs1 < id_ex.decoding.rs2 {
                    pc.pc = id_ex.pc + id_ex.decoding.sb_offset as usize;
                    flush_and_branch(pc, if_id, branch);
                }
                //self.reg_write = false;
            }
            0x05 => {
                if id_ex.decoding.rs1 >= id_ex.decoding.rs2 {
                    pc.pc = id_ex.pc + id_ex.decoding.sb_offset as usize;
                    flush_and_branch(pc, if_id, branch);
                }
                //self.reg_write = false;
            }
            0x06 => {
                if (id_ex.decoding.rs1 as u32) < (id_ex.decoding.rs2 as u32) {
                    pc.pc = id_ex.pc + id_ex.decoding.sb_offset as usize;
                    flush_and_branch(pc, if_id, branch);
                }
                //self.reg_write = false;
            }
            0x07 => {
                if (id_ex.decoding.rs1 as u32) >= (id_ex.decoding.rs2 as u32) {
                    pc.pc = id_ex.pc + id_ex.decoding.sb_offset as usize;
                    flush_and_branch(pc, if_id, branch);
                }
                //self.reg_write = false;
            }
            _ => (),
        },
        0x67 => match id_ex.decoding.funct3 {
            0x00 => {
                computation.result = id_ex.pc as i32 + 4;
                pc.pc = (id_ex.decoding.rs1 + id_ex.decoding.imm110) as usize;
                flush_and_branch(pc, if_id, branch);
                //self.reg_write = true;
            }
            _ => (),
        },
        0x6F => {
            computation.result = id_ex.pc as i32 + 4;
            pc.pc += id_ex.decoding.uj_offset as usize;
            flush_and_branch(pc, if_id, branch);
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

fn flush_and_branch(pc: &mut PC, if_id: &mut IFID, branch: &mut bool) {
    pc.instruction = 0x1000;
    if_id.instruction = 0x1000;
    *branch = true;
}
