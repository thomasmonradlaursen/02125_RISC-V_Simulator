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
        // TODO - Fix to something nicer...
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
            unimplemented => {
                return format!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                )
            }
        },

        0x03 => match funct3 {
            0x00 => {
                return format!("LB x{}, {}(x{})", rd, imm110, rs1);
            }
            0x01 => {
                return format!("LH x{}, {}(x{})", rd, imm110, rs1);
            }
            0x02 => {
                return format!("LW x{}, {}(x{})", rd, imm110, rs1);
            }
            0x04 => {
                return format!("LBU x{}, {}(x{})", rd, imm110, rs1);
            }
            0x05 => {
                return format!("LHU x{}, {}(x{})", rd, imm110, rs1);
            }
            unimplemented => {
                return format!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                )
            }
        },
        0x13 => match funct3 {
            0x00 => {
                return format!("ADDI x{}, x{}, {}", rd, rs1, imm110);
            }
            0x01 => {
                return format!("SLLI x{}, x{}, {}", rd, rs1, shamt);
            }
            0x02 => {
                return format!("SLTI x{}, x{}, {}", rd, rs1, imm110);
            }
            0x03 => {
                return format!("SLTIU x{}, x{}, {}", rd, rs1, imm110);
            }
            0x04 => {
                return format!("XORI x{}, x{}, {}", rd, rs1, imm110);
            }
            0x05 => match funct7 {
                0x00 => {
                    return format!("SRLI x{}, x{}, {}", rd, rs1, shamt);
                }
                0x20 => {
                    return format!("SRAI x{}, x{}, {}", rd, rs1, shamt);
                }
                unimplemented => {
                    return format!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, funct3, opcode
                    )
                }
            },
            0x06 => {
                return format!("ORI x{}, x{}, {}", rd, rs1, imm110);
            }
            0x07 => {
                return format!("ANDI x{}, x{}, {}", rd, rs1, imm110);
            }
            unimplemented => {
                return format!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                )
            }
        },
        0x17 => {
            return format!("AUIPC x{}, {}", rd, imm3112);
        }
        0x23 => match funct3 {
            0x00 => {
                let offset = s_format(&instruction);
                return format!("SB x{}, {}(x{})", rs2, offset, rs1);
            }
            0x01 => {
                let offset = s_format(&instruction);
                return format!("SH x{}, {}(x{})", rs2, offset, rs1);
            }
            0x02 => {
                let offset = s_format(&instruction);
                return format!("SW x{}, {}(x{})", rs2, offset, rs1);
            }
            unimplemented => {
                return format!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                )
            }
        },
        0x33 => match funct3 {
            0x00 => match funct7 {
                0x00 => {
                    return format!("ADD x{}, x{}, x{}", rd, rs1, rs2);
                }
                0x20 => {
                    return format!("SUB x{}, x{}, x{}", rd, rs1, rs2);
                }
                unimplemented => {
                    return format!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, funct3, opcode
                    )
                }
            },
            0x01 => {
                return format!("SLL x{}, x{}, x{}", rd, rs1, rs2);
            }
            0x02 => {
                return format!("SLT x{}, x{}, x{}", rd, rs1, rs2);
            }
            0x03 => {
                return format!("SLTIU x{}, x{}, x{}", rd, rs1, rs2);
            }
            0x04 => {
                return format!("XOR x{}, x{}, x{}", rd, rs1, rs2);
            }
            0x05 => match funct7 {
                0x00 => {
                    return format!("SRL x{}, x{}, x{}", rd, rs1, rs2);
                }
                0x20 => {
                    return format!("SRA x{}, x{}, x{}", rd, rs1, rs2);
                }
                unimplemented => {
                    return format!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, funct3, opcode
                    )
                }
            },
            0x06 => {
                return format!("OR x{}, x{}, x{}", rd, rs1, rs2);
            }
            0x07 => {
                return format!("AND x{}, x{}, x{}", rd, rs1, rs2);
            }
            unimplemented => {
                return format!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                )
            }
        },
        0x37 => {
            return format!("LUI x{}, {}", rd, imm3112);
        }
        0x63 => match funct3 {
            0x00 => {
                return format!("BEQ x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
            }
            0x01 => {
                return format!("BNE x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
            }
            0x04 => {
                return format!("BLT x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
            }
            0x05 => {
                return format!("BGE x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
            }
            0x06 => {
                return format!("BLTU x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
            }
            0x07 => {
                return format!("BGEU x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
            }
            unimplemented => {
                return format!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                )
            }
        },
        0x67 => match funct3 {
            0x00 => {
                return format!("JALR x{}, x{}, {}", rd, rs1, imm110);
            }
            unimplemented => {
                return format!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                )
            }
        },
        0x6F => {
            return format!("JAL x{}, {}", rd, uj_format(&instruction));
        }
        0x73 => {
            return format!("ECALL");
        }
        unimplemented => return format!("Opcode {:#02x} not implemented...", unimplemented),
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

pub fn print_mem_instructions(mem: &[u8], len: &usize) {
    let mut count = 0;
    while count < *len {
        print!("{:08b} ", mem[count]);
        count = count + 1;
        if count % 4 == 0 {
            println!();
        }
    }
}

pub fn print_registers(registers: &[i32; 32]) {
    let mut count = 0;
    for register in registers {
        println!("Reg[{:>2}]: {:>10}", count, register);
        count += 1;
    }
}

pub fn print_registers_as_char(registers: &[i32; 32]) {
    let mut count = 0;
    for register in registers {
        println!("Reg[{:>2}]: {:?}", count, (*register as u8) as char);
        count += 1;
    }
}

pub fn print_registers_not_zero(registers: &[i32; 32]) {
    let mut count = 0;
    let zero = 0;
    println!("Registers of none-zero value:");
    for register in registers {
        if *register != zero {
            println!("Reg[{:>2}]: {:>5}", count, register);
        }
        count += 1;
    }
    println!();
}

pub fn print_program_info(filename: &String, program_len: &usize) {
    println!("______________________________________");
    println!("Acorn - RISC-V Pipeline Simulator");
    println!("Binary from file: {}", filename);
    println!("Length of binary: {}", program_len);
    println!("______________________________________");
}

pub fn instructions_as_assembly(bytes: &Vec<u8>) -> Vec<String> {
    let mut instructions: Vec<String> = vec![];
    let mut i = 0;
    while i < bytes.len() {
        let bytes: [u8;4] = [bytes[i], bytes[i+1], bytes[i+2], bytes[i+3]];
        instructions.push(to_assembly(&i32::from_le_bytes(bytes)));
        i += 4;
    }
    instructions
}
