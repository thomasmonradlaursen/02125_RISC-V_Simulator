use std::fs;
use std::env;

fn main() {

    let reg: [i32; 32] = [0; 32];

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let instructions = convert_to_instructions(&read_bytes(filename));

    print_instructions(&instructions);

    let res = simulate(reg, instructions);

    print_registers(&res);

}

fn simulate(mut reg: [i32; 32], instructions: Vec<i32>) -> [i32; 32]{
    
    println!("Hello Rust RISC-V world!");
    
    let mut pc: i32 = 0;

    loop {
        let instr = instructions[(pc >> 2) as usize];
        let opcode = instr & 0x7f;
        let rd = (instr >> 7) & 0x01f;
        let rs1 = (instr >> 15) & 0x01f;
        let rs2 = (instr >> 20) & 0x01f;
        let imm = instr >> 20;

        match opcode {
            0x13 => {
                reg[rd as usize] = reg[rs1 as usize] + imm;
                println!("Running {:#02x}", opcode);
            }
            0x33 => {
                reg[rd as usize] = reg[rs1 as usize] + reg[rs2 as usize];
                println!("Running {:#02x}", opcode);
            }
            0x73 => {
                println!("Running {:#02x}", opcode);
                if reg[17] == 10 {
                    break;
                }
            }
            unimplemented => println!("Opcode {:#02x} not implemented...", unimplemented),
        }

        pc += 4; // One instruction is four bytes
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
        println!("Reg[{}]: {}", count, register);
        count += 1;
    }
}

