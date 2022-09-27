use crate::decode::Decode;
use crate::execute::Execute;
use crate::fetch::Fetch;
use crate::mem_access::MemoryAccess;
use crate::printer;
use crate::writeback::Writeback;
use std::fs;
use std::io;

pub fn run_simulation(filename: &String, stepwise: bool) -> [i32; 32] {
    let mut reg: [i32; 32] = [0; 32];
    let mut mem: [u8; 1048576] = [0; 1048576];
    let mut branch: bool = false;
    let len = read_bytes_to_mem(filename, &mut mem);

    run_engine(&mut reg, &mut mem, &mut branch, &len, stepwise);

    printer::print_registers(&reg);

    reg
}

fn run_engine(
    reg: &mut [i32; 32],
    mem: &mut [u8; 1048576],
    branch: &mut bool,
    program_len: &usize,
    stepwise: bool,
) {
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
        mem_opcode: 0,
        mem_funct3: 0,
        next_result: 0,
        next_mem_address: 0,
        next_destination: 0,
        next_mem_opcode: 0,
        next_mem_funct3: 0,
    };

    let mut mem_access = MemoryAccess {
        instruction: 0,
        next_instruction: 0,
        loaded_memory: 0,
        next_loaded_memory: 0,
        destination: 0,
        next_destination: 0,
        content: 0,
        next_content: 0,
    };

    let mut writeback = Writeback {
        instruction: 0,
        next_instruction: 0,
    };

    loop {
        *branch = false;

        //Setup next instruction
        if fetch.pc < *program_len {
            fetch.fetch_instruction(&mem[fetch.pc..(fetch.pc + 4)]);
        } else {
            fetch.instruction = 0x00;
        }
        decode.instruction = fetch.next_instruction;
        execute.instruction = decode.next_instruction;
        mem_access.instruction = execute.next_instruction;
        writeback.instruction = mem_access.next_instruction;

        // Print instructions for each stage
        fetch.print_state(&printer::to_assembly(&fetch.instruction));
        decode.print_state(&printer::to_assembly(&decode.instruction));
        execute.print_state(&printer::to_assembly(&execute.instruction));
        mem_access.print_state(&printer::to_assembly(&mem_access.instruction));
        writeback.print_state(&printer::to_assembly(&writeback.instruction));

        // Execute stage
        decode.decode_instruction(&reg);
        execute.execute_instruction(&mut fetch, &mut decode, branch);
        mem_access.access_memory(
            mem,
            &execute.next_mem_address,
            &execute.next_result,
            &execute.next_mem_opcode,
            &execute.next_mem_funct3,
            &execute.next_destination,
        );
        writeback.writeback(&mem_access.next_destination, &mem_access.next_content, reg);

        reg[0] = 0;

        //print_registers_not_zero(&reg);

        /*if !(fetch.pc < *program_len) {
            println!("PC: {}, program length: {}", fetch.pc, program_len);
            break;
        }*/

        if stepwise {
            let mut s = String::new();
            io::stdin().read_line(&mut s).expect("Did not read");
        }

        fetch.update(branch);
        decode.update();
        execute.update();
        mem_access.update();
        writeback.update();

        printer::print_registers_not_zero(&reg);

        //print!("{}[2J", 27 as char);
    }

    println!("Program exit");
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
