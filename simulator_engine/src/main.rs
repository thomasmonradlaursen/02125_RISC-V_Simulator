use std::env;
use std::fs;

fn main() {
    let reg: [i32; 32] = [0; 32];

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let instructions = convert_to_instructions(&read_bytes(filename));

    //print_instructions(&instructions);

    let res = simulate(reg, instructions);

    print_registers(&res);
}

fn simulate(mut reg: [i32; 32], instructions: Vec<i32>) -> [i32; 32] {
    println!("Hello Rust RISC-V world!");

    let mut pc: i32 = 0;

    loop {
        let instruction = instructions[(pc >> 2) as usize];
        let opcode = instruction & 0x7f;
        let funct3 = (instruction >> 12) & 0x03;
        let funct7 = instruction >> 25;
        let rd = (instruction >> 7) & 0x01f;
        let rs1 = (instruction >> 15) & 0x01f;
        let rs2 = (instruction >> 20) & 0x01f;
        let imm3112 = (instruction >> 12) << 12;
        let imm110 = instruction >> 20;
        let shamt = (instruction >> 20) & 0x01f;

        match opcode {
            0x13 => match funct3 {
                0x00 => {
                    reg[rd as usize] = reg[rs1 as usize] + imm110;
                    println!("ADDI x{}, x{}, {}", rd, rs1, imm110);
                }
                0x01 => {
                    reg[rd as usize] = reg[rs1 as usize] << shamt;
                    println!("SLLI x{}, x{}, {}", rd, rs1, shamt);
                }
                0x02 => {
                    if reg[rs1 as usize] < imm110 {
                        reg[rd as usize] = 1;
                    } else {
                        reg[rd as usize] = 0;
                    }
                    println!("SLTI x{}, x{}, {}", rd, rs1, imm110);
                }
                0x03 => {
                    if (reg[rs1 as usize] as u32) < (imm110 as u32) {
                        reg[rd as usize] = 1;
                    } else {
                        reg[rd as usize] = 0;
                    }
                    println!("SLTIU x{}, x{}, {}", rd, rs1, imm110);
                }
                0x04 => {
                    reg[rd as usize] = reg[rs1 as usize] ^ imm110;
                    println!("XORI x{}, x{}, {}", rd, rs1, imm110);
                }
                // TODO:
                0x05 => match funct7 {
                    0x00 => {
                        reg[rd as usize] = ((reg[rs1 as usize] as u32) >> (shamt as u32)) as i32;
                        println!("SRLI x{}, x{}, {}", rd, rs1, shamt);
                    }
                    0x20 => {
                        reg[rd as usize] = reg[rs1 as usize] >> shamt;
                        println!("SRAI x{}, x{}, {}", rd, rs1, shamt);
                    }
                    unimplemented => println!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, funct3, opcode
                    ),
                },
                0x06 => {
                    reg[rd as usize] = reg[rs1 as usize] | imm110;
                    println!("ORI x{}, x{}, {}", rd, rs1, imm110);
                }
                0x07 => {
                    reg[rd as usize] = reg[rs1 as usize] & imm110;
                    println!("ANDI x{}, x{}, {}", rd, rs1, imm110);
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x17 => {
                reg[rd as usize] = pc + imm3112;
                println!("AUIPC x{}, {}", rd, imm3112);
            }
            0x33 => match funct3 {
                0x00 => match funct7 {
                    0x00 => {
                        reg[rd as usize] = reg[rs1 as usize] + reg[rs2 as usize];
                        println!("ADD x{}, x{}, x{}", rd, rs1, rs2);
                    }
                    0x20 => {
                        reg[rd as usize] = reg[rs1 as usize] - reg[rs2 as usize];
                        println!("SUB x{}, x{}, x{}", rd, rs1, rs2);
                    }
                    unimplemented => println!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, funct3, opcode
                    ),
                },
                0x01 => {
                    reg[rd as usize] = reg[rs1 as usize] << reg[rs2 as usize];
                    println!("SLL x{}, x{}, x{}", rd, rs1, rs2);
                }
                0x02 => {
                    if reg[rs1 as usize] < reg[rs2 as usize] {
                        reg[rd as usize] = 1;
                    } else {
                        reg[rd as usize] = 0;
                    }
                    println!("SLT x{}, x{}, x{}", rd, rs1, rs2);
                }
                0x03 => {
                    if (reg[rs1 as usize] as u32) < (reg[rs2 as usize] as u32) {
                        reg[rd as usize] = 1;
                    } else {
                        reg[rd as usize] = 0;
                    }
                    println!("SLTIU x{}, x{}, x{}", rd, rs1, rs2);
                }
                0x04 => {
                    reg[rd as usize] = reg[rs1 as usize] ^ reg[rs2 as usize];
                    println!("XOR x{}, x{}, x{}", rd, rs1, rs2);
                }
                0x05 => match funct7 {
                    0x00 => {
                        reg[rd as usize] = ((reg[rs1 as usize] as u32) >> (reg[rs2 as usize] as u32)) as i32;
                        println!("SRL x{}, x{}, x{}", rd, rs1, rs2);
                    }
                    0x20 => {
                        reg[rd as usize] = reg[rs1 as usize] >>  reg[rs2 as usize];
                        println!("SRA x{}, x{}, x{}", rd, rs1, rs2);
                    }
                    unimplemented => println!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, funct3, opcode
                    ),
                }
                0x06 => {
                    reg[rd as usize] = reg[rs1 as usize] | reg[rs2 as usize];
                    println!("OR x{}, x{}, x{}", rd, rs1, rs2);
                }
                0x07 => {
                    reg[rd as usize] = reg[rs1 as usize] & reg[rs2 as usize];
                    println!("AND x{}, x{}, x{}", rd, rs1, rs2);
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x37 => {
                reg[rd as usize] = imm3112;
                println!("LUI x{}, {}", rd, imm3112);
            }
            0x63 => match funct3 {
                0x00 => {
                    if reg[rs1 as usize] == reg[rs2 as usize] {
                        pc += sb_format(&instruction);
                    };
                    println!("BEQ x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
                }
                0x01 => {
                    if reg[rs1 as usize] != reg[rs2 as usize] {
                        pc += sb_format(&instruction);
                    };
                    println!("BNE x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
                }
                0x04 => {
                    if reg[rs1 as usize] < reg[rs2 as usize] {
                        pc += sb_format(&instruction);
                    };
                    println!("BLT x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
                }
                0x05 => {
                    if reg[rs1 as usize] >= reg[rs2 as usize] {
                        pc += sb_format(&instruction);
                    };
                    println!("BGE x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
                }
                0x06 => {
                    if (reg[rs1 as usize] as u32) < (reg[rs2 as usize] as u32) {
                        pc += sb_format(&instruction);
                    };
                    println!("BLTU x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
                }
                0x07 => {
                    if (reg[rs1 as usize] as u32) >= (reg[rs2 as usize] as u32) {
                        pc += sb_format(&instruction);
                    };
                    println!("BGEU x{}, x{}, {}", rs1, rs2, sb_format(&instruction));
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x67 => match funct3 {
                0x00 => {
                    reg[rd as usize] = pc + 4;
                    pc = reg[rs1 as usize] + imm110;
                    println!("JALR x{}, x{}, {}", rd, rs1, imm110);
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, opcode
                ),
            },
            0x6F => {
                reg[rd as usize] = pc + 4;
                pc = pc + uj_format(&instruction);
                println!("JAL x{}, {}", rd, uj_format(&instruction));
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
        pc += 4;
        if (pc >> 2) >= instructions.len() as i32 {
            break;
        }
    }

    println!("Program exit");

    reg
}

fn read_bytes(filename: &String) -> Vec<u8> {
    let content = fs::read(filename).expect("File not found");
    content
}

fn convert_to_instructions(bytes: &Vec<u8>) -> Vec<i32> {
    let mut instructions: Vec<i32> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let instruction: [u8; 4] = [bytes[i + 3], bytes[i + 2], bytes[i + 1], bytes[i]];
        let next = i32::from_be_bytes(instruction);
        instructions.push(next);
        i += 4;
    }
    instructions
}

fn uj_format(instruction: &i32) -> i32 {
    let bit20: i32 = (instruction >> 31) << 20; // 20
    let bit101: i32 = ((instruction >> 21) & 0x3ff) << 1; // 10 9 8 7 6 5 4 3 2 1
    let bit1912 = instruction & 0xff000; // 19 18 17 16 15 14 13 12
    let bit11 = (instruction & 0x100000) >> 9; // 11
    ((bit101 | bit1912) | bit11) | bit20
}

fn sb_format(instruction: &i32) -> i32 {
    let bit11 = (instruction & 0x80) << 4; // 11
    let bit12 = (instruction >> 31) << 12; // 12
    let bit41 = ((instruction >> 8) & 0x0f) << 1; // 4 3 2 1
    let bit105 = ((instruction >> 25) & 0x3f) << 5; // 10 9 8 7 6 5
    ((bit41 | bit105) | bit11) | bit12
}

fn print_instructions(instructions: &Vec<i32>) {
    for instruction in instructions {
        println!("{:032b}", instruction);
    }
}

fn print_registers(registers: &[i32; 32]) {
    let mut count = 0;
    for register in registers {
        println!("Reg[{:>2}]: {:>5}", count, register);
        count += 1;
    }
}
