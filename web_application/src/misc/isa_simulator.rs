pub fn simulate(reg: &mut [i32; 32], mem: &mut [u8; 262144], program_len: &usize) {
    
    let mut pc: usize = 0;
    let mut branch = false;

    loop {
        let instruction = convert_to_instruction(&mem[pc..(pc  + 4)]);
        let opcode = instruction & 0x7f;
        let funct3 = (instruction >> 12) & 0x07;
        let funct7 = instruction >> 25;
        let rd = ((instruction >> 7) & 0x01f) as usize;
        let rs1 = ((instruction >> 15) & 0x01f) as usize;
        let rs2 = ((instruction >> 20) & 0x01f) as usize;
        let imm3112 = (instruction >> 12) << 12;
        let imm110 = instruction >> 20;
        let shamt = (instruction >> 20) & 0x01f;

        match opcode {
            0x03 => match funct3 {
                0x00 => {
                    reg[rd] = (mem[(reg[rs1] + imm110) as usize] as i8) as i32;
                }
                0x01 => {
                    let index = (reg[rs1] + imm110) as usize;
                    let bytes: [u8; 2] = [mem[index], mem[index + 1]];
                    reg[rd] = i16::from_le_bytes(bytes) as i32;
                }
                0x02 => {
                    let index = (reg[rs1] + imm110) as usize;
                    let integer: [u8; 4] =
                        [mem[index], mem[index + 1], mem[index + 2], mem[index + 3]];
                    reg[rd] = i32::from_le_bytes(integer);
                }
                0x04 => {
                    reg[rd] = mem[(reg[rs1] + imm110) as usize] as i32;   
                }
                0x05 => {
                    let index = (reg[rs1] + imm110) as usize;
                    let bytes: [u8; 2] = [mem[index], mem[index + 1]];
                    reg[rd] = u16::from_le_bytes(bytes) as i32;
                }
                _ => (),
            },
            0x13 => match funct3 {
                0x00 => {
                    reg[rd] = reg[rs1] + imm110;
                }
                0x01 => {
                    reg[rd] = reg[rs1] << shamt;
                }
                0x02 => {
                    if reg[rs1] < imm110 {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }
                }
                0x03 => {
                    if (reg[rs1] as u32) < (imm110 as u32) {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }
                }
                0x04 => {
                    reg[rd] = reg[rs1] ^ imm110;   
                }
                0x05 => match funct7 {
                    0x00 => {
                        reg[rd] = ((reg[rs1] as u32) >> (shamt as u32)) as i32;
                    }
                    0x20 => {
                        reg[rd] = reg[rs1] >> shamt;
                    }
                    _ => (),
                },
                0x06 => {
                    reg[rd] = reg[rs1] | imm110;
                }
                0x07 => {
                    reg[rd] = reg[rs1] & imm110;
                }
                _ => (),
            },
            0x17 => {
                reg[rd] = pc as i32 + imm3112;
            }
            0x23 => match funct3 {
                0x00 => {
                    let offset = s_format(&instruction);
                    let bytes = i32::to_le_bytes(reg[rs2]);
                    mem[reg[rs1] as usize + offset] = bytes[0];
                }
                0x01 => {
                    let offset = s_format(&instruction);
                    let bytes = i32::to_le_bytes(reg[rs2]);
                    mem[reg[rs1] as usize + offset] = bytes[0];
                    mem[reg[rs1] as usize + offset + 1] = bytes[1];
                }
                0x02 => {
                    let offset = s_format(&instruction);
                    let bytes = i32::to_le_bytes(reg[rs2]);
                    mem[reg[rs1] as usize + offset] = bytes[0];
                    mem[reg[rs1] as usize + offset + 1] = bytes[1];
                    mem[reg[rs1] as usize + offset + 2] = bytes[2];
                    mem[reg[rs1] as usize + offset + 3] = bytes[3];   
                }
                _ => (),
            },
            0x33 => match funct3 {
                0x00 => match funct7 {
                    0x00 => {
                        reg[rd] = reg[rs1] + reg[rs2];
                    }
                    0x20 => {
                        reg[rd] = reg[rs1] - reg[rs2];
                    }
                    _ => (),
                },
                0x01 => {
                    reg[rd] = reg[rs1] << reg[rs2];
                }
                0x02 => {
                    if reg[rs1] < reg[rs2] {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }
                }
                0x03 => {
                    if (reg[rs1] as u32) < (reg[rs2] as u32) {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }
                }
                0x04 => {
                    reg[rd] = reg[rs1] ^ reg[rs2];
                }
                0x05 => match funct7 {
                    0x00 => {
                        reg[rd] = ((reg[rs1] as u32) >> (reg[rs2] as u32)) as i32;
                    }
                    0x20 => {
                        reg[rd] = reg[rs1] >> reg[rs2];
                    }
                    _ => (),
                },
                0x06 => {
                    reg[rd] = reg[rs1] | reg[rs2];
                }
                0x07 => {
                    reg[rd] = reg[rs1] & reg[rs2];
                }
                _ => (),
            },
            0x37 => {
                reg[rd] = imm3112;
            }
            0x63 => match funct3 {
                0x00 => {
                    if reg[rs1] == reg[rs2] {
                        pc += sb_format(&instruction);
                        branch = true;
                    };
                }
                0x01 => {
                    if reg[rs1] != reg[rs2] {
                        pc += sb_format(&instruction);
                        branch = true;
                    };
                }
                0x04 => {
                    if reg[rs1] < reg[rs2] {
                        pc += sb_format(&instruction);
                        branch = true;
                    };
                }
                0x05 => {
                    if reg[rs1] >= reg[rs2] {
                        pc += sb_format(&instruction);
                        branch = true;
                    };
                }
                0x06 => {
                    if (reg[rs1] as u32) < (reg[rs2] as u32) {
                        pc += sb_format(&instruction);
                        branch = true;
                    };
                }
                0x07 => {
                    if (reg[rs1] as u32) >= (reg[rs2] as u32) {
                        pc += sb_format(&instruction);
                        branch = true;
                    };
                }
                _ => (),
            },
            0x67 => match funct3 {
                0x00 => {
                    reg[rd] = pc as i32 + 4;
                    pc = (reg[rs1] + imm110) as usize;
                    branch = true;
                }
                _ => (),
            },
            0x6F => {
                reg[rd] = (pc + 4) as i32;
                pc = pc + uj_format(&instruction);
                branch = true;
            }
            0x73 => {
                if reg[17] == 10 {
                    break;
                }
            }
            _ => (),
        }

        reg[0] = 0;
        
        if !branch {
            pc += 4;
        }

        branch = false;

        if pc >= *program_len {
            break;
        }
    }    
}

pub fn convert_to_instruction(bytes: &[u8]) -> i32 {
    let instruction: [u8; 4] = [bytes[3], bytes[2], bytes[1], bytes[0]];
    let next = i32::from_be_bytes(instruction);
    next
}

pub fn uj_format(instruction: &i32) -> usize {
    let bit20: i32 = (instruction >> 31) << 20; // 20
    let bit101: i32 = ((instruction >> 21) & 0x3ff) << 1; // 10 9 8 7 6 5 4 3 2 1
    let bit1912 = instruction & 0xff000; // 19 18 17 16 15 14 13 12
    let bit11 = (instruction & 0x100000) >> 9; // 11
    (((bit101 | bit1912) | bit11) | bit20) as usize
}

pub fn sb_format(instruction: &i32) -> usize {
    let bit11 = (instruction & 0x80) << 4; // 11
    let bit12 = (instruction >> 31) << 12; // 12
    let bit41 = ((instruction >> 8) & 0x0f) << 1; // 4 3 2 1
    let bit105 = ((instruction >> 25) & 0x3f) << 5; // 10 9 8 7 6 5
    (((bit41 | bit105) | bit11) | bit12) as usize
}

pub fn s_format(instruction: &i32) -> usize {
    let bit40 = (instruction >> 7) & 0x1f;
    let bit115 = (instruction >> 25) << 5;
    (bit40 | bit115) as usize
}