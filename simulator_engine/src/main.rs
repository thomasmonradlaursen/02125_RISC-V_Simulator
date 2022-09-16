use mylib::decode::Decode;
use mylib::fetch::Fetch;
use mylib::printer;
use std::env;
use std::fs;
use std::io;

fn main() {
    let reg: [i32; 32] = [0; 32];

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut mem: [u8; 1048576] = [0; 1048576];
    let len = read_bytes_to_mem(filename, &mut mem);

    //print_mem_instructions(&mem, &len);
    //print_instructions(&instructions);

    let res = simulate(reg, mem, &len);

    print_registers(&res);

    //print_registers_as_char(&res);
}

fn simulate(mut reg: [i32; 32], mut mem: [u8; 1048576], program_len: &usize) -> [i32; 32] {
    println!("Hello Rust RISC-V world!");

    let mut fetch = Fetch {
        pc: 0,
        instruction: 0,
        next_instruction: 0,
    };

    let mut decode = Decode {
        instruction: 0,
        next_instruction: 0,
        opcode: 0,
        funct3: 0,
        funct7: 0,
        rd: 0,
        rs1: 0,
        rs2: 0,
        imm3112: 0,
        imm110: 0,
        shamt: 0,
        next_opcode: 0,
        next_funct3: 0,
        next_funct7: 0,
        next_rd: 0,
        next_rs1: 0,
        next_rs2: 0,
        next_imm3112: 0,
        next_imm110: 0,
        next_shamt: 0,
    };

    let stepwise = true;

    let mut branch: bool = false;

    loop {
        fetch.fetch_instruction(&mem[fetch.pc..(fetch.pc + 4)]);
        decode.instruction = fetch.next_instruction;
        decode.decode_instruction();

        match decode.opcode {
            0x03 => match decode.funct3 {
                0x00 => {
                    reg[decode.rd] = mem[(reg[decode.rs1] + decode.imm110) as usize] as i32;
                }
                0x01 => {
                    let index = (reg[decode.rs1] + decode.imm110) as usize;
                    let short: [u8; 4] = [mem[index], mem[index + 1], 0, 0];
                    reg[decode.rd] = i32::from_be_bytes(short);
                }
                0x02 => {
                    let index = (reg[decode.rs1] + decode.imm110) as usize;
                    let integer: [u8; 4] =
                        [mem[index], mem[index + 1], mem[index + 2], mem[index + 3]];
                    reg[decode.rd] = i32::from_be_bytes(integer);
                }
                0x04 => {
                    let index = (reg[decode.rs1] + decode.imm110) as usize;
                    let short: [u8; 4] = [mem[index], mem[index + 1], 0, 0];
                    reg[decode.rd] = u32::from_be_bytes(short) as i32;
                }
                0x05 => {
                    let index = (reg[decode.rs1] + decode.imm110) as usize;
                    let integer: [u8; 4] =
                        [mem[index], mem[index + 1], mem[index + 2], mem[index + 3]];
                    reg[decode.rd] = u32::from_be_bytes(integer) as i32;
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, decode.opcode
                ),
            },
            0x13 => match decode.funct3 {
                0x00 => {
                    reg[decode.rd] = reg[decode.rs1] + decode.imm110;
                }
                0x01 => {
                    reg[decode.rd] = reg[decode.rs1] << decode.shamt;
                }
                0x02 => {
                    if reg[decode.rs1] < decode.imm110 {
                        reg[decode.rd] = 1;
                    } else {
                        reg[decode.rd] = 0;
                    }
                }
                0x03 => {
                    if (reg[decode.rs1] as u32) < (decode.imm110 as u32) {
                        reg[decode.rd] = 1;
                    } else {
                        reg[decode.rd] = 0;
                    }
                }
                0x04 => {
                    reg[decode.rd] = reg[decode.rs1] ^ decode.imm110;
                }
                // TODO:
                0x05 => match decode.funct7 {
                    0x00 => {
                        reg[decode.rd] = ((reg[decode.rs1] as u32) >> (decode.shamt as u32)) as i32;
                    }
                    0x20 => {
                        reg[decode.rd] = reg[decode.rs1] >> decode.shamt;
                    }
                    unimplemented => println!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, decode.funct3, decode.opcode
                    ),
                },
                0x06 => {
                    reg[decode.rd] = reg[decode.rs1] | decode.imm110;
                }
                0x07 => {
                    reg[decode.rd] = reg[decode.rs1] & decode.imm110;
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, decode.opcode
                ),
            },
            0x17 => {
                reg[decode.rd] = fetch.pc as i32 + decode.imm3112;
            }
            0x23 => match decode.funct3 {
                0x00 => {
                    let offset = s_format(&fetch.instruction);
                    let bytes = i32::to_be_bytes(reg[decode.rs2]);
                    mem[reg[decode.rs1] as usize + offset as usize] = bytes[0];
                }
                0x01 => {
                    let offset = s_format(&fetch.instruction);
                    let bytes = i32::to_be_bytes(reg[decode.rs2]);
                    mem[reg[decode.rs1] as usize + offset as usize] = bytes[0];
                    mem[reg[decode.rs1] as usize + offset as usize + 1] = bytes[1];
                }
                0x02 => {
                    let offset = s_format(&fetch.instruction);
                    let bytes = i32::to_be_bytes(reg[decode.rs2]);
                    mem[reg[decode.rs1] as usize + offset as usize] = bytes[0];
                    mem[reg[decode.rs1] as usize + offset as usize + 1] = bytes[1];
                    mem[reg[decode.rs1] as usize + offset as usize + 2] = bytes[2];
                    mem[reg[decode.rs1] as usize + offset as usize + 3] = bytes[3];
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, decode.opcode
                ),
            },
            0x33 => match decode.funct3 {
                0x00 => match decode.funct7 {
                    0x00 => {
                        reg[decode.rd] = reg[decode.rs1] + reg[decode.rs2];
                    }
                    0x20 => {
                        reg[decode.rd] = reg[decode.rs1] - reg[decode.rs2];
                    }
                    unimplemented => println!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, decode.funct3, decode.opcode
                    ),
                },
                0x01 => {
                    reg[decode.rd] = reg[decode.rs1] << reg[decode.rs2];
                }
                0x02 => {
                    if reg[decode.rs1] < reg[decode.rs2] {
                        reg[decode.rd] = 1;
                    } else {
                        reg[decode.rd] = 0;
                    }
                }
                0x03 => {
                    if (reg[decode.rs1] as u32) < (reg[decode.rs2] as u32) {
                        reg[decode.rd] = 1;
                    } else {
                        reg[decode.rd] = 0;
                    }
                }
                0x04 => {
                    reg[decode.rd] = reg[decode.rs1] ^ reg[decode.rs2];
                }
                0x05 => match decode.funct7 {
                    0x00 => {
                        reg[decode.rd] = ((reg[decode.rs1] as u32) >> (reg[decode.rs2] as u32)) as i32;
                    }
                    0x20 => {
                        reg[decode.rd] = reg[decode.rs1] >> reg[decode.rs2];
                    }
                    unimplemented => println!(
                        "Funct7 {:#02x} for funct3 {:#02x} for opcode {:#02x} not implemented...",
                        unimplemented, decode.funct3, decode.opcode
                    ),
                },
                0x06 => {
                    reg[decode.rd] = reg[decode.rs1] | reg[decode.rs2];
                }
                0x07 => {
                    reg[decode.rd] = reg[decode.rs1] & reg[decode.rs2];
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, decode.opcode
                ),
            },
            0x37 => {
                reg[decode.rd] = decode.imm3112;
            }
            0x63 => match decode.funct3 {
                0x00 => {
                    if reg[decode.rs1] == reg[decode.rs2] {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };
                }
                0x01 => {
                    if reg[decode.rs1] != reg[decode.rs2] {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };
                }
                0x04 => {
                    if reg[decode.rs1] < reg[decode.rs2] {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };
                }
                0x05 => {
                    if reg[decode.rs1] >= reg[decode.rs2] {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };
                }
                0x06 => {
                    if (reg[decode.rs1] as u32) < (reg[decode.rs2] as u32) {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };
                }
                0x07 => {
                    if (reg[decode.rs1] as u32) >= (reg[decode.rs2] as u32) {
                        fetch.pc += sb_format(&fetch.instruction) as usize;
                        branch = true;
                    };
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, decode.opcode
                ),
            },
            0x67 => match decode.funct3 {
                0x00 => {
                    reg[decode.rd] = fetch.pc as i32 + 4;
                    fetch.pc = (reg[decode.rs1] + decode.imm110) as usize;
                    branch = true;
                }
                unimplemented => println!(
                    "Funct3 {:#02x} for opcode {:#02x} not implemented...",
                    unimplemented, decode.opcode
                ),
            },
            0x6F => {
                reg[decode.rd] = (fetch.pc + 4) as i32;
                fetch.pc = fetch.pc + uj_format(&fetch.instruction) as usize;
                branch = true;
            }
            0x73 => {
                if reg[17] == 10 {
                    break;
                }
            }
            unimplemented => println!("Opcode {:#02x} not implemented...", unimplemented),
        }

        reg[0] = 0;

        //print_registers_not_zero(&reg);

        if fetch.pc >= *program_len {
            break;
        }

        if stepwise {
            let mut s = String::new();
            io::stdin().read_line(&mut s).expect("Did not read");
        }

        fetch.print_state(&printer::to_assembly(&fetch.instruction));
        decode.print_state(&printer::to_assembly(&decode.instruction));
        
        
        if !branch {
            fetch.update(4);
        } else {
            fetch.update(0);
        }

        decode.update();

        print_registers_not_zero(&reg);
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