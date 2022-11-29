pub fn to_assembly(instruction: &i32) -> String {
    let opcode = instruction & 0x7f;
    let funct3 = (instruction >> 12) & 0x07;
    let funct7 = instruction >> 25;
    let rd = (instruction >> 7) & 0x01f;
    let rs1 = (instruction >> 15) & 0x01f;
    let rs2 = (instruction >> 20) & 0x01f;
    let imm3112 = (instruction >> 12) << 12;
    let imm110 = instruction >> 20;
    let shamt = (instruction >> 20) & 0x01f;

    match opcode {
        0x00 => match funct3 {
            0x00 => {
                return format!("");
            }
            0x01 => {
                return format!("flush");
            }
            0x02 => {
                return format!("");
            }
            0x03 => {
                return format!("stall");
            }
            0x04 => {
                return format!("");
            }
            _ => {
                return format!("Unknown instruction")
            }
        },

        0x03 => match funct3 {
            0x00 => {
                return format!("lb x{} {}(x{})", rd, imm110, rs1);
            }
            0x01 => {
                return format!("lh x{} {}(x{})", rd, imm110, rs1);
            }
            0x02 => {
                return format!("lw x{} {}(x{})", rd, imm110, rs1);
            }
            0x04 => {
                return format!("lbu x{} {}(x{})", rd, imm110, rs1);
            }
            0x05 => {
                return format!("lhu x{} {}(x{})", rd, imm110, rs1);
            }
            _ => {
                return format!("Unknown instruction")
            }
        },
        0x13 => match funct3 {
            0x00 => {
                return format!("addi x{} x{} {}", rd, rs1, imm110);
            }
            0x01 => {
                return format!("slli x{} x{} {}", rd, rs1, shamt);
            }
            0x02 => {
                return format!("slti x{} x{} {}", rd, rs1, imm110);
            }
            0x03 => {
                return format!("sltiu x{} x{} {}", rd, rs1, imm110);
            }
            0x04 => {
                return format!("xori x{} x{} {}", rd, rs1, imm110);
            }
            0x05 => match funct7 {
                0x00 => {
                    return format!("slri x{} x{} {}", rd, rs1, shamt);
                }
                0x20 => {
                    return format!("srai x{} x{} {}", rd, rs1, shamt);
                }
                _ => {
                    return format!("Unknown instruction")
                }
            },
            0x06 => {
                return format!("ori x{} x{} {}", rd, rs1, imm110);
            }
            0x07 => {
                return format!("andi x{} x{} {}", rd, rs1, imm110);
            }
            _ => {
                return format!("Unknown instruction")
            }
        },
        0x17 => {
            return format!("auipc x{} {}", rd, imm3112);
        }
        0x23 => match funct3 {
            0x00 => {
                let offset = s_format(&instruction);
                return format!("sb x{} {}(x{})", rs2, offset, rs1);
            }
            0x01 => {
                let offset = s_format(&instruction);
                return format!("sh x{} {}(x{})", rs2, offset, rs1);
            }
            0x02 => {
                let offset = s_format(&instruction);
                return format!("sw x{} {}(x{})", rs2, offset, rs1);
            }
            _ => {
                return format!("Unknown instruction")
            }
        },
        0x33 => match funct3 {
            0x00 => match funct7 {
                0x00 => {
                    return format!("add x{} x{} x{}", rd, rs1, rs2);
                }
                0x20 => {
                    return format!("sub x{} x{} x{}", rd, rs1, rs2);
                }
                _ => {
                    return format!("Unknown instruction")
                }
            },
            0x01 => {
                return format!("sll x{} x{} x{}", rd, rs1, rs2);
            }
            0x02 => {
                return format!("slt x{} x{} x{}", rd, rs1, rs2);
            }
            0x03 => {
                return format!("sltu x{} x{} x{}", rd, rs1, rs2);
            }
            0x04 => {
                return format!("xor x{} x{} x{}", rd, rs1, rs2);
            }
            0x05 => match funct7 {
                0x00 => {
                    return format!("srl x{} x{} x{}", rd, rs1, rs2);
                }
                0x20 => {
                    return format!("sra x{} x{} x{}", rd, rs1, rs2);
                }
                _ => {
                    return format!("Unknown instruction")
                }
            },
            0x06 => {
                return format!("or x{} x{} x{}", rd, rs1, rs2);
            }
            0x07 => {
                return format!("and x{} x{} x{}", rd, rs1, rs2);
            }
            _ => {
                return format!("Unknown instruction")
            }
        },
        0x37 => {
            return format!("lui x{} {}", rd, imm3112);
        }
        0x63 => match funct3 {
            0x00 => {
                return format!("beq x{} x{} {}", rs1, rs2, sb_format(&instruction));
            }
            0x01 => {
                return format!("bne x{} x{} {}", rs1, rs2, sb_format(&instruction));
            }
            0x04 => {
                return format!("blt x{} x{} {}", rs1, rs2, sb_format(&instruction));
            }
            0x05 => {
                return format!("bge x{} x{} {}", rs1, rs2, sb_format(&instruction));
            }
            0x06 => {
                return format!("bltu x{} x{} {}", rs1, rs2, sb_format(&instruction));
            }
            0x07 => {
                return format!("bgeu x{} x{} {}", rs1, rs2, sb_format(&instruction));
            }
            _ => {
                return format!("Unknown instruction")
            }
        },
        0x67 => match funct3 {
            0x00 => {
                return format!("jalr x{} x{} {}", rd, rs1, imm110);
            }
            _ => {
                return format!("Unknown instruction")
            }
        },
        0x6F => {
            return format!("jal x{} {}", rd, uj_format(&instruction));
        }
        0x73 => {
            return format!("ecall");
        }
        _ => return format!("Unknown instruction"),
    }
}

fn uj_format(instruction: &i32) -> i32 {
    let bit20: i32 = (instruction >> 31) << 20; // 20
    let bit101: i32 = ((instruction >> 21) & 0x3ff) << 1; // 10 9 8 7 6 5 4 3 2 1
    let bit1912: i32 = instruction & 0xff000; // 19 18 17 16 15 14 13 12
    let bit11: i32 = (((instruction & 0x100000) as u32) >> 9) as i32; // 11
    ((bit101 | bit1912) | bit11) | bit20
}

fn sb_format(instruction: &i32) -> i32 {
    let bit11 = (instruction & 0x80) << 4; // 11
    let bit12 = (instruction >> 31) << 12; // 12
    let bit41 = ((instruction >> 8) & 0x0f) << 1; // 4 3 2 1
    let bit105 = ((instruction >> 25) & 0x3f) << 5; // 10 9 8 7 6 5
    ((bit41 | bit105) | bit11) | bit12
}

fn s_format(instruction: &i32) -> i32 {
    let bit40 = (instruction >> 7) & 0x1f;
    let bit115 = (instruction >> 25) << 5;
    bit40 | bit115
}

pub fn instructions_as_assembly(bytes: &Vec<u8>) -> Vec<String> {
    let mut instructions: Vec<String> = vec![];
    let mut i = 0;
    if bytes.len() > 0 {
        while i < bytes.len()-4 {
            let bytes: [u8; 4] = [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]];
            instructions.push(to_assembly(&i32::from_le_bytes(bytes)));
            i += 4;
        }
    }
    instructions
}
