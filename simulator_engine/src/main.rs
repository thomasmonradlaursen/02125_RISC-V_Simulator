use std::env;
use std::fs;

fn main() {
    let reg: [i32; 32] = [0; 32];

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut mem: [u8; 1048576] = [0; 1048576];
    let len = read_bytes_to_mem(filename, &mut mem);

    print_mem_instructions(&mem, &len);

    print_instructions(&mem, &len);
    //print_instructions(&instructions);

    decode(&mem, &len);

    let res = simulate(reg, mem, &len);

    print_registers(&res);

    //print_registers_as_char(&res);
}

fn simulate(mut reg: [i32; 32], mut mem: [u8; 1048576], program_len: &usize) -> [i32; 32] {
    println!("Hello Rust RISC-V world!");

    let mut pc: usize = 0;

    let next_index = 16;

    let mut branch: bool = false;

    loop {
        let instruction = convert_to_instruction(&mem[(pc >> 2)..((pc >> 2) + 4)]);
        let opcode = instruction & 0x7f;
        let funct3 = (instruction >> 12) & 0x07;
        let funct7 = instruction >> 25;
        let rd = ((instruction >> 7) & 0x01f) as usize;
        let rs1 = ((instruction >> 15) & 0x01f) as usize;
        let rs2 = ((instruction >> 20) & 0x01f) as usize;
        let imm3112 = (instruction >> 12) << 12;
        let imm110 = (instruction >> 20);
        let shamt = (instruction >> 20) & 0x01f;

        match opcode {
            0x03 => match funct3 {
                0x00 => {
                    println!("LB x{}, {}(x{})", rd, imm110, rs1);
                    reg[rd] = mem[(reg[rs1] + imm110) as usize] as i32;
                }
                0x01 => {
                    println!("LH x{}, {}(x{})", rd, imm110, rs1);
                    let index = (reg[rs1] + imm110) as usize;
                    let short: [u8; 4] = [mem[index], mem[index + 1], 0, 0];
                    reg[rd] = i32::from_be_bytes(short);
                }
                0x02 => {
                    println!("LW x{}, {}(x{})", rd, imm110, rs1);
                    let index = (reg[rs1] + imm110) as usize;
                    let integer: [u8; 4] =
                        [mem[index], mem[index + 1], mem[index + 2], mem[index + 3]];
                    reg[rd] = i32::from_be_bytes(integer);
                }
                0x04 => {
                    println!("LHU x{}, {}(x{})", rd, imm110, rs1);
                    let index = (reg[rs1] + imm110) as usize;
                    let short: [u8; 4] = [mem[index], mem[index + 1], 0, 0];
                    reg[rd] = u32::from_be_bytes(short) as i32;
                }
                0x05 => {
                    println!("LWU x{}, {}(x{})", rd, imm110, rs1);
                    let index = (reg[rs1] + imm110) as usize;
                    let integer: [u8; 4] =
                        [mem[index], mem[index + 1], mem[index + 2], mem[index + 3]];
                    reg[rd] = u32::from_be_bytes(integer) as i32;
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x13 => match funct3 {
                0x00 => {
                    println!("ADDI x{}, x{}, {}", rd, rs1, imm110);
                    reg[rd] = reg[rs1] + imm110;
                }
                0x01 => {
                    println!("SLLI x{}, x{}, {}", rd, rs1, shamt);
                    reg[rd] = reg[rs1] << shamt;
                }
                0x02 => {
                    println!("SLTI x{}, x{}, {}", rd, rs1, imm110);
                    if reg[rs1] < imm110 {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }
                }
                0x03 => {
                    println!("SLTIU x{}, x{}, {}", rd, rs1, imm110);
                    if (reg[rs1] as u32) < (imm110 as u32) {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }
                }
                0x04 => {
                    println!("XORI x{}, x{}, {}", rd, rs1, imm110);
                    reg[rd] = reg[rs1] ^ imm110;
                }
                // TODO:
                0x05 => match funct7 {
                    0x00 => {
                        println!("SRLI x{}, x{}, {}", rd, rs1, shamt);
                        reg[rd] = ((reg[rs1] as u32) >> (shamt as u32)) as i32;
                    }
                    0x20 => {
                        println!("SRAI x{}, x{}, {}", rd, rs1, shamt);
                        reg[rd] = reg[rs1] >> shamt;
                    }
                    unimplemented => println!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, funct3, opcode
                    ),
                },
                0x06 => {
                    println!("ORI x{}, x{}, {}", rd, rs1, imm110);
                    reg[rd] = reg[rs1] | imm110;
                }
                0x07 => {
                    println!("ANDI x{}, x{}, {}", rd, rs1, imm110);
                    reg[rd] = reg[rs1] & imm110;
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x17 => {
                println!("AUIPC x{}, {}", rd, imm3112);
                reg[rd] = pc as i32 + imm3112;
            }
            0x23 => match funct3 {
                0x00 => {
                    let offset = s_format(&instruction);
                    println!("SB x{}, {}(x{})", rd, offset, rs1);
                    let bytes = i32::to_be_bytes(reg[rs2]);
                    mem[reg[rs1] as usize + offset as usize] = bytes[0];
                }
                0x01 => {
                    let offset = s_format(&instruction);
                    println!("SH x{}, {}(x{})", rd, offset, rs1);
                    let bytes = i32::to_be_bytes(reg[rs2]);
                    mem[reg[rs1] as usize + offset as usize] = bytes[0];
                    mem[reg[rs1] as usize + offset as usize + 1] = bytes[1];
                }
                0x02 => {
                    let offset = s_format(&instruction);
                    println!("SW x{}, {}(x{})", rd, offset, rs1);
                    let bytes = i32::to_be_bytes(reg[rs2]);
                    mem[reg[rs1] as usize + offset as usize] = bytes[0];
                    mem[reg[rs1] as usize + offset as usize + 1] = bytes[1];
                    mem[reg[rs1] as usize + offset as usize + 2] = bytes[2];
                    mem[reg[rs1] as usize + offset as usize + 3] = bytes[3];
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x33 => match funct3 {
                0x00 => match funct7 {
                    0x00 => {
                        println!("ADD x{}, x{}, x{}", rd, rs1, rs2);
                        reg[rd] = reg[rs1] + reg[rs2];
                    }
                    0x20 => {
                        println!("SUB x{}, x{}, x{}", rd, rs1, rs2);
                        reg[rd] = reg[rs1] - reg[rs2];
                    }
                    unimplemented => println!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, funct3, opcode
                    ),
                },
                0x01 => {
                    println!("SLL x{}, x{}, x{}", rd, rs1, rs2);
                    reg[rd] = reg[rs1] << reg[rs2];
                }
                0x02 => {
                    println!("SLT x{}, x{}, x{}", rd, rs1, rs2);
                    if reg[rs1] < reg[rs2] {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }
                }
                0x03 => {
                    println!("SLTIU x{}, x{}, x{}", rd, rs1, rs2);
                    if (reg[rs1] as u32) < (reg[rs2] as u32) {
                        reg[rd] = 1;
                    } else {
                        reg[rd] = 0;
                    }
                }
                0x04 => {
                    println!("XOR x{}, x{}, x{}", rd, rs1, rs2);
                    reg[rd] = reg[rs1] ^ reg[rs2];
                }
                0x05 => match funct7 {
                    0x00 => {
                        println!("SRL x{}, x{}, x{}", rd, rs1, rs2);
                        reg[rd] = ((reg[rs1] as u32) >> (reg[rs2] as u32)) as i32;
                    }
                    0x20 => {
                        println!("SRA x{}, x{}, x{}", rd, rs1, rs2);
                        reg[rd] = reg[rs1] >> reg[rs2];
                    }
                    unimplemented => println!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, funct3, opcode
                    ),
                },
                0x06 => {
                    println!("OR x{}, x{}, x{}", rd, rs1, rs2);
                    reg[rd] = reg[rs1] | reg[rs2];
                }
                0x07 => {
                    println!("AND x{}, x{}, x{}", rd, rs1, rs2);
                    reg[rd] = reg[rs1] & reg[rs2];
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x37 => {
                println!("LUI x{}, {}", rd, imm3112);
                reg[rd] = imm3112;
            }
            0x63 => match funct3 {
                0x00 => {
                    println!("BEQ x{}, x{}, {}", rs1, rs2, sb_format(&instruction) * 4);
                    if reg[rs1] == reg[rs2] {
                        pc += (sb_format(&instruction) * 4) as usize;
                        branch = true;
                    };
                }
                0x01 => {
                    println!("BNE x{}, x{}, {}", rs1, rs2, sb_format(&instruction) * 4);
                    if reg[rs1] != reg[rs2] {
                        pc += (sb_format(&instruction) * 4) as usize;
                        branch = true;
                    };
                }
                0x04 => {
                    println!("BLT x{}, x{}, {}", rs1, rs2, sb_format(&instruction) * 4);
                    if reg[rs1] < reg[rs2] {
                        pc += (sb_format(&instruction) * 4) as usize;
                        branch = true;
                    };
                }
                0x05 => {
                    println!("BGE x{}, x{}, {}", rs1, rs2, sb_format(&instruction) * 4);
                    if reg[rs1] >= reg[rs2] {
                        pc += (sb_format(&instruction) * 4) as usize;
                        branch = true;
                    };
                }
                0x06 => {
                    println!("BLTU x{}, x{}, {}", rs1, rs2, sb_format(&instruction) * 4);
                    if (reg[rs1] as u32) < (reg[rs2] as u32) {
                        pc += (sb_format(&instruction) * 4) as usize;
                        branch = true;
                    };
                }
                0x07 => {
                    println!("BGEU x{}, x{}, {}", rs1, rs2, sb_format(&instruction) * 4);
                    if (reg[rs1] as u32) >= (reg[rs2] as u32) {
                        pc += (sb_format(&instruction) * 4) as usize;
                        branch = true;
                    };
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x67 => match funct3 {
                0x00 => {
                    println!("JALR x{}, x{}, {}", rd, rs1, imm110);
                    reg[rd] = pc as i32 + next_index;
                    pc = (reg[rs1] + imm110 * 4) as usize;
                    branch = true;
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x6F => {
                println!("JAL x{}, {}", rd, uj_format(&instruction));
                reg[rd] = (pc + next_index as usize) as i32;
                pc = pc + (uj_format(&instruction) * 4) as usize;
                branch = true;
            }
            0x73 => {
                println!("ECALL");
                if reg[17] == 10 {
                    break;
                }
            }
            unimplemented => println!("Opcode {:#02x} not implemented...", unimplemented),
        }

        reg[0] = 0;

        //print_registers_not_zero(&reg);

        if !branch {
            pc += next_index as usize;
        } else {
            branch = false;
        }

        if (pc >> 2) >= *program_len {
            break;
        }
    }

    println!("Program exit");

    reg
}

fn read_bytes_to_mem(filename: &String, mem: &mut [u8; 1048576]) -> usize {
    let content = fs::read(filename).expect("File not found");
    let mut count = 0;
    while count < content.len() {
        mem[count] = content[count];
        count = count + 1;
    }
    println!("Bytes have been loaded to memory");
    content.len()
}

fn convert_to_instruction(bytes: &[u8]) -> i32 {
    let instruction: [u8; 4] = [bytes[3], bytes[2], bytes[1], bytes[0]];
    let next = i32::from_be_bytes(instruction);
    next
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
    let bit41 =  ((instruction >> 8) & 0x0f) << 1; // 4 3 2 1
    let bit105 =  ((instruction >> 25) & 0x3f) << 5; // 10 9 8 7 6 5
    ((bit41 | bit105) | bit11) | bit12
}

fn s_format(instruction: &i32) -> i32 {
    let bit40 = (instruction >> 7) & 0x1f;
    let bit115 = (instruction >> 25) << 5;
    bit40 | bit115
}

fn print_mem_instructions(mem: &[u8], len: &usize) {
    let mut count = 0;
    while count < *len {
        print!("{:08b} ", mem[count]);
        count = count + 1;
        if count % 4 == 0 {
            println!();
        }
    }
}

fn print_instructions(mem: &[u8], len: &usize) {
    let mut count = 0;
    while count < *len {
        println!("{:032b}", convert_to_instruction(&mem[count..count + 4]));
        count = count + 4;
    }
}

fn print_registers(registers: &[i32; 32]) {
    let mut count = 0;
    for register in registers {
        println!("Reg[{:>2}]: {:>10}", count, register);
        count += 1;
    }
}

fn print_registers_as_char(registers: &[i32; 32]) {
    let mut count = 0;
    for register in registers {
        println!("Reg[{:>2}]: {:?}", count, (*register as u8) as char);
        count += 1;
    }
}

fn print_registers_not_zero(registers: &[i32; 32]) {
    let mut count = 0;
    let zero = 0;
    for register in registers {
        if *register != zero {
            println!("Reg[{:>2}]: {:>5}", count, register);
        }
        count += 1;
    }
    println!("___")
}

fn decode(mem: &[u8; 1048576], program_len: &usize) {
    println!("Converting binary to assembly");

    let mut pc: usize = 0;

    let next_index = 16;

    let mut branch: bool = false;

    loop {
        let instruction = convert_to_instruction(&mem[(pc >> 2)..((pc >> 2) + 4)]);
        let opcode = instruction & 0x7f;
        let funct3 = (instruction >> 12) & 0x07;
        let funct7 = instruction >> 25;
        let rd = (instruction >> 7) & 0x01f;
        let rs1 = (instruction >> 15) & 0x01f;
        let rs2 = (instruction >> 20) & 0x01f;
        let imm3112 = (instruction >> 12) << 12;
        let imm110 = (instruction >> 20);
        let shamt = (instruction >> 20) & 0x01f;

        match opcode {
            0x03 => match funct3 {
                0x00 => {
                    println!("LB x{}, {}(x{})", rd, imm110, rs1);
                }
                0x01 => {
                    println!("LH x{}, {}(x{})", rd, imm110, rs1);
                }
                0x02 => {
                    println!("LW x{}, {}(x{})", rd, imm110, rs1);
                }
                0x04 => {
                    println!("LHU x{}, {}(x{})", rd, imm110, rs1);
                }
                0x05 => {
                    println!("LWU x{}, {}(x{})", rd, imm110, rs1);
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x13 => match funct3 {
                0x00 => {
                    println!("ADDI x{}, x{}, {}", rd, rs1, imm110);
                }
                0x01 => {
                    println!("SLLI x{}, x{}, {}", rd, rs1, shamt);
                }
                0x02 => {
                    println!("SLTI x{}, x{}, {}", rd, rs1, imm110);
                }
                0x03 => {
                    println!("SLTIU x{}, x{}, {}", rd, rs1, imm110);
                }
                0x04 => {
                    println!("XORI x{}, x{}, {}", rd, rs1, imm110);
                }
                // TODO:
                0x05 => match funct7 {
                    0x00 => {
                        println!("SRLI x{}, x{}, {}", rd, rs1, shamt);
                    }
                    0x20 => {
                        println!("SRAI x{}, x{}, {}", rd, rs1, shamt);
                    }
                    unimplemented => println!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, funct3, opcode
                    ),
                },
                0x06 => {
                    println!("ORI x{}, x{}, {}", rd, rs1, imm110);
                }
                0x07 => {
                    println!("ANDI x{}, x{}, {}", rd, rs1, imm110);
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x17 => {
                println!("AUIPC x{}, {}", rd, imm3112);
            }
            0x23 => match funct3 {
                0x00 => {
                    let offset = s_format(&instruction);
                    println!("SB x{}, {}(x{})", rd, offset, rs1);
                }
                0x01 => {
                    let offset = s_format(&instruction);
                    println!("SH x{}, {}(x{})", rd, offset, rs1);
                }
                0x02 => {
                    let offset = s_format(&instruction);
                    println!("SW x{}, {}(x{})", rd, offset, rs1);
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x33 => match funct3 {
                0x00 => match funct7 {
                    0x00 => {
                        println!("ADD x{}, x{}, x{}", rd, rs1, rs2);
                    }
                    0x20 => {
                        println!("SUB x{}, x{}, x{}", rd, rs1, rs2);
                    }
                    unimplemented => println!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, funct3, opcode
                    ),
                },
                0x01 => {
                    println!("SLL x{}, x{}, x{}", rd, rs1, rs2);
                }
                0x02 => {
                    println!("SLT x{}, x{}, x{}", rd, rs1, rs2);
                }
                0x03 => {
                    println!("SLTIU x{}, x{}, x{}", rd, rs1, rs2);
                }
                0x04 => {
                    println!("XOR x{}, x{}, x{}", rd, rs1, rs2);
                }
                0x05 => match funct7 {
                    0x00 => {
                        println!("SRL x{}, x{}, x{}", rd, rs1, rs2);
                    }
                    0x20 => {
                        println!("SRA x{}, x{}, x{}", rd, rs1, rs2);
                    }
                    unimplemented => println!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, funct3, opcode
                    ),
                },
                0x06 => {
                    println!("OR x{}, x{}, x{}", rd, rs1, rs2);
                }
                0x07 => {
                    println!("AND x{}, x{}, x{}", rd, rs1, rs2);
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x37 => {
                println!("LUI x{}, {}", rd, imm3112);
            }
            0x63 => match funct3 {
                0x00 => {
                    println!("BEQ x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
                }
                0x01 => {
                    println!("BNE x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
                }
                0x04 => {
                    println!("BLT x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
                }
                0x05 => {
                    println!("BGE x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
                }
                0x06 => {
                    println!("BLTU x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
                }
                0x07 => {
                    println!("BGEU x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x67 => match funct3 {
                0x00 => {
                    println!("JALR x{}, x{}, {}", rd, rs1, imm110);
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x6F => {
                println!("JAL x{}, {}", rd, uj_format(&instruction));
            }
            0x73 => {
                println!("ECALL");
            }
            unimplemented => println!("Opcode {:#02x} not implemented...", unimplemented),
        }

        //print_registers_not_zero(&reg);

        if !branch {
            pc += next_index as usize;
        } else {
            branch = false;
        }

        if (pc >> 2) >= *program_len {
            break;
        }
    }

    println!("Converted to assembly");

}
