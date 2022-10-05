use crate::hazard::HazardDetectionUnit;
use crate::printer;
use std::fs;
use std::io;

pub fn run_simulation(filename: &String, stepwise: bool) -> [i32; 32] {
    let mut reg: [i32; 32] = [0; 32];
    let mut mem: [u8; 1048576] = [0; 1048576];
    let len = read_bytes_to_mem(filename, &mut mem);

    run_engine(&mut reg, &mut mem, &len, stepwise);

    printer::print_registers(&reg);

    reg
}

fn run_engine(reg: &mut [i32; 32], mem: &mut [u8; 1048576], program_len: &usize, stepwise: bool) {
    
    println!("Hello Rust RISC-V Pipeline!");

    let mut hazard = HazardDetectionUnit {
        ..Default::default()
    };

    

    let mut branch: bool;
    let mut running: bool = true;
    let mut stall: bool;

    while running {
        
        branch = false;
        stall = false;

        //Setup next instruction
        if !stall {
            if fetch.pc < *program_len {
                fetch.fetch_instruction(&mem[fetch.pc..(fetch.pc + 4)]);
            } else {
                fetch.instruction = 0x2000;
            }
            decode.initialize_fields(&fetch);
        }
        execute.initialize_fields(&decode);
        mem_access.initialize_fields(&execute);
        writeback.initialize_fields(&mem_access);

        // Print instructions for each stage
        fetch.print_state(&printer::to_assembly(&fetch.instruction));
        decode.print_state(&printer::to_assembly(&decode.instruction));
        execute.print_state(&printer::to_assembly(&execute.instruction));
        mem_access.print_state(&printer::to_assembly(&mem_access.instruction));
        writeback.print_state(&printer::to_assembly(&writeback.instruction));

        // Check for hazards
        hazard.initialize_fields(&decode, &execute, &mem_access);
        hazard.print_values();
        hazard.detect_hazard(&mut decode, &mut stall);

        // Execute stage
        if !stall {
            decode.decode_instruction(&reg);
        }
        execute.execute_instruction(&mut fetch, &mut decode, &mut branch);
        mem_access.access_memory(
            mem,
            &execute.next_mem_address,
            &execute.next_result,
            &execute.next_mem_opcode,
            &execute.next_mem_funct3,
            &execute.next_destination,
            &execute.next_reg_write,
        );
        writeback.writeback(
            &mem_access.next_destination,
            &mem_access.next_content,
            reg,
            &mut running,
            &mem_access.next_reg_write,
        );

        reg[0] = 0;

        if stepwise {
            let mut s = String::new();
            io::stdin().read_line(&mut s).expect("Did not read");
        }

        if !stall {
            fetch.update(&program_len, &mut branch);
        }
        decode.update();
        execute.update();
        mem_access.update();
        writeback.update();

        printer::print_registers_not_zero(&reg);
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
