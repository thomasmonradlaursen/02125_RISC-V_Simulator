use crate::decode::Decode;

pub struct Execute {
    pub instruction: i32,
    pub next_instruction: i32,
    pub result: i32,
    pub mem_address: usize,
    pub destination: usize,
}

impl Execute {
    pub fn execute_instruction(&mut self, decode: &Decode) {
        self.destination = decode.next_rd;
        match decode.next_opcode {
            0x03 => match decode.next_funct3 {
                0x00 => {
                    // LB - Load byte
                    self.mem_address = (decode.next_rs1 + decode.next_imm110) as usize;
                    //reg[rd] = mem[ decode.next_rs1 + decode.next_imm110) as usize] as i32;
                }
                0x01 => {
                    // LH - Load halfword
                    self.mem_address = (decode.next_rs1 + decode.next_imm110) as usize;
                    //let short: [u8; 4] = [mem[index], mem[index + 1], 0, 0];
                    //reg[rd] = i32::from_be_bytes(short);
                }
                0x02 => {
                    // LW - Load word
                    self.mem_address = (decode.next_rs1 + decode.next_imm110) as usize;
                    //let integer: [u8; 4] = [mem[index], mem[index + 1], mem[index + 2], mem[index + 3]];
                    //reg[rd] = i32::from_be_bytes(integer);
                }
                0x04 => {
                    // LHU - Load halfword unsigned
                    self.mem_address = (decode.next_rs1 + decode.next_imm110) as usize;
                    //let short: [u8; 4] = [mem[index], mem[index + 1], 0, 0];
                    //reg[rd] = u32::from_be_bytes(short) as i32;
                }
                0x05 => {
                    // LWU - Load word unsigned
                    self.mem_address = (decode.next_rs1 + decode.next_imm110) as usize;
                    //let integer: [u8; 4] = mem[index], mem[index + 1], mem[index + 2], mem[index + 3]];
                    //reg[rd] = u32::from_be_bytes(integer) as i32;
                }
                _ => (),
            },
            0x13 => match decode.next_funct3 {
                0x00 => {
                    // ADDI
                   self.result = decode.next_rs1 + decode.next_imm110;
                }
                0x01 => {
                    self.result = decode.next_rs1 << decode.next_shamt
                    //reg[rd] = decode.next_rs1 << decode.next_shamt;
                }
                0x02 => {
                    self.result = (decode.next_rs1 < decode.next_imm110) as i32;
                    /*if decode.next_rs1 < decode.next_imm110 {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }*/
                }
                0x03 => {
                    self.result = ((decode.next_rs1 as u32) < (decode.next_imm110 as u32)) as i32;
                    /* if  decode.next_rs1 as u32) < (decode.next_imm110 as u32) {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    } */
                }
                0x04 => {
                    self.result = decode.next_rs1 ^ decode.next_imm110;
                    // reg[rd] = decode.next_rs1 ^ decode.next_imm110;
                }
                // TODO:
                0x05 => match decode.next_funct7 {
                    0x00 => {
                        self.result = ((decode.next_rs1 as u32) >> (decode.next_shamt as u32)) as i32;
                        //reg[rd] = ((decode.next_rs1 as u32) >> (decode.next_shamt as u32)) as i32;
                    }
                    0x20 => {
                        self.result = decode.next_rs1 >> decode.next_shamt;
                        //reg[rd] = decode.next_rs1 >> decode.next_shamt;
                    }
                    _ => (),
                },
                0x06 => {
                    self.result = decode.next_rs1 | decode.next_imm110;
                    //reg[rd] = decode.next_rs1 | decode.next_imm110;
                }
                0x07 => {
                    self.result = decode.next_rs1 & decode.next_imm110;
                    //reg[rd] = decode.next_rs1 & decode.next_imm110;
                }
                _ => (),
            },
            0x17 => {
                !todo!()
                //reg[rd] = fetch.pc as i32 + decode.next_imm3112;
            }
            0x23 => match decode.next_funct3 {
                0x00 => {
                    self.mem_address = decode.next_rs1 as usize + decode.next_s_offset as usize;
                    // TODO
                    /* let offset = decode.next_s_offset;
                    let bytes = i32::to_be_bytes(decode.next_rs2);
                    mem[decode.next_rs1 as usize + offset as usize] = bytes[0]; */
                }
                0x01 => {
                    /*decode.next_s_offset;
                    let bytes = i32::to_be_bytes(decode.next_rs2);
                    mem decode.next_rs1 as usize + offset as usize] = bytes[0];
                    mem decode.next_rs1 as usize + offset as usize + 1] = bytes[1];*/
                }
                0x02 => {
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
                        //reg[rd] = decode.next_rs1 + decode.next_rs2;
                    }
                    0x20 => {
                        self.result = decode.next_rs1 - decode.next_rs2;
                        //reg[rd] = decode.next_rs1 - decode.next_rs2;
                    }
                    _ => (),
                },
                0x01 => {
                    self.result = decode.next_rs1 << decode.next_rs2;
                    //reg[rd] = decode.next_rs1 << decode.next_rs2;
                }
                0x02 => {
                    self.result = (decode.next_rs1 < decode.next_rs2) as i32;
                    /*if decode.next_rs1 < decode.next_rs2 {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }*/
                }
                0x03 => {
                    self.result = ((decode.next_rs1 as u32) < (decode.next_rs2 as u32)) as i32;
                    /*if (decode.next_rs1 as u32) < (decode.next_rs2 as u32) {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }*/
                }
                0x04 => {
                    self.result = decode.next_rs1 ^ decode.next_rs2;
                    //reg[rd] = decode.next_rs1 ^ decode.next_rs2;
                }
                0x05 => match decode.next_funct7 {
                    0x00 => {
                        self.result = ((decode.next_rs1 as u32) >> (decode.next_rs2 as u32)) as i32;
                        //reg[rd] = ((decode.next_rs1 as u32) >> (decode.next_rs2 as u32)) as i32;
                    }
                    0x20 => {
                        self.result = decode.next_rs1 >> decode.next_rs2;
                        //reg[rd] = decode.next_rs1 >> decode.next_rs2;
                    }
                    _ => (),
                },
                0x06 => {
                    self.result = decode.next_rs1 | decode.next_rs2;
                    //reg[rd] = decode.next_rs1 | decode.next_rs2;
                }
                0x07 => {
                    self.result =  decode.next_rs1 & decode.next_rs2;
                    //reg[rd] = decode.next_rs1 & decode.next_rs2;
                }
                _ => (),
            },
            0x37 => {
                // AUIPC
                // TODO
                //reg[rd] = decode.next_imm3112;
            }
            0x63 => match decode.next_funct3 {
                0x00 => {
                    self.result = (decode.next_rs1 == decode.next_rs2) as i32;
                    /*if decode.next_rs1 == decode.next_rs2 {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };*/
                }
                0x01 => {
                    self.result = (decode.next_rs1 != decode.next_rs2) as i32;
                    /*if decode.next_rs1 != decode.next_rs2 {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };*/
                }
                0x04 => {
                    self.result = (decode.next_rs1 < decode.next_rs2) as i32;
                    /*if decode.next_rs1 < decode.next_rs2 {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };*/
                }
                0x05 => {
                    self.result = (decode.next_rs1 >= decode.next_rs2) as i32;
                    /*if decode.next_rs1 >= decode.next_rs2 {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };*/
                }
                0x06 => {
                    self.result = ((decode.next_rs1 as u32) < (decode.next_rs2 as u32)) as i32;
                    /*if  (decode.next_rs1 as u32) < (decode.next_rs2 as u32) {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };*/
                }
                0x07 => {
                    self.result = ((decode.next_rs1 as u32) >= (decode.next_rs2 as u32)) as i32;
                    /*if  (decode.next_rs1 as u32) >= (decode.next_rs2 as u32) {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };*/
                }
                _ => (),
            },
            0x67 => match decode.next_funct3 {
                0x00 => {
                    self.result = decode.next_rs1 + decode.next_imm110;
                    /*reg[rd] = fetch.pc as i32 + 4;
                    fetch.pc = (decode.next_rs1 + decode.next_imm110) as usize;
                    branch = true; */
                }
                _ => (),
            },
            0x6F => {
                // TODO
                /*reg[rd] = (fetch.pc + 4) as i32;
                fetch.pc = fetch.pc + uj_format(&fetch.instruction) as usize;
                branch = true;*/
            }
            0x73 => {
                // TODO
                /*if reg[17] == 10 {
                    break;
                }*/
            }
            _ => (),
        }
    }

    pub fn update(&mut self) {
        self.next_instruction = self.instruction;
    }

    pub fn print_state(&self, instruction_string: &String) {
        println!("EXECUTE STAGE");
        println!("Instruction: {}", instruction_string);
    }
}
