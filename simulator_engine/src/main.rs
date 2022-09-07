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
        let rd = (instruction >> 7) & 0x01f;
        let rs1 = (instruction >> 15) & 0x01f;
        let rs2 = (instruction >> 20) & 0x01f;
        let imm3112 = (instruction >> 12) << 12;
        let imm110 = instruction >> 20;

        match opcode {
            0x13 => {
                reg[rd as usize] = reg[rs1 as usize] + imm110;
                println!("ADDI x{}, x{}, {}", rd, rs1, imm110);
            }
            0x17 => {
                reg[rd as usize] = pc + imm3112;
                println!("AUIPC x{}, {}", rd, imm3112);
            }
            0x33 => {
                reg[rd as usize] = reg[rs1 as usize] + reg[rs2 as usize];
                println!("ADD x{}, x{}, x{}", rd, rs1, rs2);
            }
            0x37 => {
                reg[rd as usize] = imm3112;
                println!("LUI x{}, {}", rd, imm3112);
            }
            0x67 => match funct3 {
                0x00 => {
                    reg[rd as usize] = pc + 4;
                    pc = reg[rs1 as usize] + imm110;
                    println!("JALR x{}, x{}, {}", rd, rs1, imm110);
                }
                unimplemented => println!("Funct3 {:#02x} not implemented...", unimplemented),
            }
            0x6F => {
                reg[rd as usize] = pc + 4;
                pc = pc + imm3112;
                println!("JAL x{}, {}", rd, imm3112);
            }
            0x73 => {
                println!("ECALL");
                if reg[17] == 10 {
                    break;
                }
            }
            unimplemented => println!("Opcode {:#02x} not implemented...", unimplemented),
        }

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
