use mylib::decode::Decode;
use mylib::fetch::Fetch;
use mylib::execute::Execute;
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
        s_offset: 0,
        sb_offset: 0,
        uj_offset: 0,
        next_opcode: 0,
        next_funct3: 0,
        next_funct7: 0,
        next_rd: 0,
        next_rs1: 0,
        next_rs2: 0,
        next_imm3112: 0,
        next_imm110: 0,
        next_shamt: 0,
        next_s_offset: 0,
        next_sb_offset: 0,
        next_uj_offset: 0,
    };

    let mut execute = Execute {
        instruction: 0,
        next_instruction: 0,
        result: 0,
        mem_address: 0,
        destination: 0,
    };

    let stepwise = true;

    let mut branch: bool = false;

    loop {
        fetch.fetch_instruction(&mem[fetch.pc..(fetch.pc + 4)]);
        decode.instruction = fetch.next_instruction;
        decode.decode_instruction(&reg);
        execute.instruction = decode.next_instruction;
        execute.execute_instruction(&decode);

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
        execute.print_state(&printer::to_assembly(&execute.instruction));
        
        if !branch {
            fetch.update(4);
        } else {
            fetch.update(0);
        }

        decode.update();
        execute.update();

        print_registers_not_zero(&reg);
    }

    println!("Program exit");

    reg
}

fn read_bytes_to_mem(filename: &String, mem: &mut [u8; 1048576]) -> usize {
    
    let mut content: Vec<u8> = vec![];
    while content.len() <= 0 {
        let file = fs::read(filename);
        match file {
            Ok(raw_bytes) => content = raw_bytes,
            Err(err) => println!("{:?}", err),
        }
    }
    let mut count = 0;
    while count < content.len() {
        mem[count] = content[count];
        count = count + 1;
    }
    println!("Bytes have been loaded to memory");
    content.len()
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