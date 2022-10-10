use crate::hazard;
use crate::registers::{EXMEMReg, IDEXReg, IFIDReg, MEMWBReg, PCReg};
use crate::registers::{EXMEM, IDEX, IFID, MEMWB, PC};
use crate::{counter, decode, execute, fetch, mem_access, writeback, printer};
use std::fs;
use std::io;

pub fn run_simulation(filename: &String, stepwise: bool) -> [i32; 32] {
    let mut reg: [i32; 32] = [0; 32];
    let mut mem: [u8; 1048576] = [0; 1048576];
    let program_len = read_bytes_to_mem(filename, &mut mem);

    printer::print_program_info(filename, &program_len);

    run_engine(&mut reg, &mut mem, &program_len, stepwise);

    printer::print_registers(&reg);

    reg
}

fn run_engine(reg: &mut [i32; 32], mem: &mut [u8; 1048576], program_len: &usize, stepwise: bool) {
    let mut pc: PCReg = PCReg {
        pc: PC { pc: 0,instruction: i32::from_le_bytes([mem[0], mem[1], mem[2], mem[3]]), },
        fetch: PC { ..Default::default() },
    };

    let mut if_id: IFIDReg = IFIDReg {
        fetch: IFID { ..Default::default() },
        decode: IFID { ..Default::default() },
    };

    let mut id_ex: IDEXReg = IDEXReg {
        decode: IDEX { ..Default::default() },
        execute: IDEX { ..Default::default() },
    };

    let mut ex_mem: EXMEMReg = EXMEMReg {
        execute: EXMEM { ..Default::default() },
        mem: EXMEM { ..Default::default() },
    };

    let mut mem_wb: MEMWBReg = MEMWBReg {
        mem: MEMWB { ..Default::default() },
        wb: MEMWB { ..Default::default() },
    };

    let mut branch: bool;
    let mut running: bool = true;
    let mut stall: bool;
    let mut pc_src: usize;
    let mut cycles: u32 = 0;

    while running {
        
        branch = false;
        stall = false;
        pc_src = 4;

        // Print state of pipeline registers
        pc.pc.print_state();
        if_id.fetch.print_state();
        id_ex.decode.print_state();
        ex_mem.execute.print_state();
        mem_wb.mem.print_state();

        if stepwise {
            let mut s = String::new();
            io::stdin().read_line(&mut s).expect("Did not read");
        }

        // Run pipeline
        counter::increment_program_counter(&mut pc.pc, &pc_src, &stall);
        let mem_next = &mem[(if_id.fetch.pc)..(if_id.fetch.pc + 4)];
        fetch::fetch_to_register(&mut pc.fetch, &mut if_id.fetch, mem_next, program_len);
        decode::decode_to_register(&mut if_id.decode, &mut id_ex.decode, reg);
        execute::execute_to_register(&mut id_ex.execute, &mut ex_mem.execute, &mut pc_src, &mut branch, &mut pc.fetch);
        mem_access::memory_to_register(&mut ex_mem.mem, &mut mem_wb.mem, mem);
        writeback::writeback(&mem_wb.wb, reg, &mut running);

        // Update register values for next iteration
        counter::update_for_fetch(&mut pc.pc, &mut pc.fetch);
        fetch::update_for_decode(&mut if_id.fetch, &mut if_id.decode);
        decode::update_for_execution(&mut id_ex.decode, &mut id_ex.execute);
        execute::update_for_memory(&mut ex_mem.execute, &mut ex_mem.mem);
        mem_access::update_for_writeback(&mut mem_wb.mem, &mut mem_wb.wb);

        reg[0] = 0;

        cycles+=1;
        println!("Cycle number: {}", cycles);
    }
    println!("Execution completed.");
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
    content.len()
}
