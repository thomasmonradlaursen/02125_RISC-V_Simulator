use crate::{decode::Decode, fetch::Fetch};

pub struct Execute {
    pub instruction: i32,
    pub next_instruction: i32,

    pub pc: usize,

    pub result: i32,
    pub mem_address: usize,
    pub destination: usize,
    pub mem_opcode: i32,
    pub mem_funct3: i32,
    pub reg_write: bool,

    pub next_result: i32,
    pub next_mem_address: usize,
    pub next_destination: usize,
    pub next_mem_opcode: i32,
    pub next_mem_funct3: i32,
    pub next_reg_write: bool,
}

impl Execute {
    pub fn execute_instruction(&mut self, fetch: &mut Fetch, decode: &mut Decode, branch: &mut bool) {
        self.destination = decode.next_rd;
        self.mem_opcode = 0;
        self.mem_funct3 = 0;
        match decode.next_opcode {
            0x00 => match decode.next_funct3 {
                0x02 => {
                    self.result = -1;
                }
                _ => (),

            }
            0x03 => match decode.next_funct3 {
                0x00 => {
                    // LB - Load byte
                    self.mem_address = (decode.next_rs1 + decode.next_imm110) as usize;
                    self.mem_opcode = decode.opcode;
                    self.mem_funct3 = decode.funct3;
                    self.reg_write = true;
                    //reg[rd] = mem[ decode.next_rs1 + decode.next_imm110) as usize] as i32;
                }
                0x01 => {
                    // LH - Load halfword
                    self.mem_address = (decode.next_rs1 + decode.next_imm110) as usize;
                    self.mem_opcode = decode.opcode;
                    self.mem_funct3 = decode.funct3;
                    self.reg_write = true;
                    //let short: [u8; 4] = [mem[index], mem[index + 1], 0, 0];
                    //reg[rd] = i32::from_be_bytes(short);
                }
                0x02 => {
                    // LW - Load word
                    self.mem_address = (decode.next_rs1 + decode.next_imm110) as usize;
                    self.mem_opcode = decode.opcode;
                    self.mem_funct3 = decode.funct3;
                    self.reg_write = true;
                    //let integer: [u8; 4] = [mem[index], mem[index + 1], mem[index + 2], mem[index + 3]];
                    //reg[rd] = i32::from_be_bytes(integer);
                }
                0x04 => {
                    // LHU - Load halfword unsigned
                    self.mem_address = (decode.next_rs1 + decode.next_imm110) as usize;
                    self.mem_opcode = decode.opcode;
                    self.mem_funct3 = decode.funct3;
                    self.reg_write = true;
                    //let short: [u8; 4] = [mem[index], mem[index + 1], 0, 0];
                    //reg[rd] = u32::from_be_bytes(short) as i32;
                }
                0x05 => {
                    // LWU - Load word unsigned
                    self.mem_address = (decode.next_rs1 + decode.next_imm110) as usize;
                    self.mem_opcode = decode.opcode;
                    self.mem_funct3 = decode.funct3;
                    self.reg_write = true;
                    //let integer: [u8; 4] = mem[index], mem[index + 1], mem[index + 2], mem[index + 3]];
                    //reg[rd] = u32::from_be_bytes(integer) as i32;
                }
                _ => (),
            },
            0x13 => match decode.next_funct3 {
                0x00 => {
                    // ADDI
                   self.result = decode.next_rs1 + decode.next_imm110;
                   self.reg_write = true;
                }
                0x01 => {
                    self.result = decode.next_rs1 << decode.next_shamt;
                    self.reg_write = true;
                    //reg[rd] = decode.next_rs1 << decode.next_shamt;
                }
                0x02 => {
                    self.result = (decode.next_rs1 < decode.next_imm110) as i32;
                    self.reg_write = true;
                    /*if decode.next_rs1 < decode.next_imm110 {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }*/
                }
                0x03 => {
                    self.result = ((decode.next_rs1 as u32) < (decode.next_imm110 as u32)) as i32;
                    self.reg_write = true;
                    /* if  decode.next_rs1 as u32) < (decode.next_imm110 as u32) {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    } */
                }
                0x04 => {
                    self.result = decode.next_rs1 ^ decode.next_imm110;
                    self.reg_write = true;
                    // reg[rd] = decode.next_rs1 ^ decode.next_imm110;
                }
                // TODO:
                0x05 => match decode.next_funct7 {
                    0x00 => {
                        self.result = ((decode.next_rs1 as u32) >> (decode.next_shamt as u32)) as i32;
                        self.reg_write = true;
                        //reg[rd] = ((decode.next_rs1 as u32) >> (decode.next_shamt as u32)) as i32;
                    }
                    0x20 => {
                        self.result = decode.next_rs1 >> decode.next_shamt;
                        self.reg_write = true;
                        //reg[rd] = decode.next_rs1 >> decode.next_shamt;
                    }
                    _ => (),
                },
                0x06 => {
                    self.result = decode.next_rs1 | decode.next_imm110;
                    self.reg_write = true;
                    //reg[rd] = decode.next_rs1 | decode.next_imm110;
                }
                0x07 => {
                    self.result = decode.next_rs1 & decode.next_imm110;
                    self.reg_write = true;
                    //reg[rd] = decode.next_rs1 & decode.next_imm110;
                }
                _ => (),
            },
            0x17 => {
                // TODO
                // AUIPC
                self.result = self.pc as i32 + decode.next_imm3112;
                self.reg_write = true;
                //reg[rd] = fetch.pc as i32 + decode.next_imm3112;
            }
            0x23 => match decode.next_funct3 {
                0x00 => {
                    self.mem_address = decode.next_rs1 as usize + decode.next_s_offset as usize;
                    self.result = decode.next_rs2;
                    self.mem_opcode = decode.opcode;
                    self.mem_funct3 = decode.funct3;
                    self.reg_write = false;
                    // TODO
                    /* let offset = decode.next_s_offset;
                    let bytes = i32::to_be_bytes(decode.next_rs2);
                    mem[decode.next_rs1 as usize + offset as usize] = bytes[0]; */
                }
                0x01 => {
                    self.mem_address = decode.next_rs1 as usize + decode.next_s_offset as usize;
                    self.result = decode.next_rs2;
                    self.mem_opcode = decode.opcode;
                    self.mem_funct3 = decode.funct3;
                    self.reg_write = false;
                    /*decode.next_s_offset;
                    let bytes = i32::to_be_bytes(decode.next_rs2);
                    mem decode.next_rs1 as usize + offset as usize] = bytes[0];
                    mem decode.next_rs1 as usize + offset as usize + 1] = bytes[1];*/
                }
                0x02 => {
                    self.mem_address = decode.next_rs1 as usize + decode.next_s_offset as usize;
                    self.result = decode.next_rs2;
                    self.mem_opcode = decode.opcode;
                    self.mem_funct3 = decode.funct3;
                    self.reg_write = false;
                    /*decode.next_s_offset
                    let bytes = i32::to_be_bytes(decode.next_rs2);
                    mem decode.next_rs1 as usize + offset as usize] = bytes[0];
                    mem decode.next_rs1 as usize + offset as usize + 1] = bytes[1];
                    mem decode.next_rs1 as usize + offset as usize + 2] = bytes[2];
                    mem decode.next_rs1 as usize + offset as usize + 3] = bytes[3];*/
                }
                _ => (),
            },
            0x33 => match decode.next_funct3 {
                0x00 => match decode.next_funct7 {
                    0x00 => {
                        self.result = decode.next_rs1 + decode.next_rs2;
                        self.reg_write = true;
                        //reg[rd] = decode.next_rs1 + decode.next_rs2;
                    }
                    0x20 => {
                        self.result = decode.next_rs1 - decode.next_rs2;
                        self.reg_write = true;
                        //reg[rd] = decode.next_rs1 - decode.next_rs2;
                    }
                    _ => (),
                },
                0x01 => {
                    self.result = decode.next_rs1 << decode.next_rs2;
                    self.reg_write = true;
                    //reg[rd] = decode.next_rs1 << decode.next_rs2;
                }
                0x02 => {
                    self.result = (decode.next_rs1 < decode.next_rs2) as i32;
                    self.reg_write = true;
                    /*if decode.next_rs1 < decode.next_rs2 {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }*/
                }
                0x03 => {
                    self.result = ((decode.next_rs1 as u32) < (decode.next_rs2 as u32)) as i32;
                    self.reg_write = true;
                    /*if (decode.next_rs1 as u32) < (decode.next_rs2 as u32) {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }*/
                }
                0x04 => {
                    self.result = decode.next_rs1 ^ decode.next_rs2;
                    self.reg_write = true;
                    //reg[rd] = decode.next_rs1 ^ decode.next_rs2;
                }
                0x05 => match decode.next_funct7 {
                    0x00 => {
                        self.result = ((decode.next_rs1 as u32) >> (decode.next_rs2 as u32)) as i32;
                        self.reg_write = true;
                        //reg[rd] = ((decode.next_rs1 as u32) >> (decode.next_rs2 as u32)) as i32;
                    }
                    0x20 => {
                        self.result = decode.next_rs1 >> decode.next_rs2;
                        self.reg_write = true;
                        //reg[rd] = decode.next_rs1 >> decode.next_rs2;
                    }
                    _ => (),
                },
                0x06 => {
                    self.result = decode.next_rs1 | decode.next_rs2;
                    self.reg_write = true;
                    //reg[rd] = decode.next_rs1 | decode.next_rs2;
                }
                0x07 => {
                    self.result =  decode.next_rs1 & decode.next_rs2;
                    self.reg_write = true;
                    //reg[rd] = decode.next_rs1 & decode.next_rs2;
                }
                _ => (),
            },
            0x37 => {
                // LUI
                // TODO
                self.result = decode.next_imm3112;
                self.reg_write = true;
            }
            0x63 => match decode.next_funct3 {
                0x00 => {
                    if decode.next_rs1 == decode.next_rs2 {
                        fetch.pc = self.pc + decode.next_sb_offset as usize;
                        self.flush_and_branch(fetch, decode, branch);
                    }
                    self.reg_write = false;
                    /*if decode.next_rs1 == decode.next_rs2 {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };*/
                }
                0x01 => {
                    if decode.next_rs1 != decode.next_rs2 {
                        fetch.pc = self.pc + decode.next_sb_offset as usize;
                        self.flush_and_branch(fetch, decode, branch);
                    }
                    self.reg_write = false;
                    /*if decode.next_rs1 != decode.next_rs2 {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };*/
                }
                0x04 => {
                    if decode.next_rs1 < decode.next_rs2 {
                        fetch.pc = self.pc + decode.next_sb_offset as usize;
                        self.flush_and_branch(fetch, decode, branch);
                    }
                    self.reg_write = false;
                    /*if decode.next_rs1 < decode.next_rs2 {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };*/
                }
                0x05 => {
                    if decode.next_rs1 >= decode.next_rs2 {
                        fetch.pc = self.pc + decode.next_sb_offset as usize;
                        self.flush_and_branch(fetch, decode, branch);
                    }
                    self.reg_write = false;
                    /*if decode.next_rs1 >= decode.next_rs2 {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };*/
                }
                0x06 => {
                    if (decode.next_rs1 as u32) < (decode.next_rs2 as u32) {
                        fetch.pc = self.pc + decode.next_sb_offset as usize;
                        self.flush_and_branch(fetch, decode, branch);
                    }
                    self.reg_write = false;
                    /*if  (decode.next_rs1 as u32) < (decode.next_rs2 as u32) {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };*/
                }
                0x07 => {
                    if (decode.next_rs1 as u32) >= (decode.next_rs2 as u32) {
                        fetch.pc = self.pc + decode.next_sb_offset as usize;
                        self.flush_and_branch(fetch, decode, branch);
                    }
                    self.reg_write = false;
                    /*if  (decode.next_rs1 as u32) >= (decode.next_rs2 as u32) {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };*/
                }
                _ => (),
            },
            0x67 => match decode.next_funct3 {
                0x00 => {
                    self.result = self.pc as i32 + 4;
                    fetch.pc = (decode.next_rs1 + decode.next_imm110) as usize;
                    self.flush_and_branch(fetch, decode, branch);
                    self.reg_write = true;
                    /*reg[rd] = fetch.pc as i32 + 4;
                    fetch.pc = (decode.next_rs1 + decode.next_imm110) as usize;
                    branch = true; */
                }
                _ => (),
            },
            0x6F => {
                // TODO
                self.result = self.pc as i32 + 4;
                fetch.pc += decode.next_uj_offset as usize;
                self.flush_and_branch(fetch, decode, branch);
                self.reg_write = true;
                /*reg[rd] = (fetch.pc + 4) as i32;
                fetch.pc = fetch.pc + uj_format(&fetch.instruction) as usize;
                branch = true;*/
            }
            0x73 => {
                // TODO ECALL
                /*if reg[17] == 10 {
                    break;
                }*/
            }
            _ => (),
        }
    }

    fn flush_and_branch(&mut self, fetch: &mut Fetch, decode: &mut Decode, branch: &mut bool) {
        fetch.instruction = 0x1000;
        decode.instruction = 0x1000;
        decode.opcode = 0;
        decode.funct3 = 0;
        decode.funct7 = 0;
        decode.rd = 0;
        decode.rs1 = 0;
        decode.rs2 = 0;
        decode.imm3112 = 0;
        decode.imm110 = 0;
        decode.shamt = 0;
        decode.s_offset = 0;
        decode.sb_offset = 0;
        decode.uj_offset = 0;
        *branch = true;
    }

    pub fn update(&mut self) {
        self.next_instruction = self.instruction;
        self.next_result = self.result;
        self.next_mem_address = self. mem_address;
        self.next_destination = self. destination;
        self.next_mem_opcode = self.mem_opcode;
        self.next_mem_funct3 = self.mem_funct3;
        self.next_reg_write = self.reg_write;
    }

    pub fn print_state(&self, instruction_string: &String) {
        println!("EXECUTE STAGE");
        println!("Program counter: {}", self.pc);
        println!("Instruction: {}\n", instruction_string);
    }
}
