use crate::forward;
use crate::hazard;
use crate::registers::{EXMEMReg, IDEXReg, IFIDReg, MEMWBReg};
use crate::registers::{EXMEM, IDEX, IFID, MEMWB};
use crate::{decode, execute, fetch, mem_access, printer, writeback};
use std::fs;
use std::io;

pub fn run_simulation(filename: &String, stepwise: bool, hazard: bool, forward: bool) -> ([i32; 32], u32) {
    let mut reg: [i32; 32] = [0; 32];
    let mut mem: [u8; 1048576] = [0; 1048576];
    let mut cycles: u32 = 0;
    let program_len = read_bytes_to_mem(filename, &mut mem);

    printer::print_program_info(filename, &program_len);

    run_engine(&mut reg, &mut mem, &mut cycles, &program_len, stepwise, hazard, forward);

    printer::print_registers(&reg);

    (reg, cycles)
}

fn run_engine(
    reg: &mut [i32; 32],
    mem: &mut [u8; 1048576],
    cycles: &mut u32,
    program_len: &usize,
    stepwise: bool,
    enable_hazard: bool,
    enable_forwarding: bool,
) {
    let mut if_id: IFIDReg = IFIDReg {
        fetch: IFID {
            ..Default::default()
        },
        decode: IFID {
            ..Default::default()
        },
    };

    let mut id_ex: IDEXReg = IDEXReg {
        decode: IDEX {
            ..Default::default()
        },
        execute: IDEX {
            ..Default::default()
        },
    };

    let mut ex_mem: EXMEMReg = EXMEMReg {
        execute: EXMEM {
            ..Default::default()
        },
        mem: EXMEM {
            ..Default::default()
        },
    };

    let mut mem_wb: MEMWBReg = MEMWBReg {
        mem: MEMWB {
            ..Default::default()
        },
        wb: MEMWB {
            ..Default::default()
        },
    };

    let mut branch: bool;
    let mut running: bool = true;
    let mut stall: bool;
    let mut pc_src: usize;
    let mut pc: usize = 0;
    let mut forward_a: u8 = 0;
    let mut forward_b: u8 = 0;

    while running {
        branch = false;
        stall = false;
        pc_src = pc + 4;

        let next_instruction = &mem[(pc)..(pc + 4)];

        println!("Cycle number: {}", cycles);

        // Print state of pipeline registers
        println!(
            "Fetch:\nInstruction: {}\nPC: {}\n",
            printer::to_assembly(&fetch::fetch_instruction(&next_instruction)),
            pc
        );
        if_id.print_decode();
        id_ex.print_execute();
        ex_mem.print_mem();
        mem_wb.print_wb();

        // Handle hazards

        // Hazard detection should properly overwrite any current result in decode

        // Run pipeline

        if !stall {
            fetch::fetch_to_register(&mut pc, &mut if_id.fetch, &next_instruction, program_len);
            decode::decode_to_register(&mut if_id.decode, &mut id_ex.decode, reg);
        } else {
            println!("Stalling fetch and decode");
        }

        execute::execute_to_register(
            &mut id_ex.execute,
            &mut ex_mem.execute,
            &mut pc_src,
            &mut branch,
            &mut running,
        );

        mem_access::memory_to_register(&mut ex_mem.mem, &mut mem_wb.mem, mem);
        writeback::writeback(&mem_wb.wb, reg, &mut running, program_len);

        // Hazard
        if enable_hazard {
            hazard::load_use_hazard(&if_id.decode, &id_ex.execute, &mut stall);
            hazard::control_hazard(&mut if_id.fetch, &mut id_ex.decode, &branch);
            if !enable_forwarding {
                hazard::load_use_hazard_extended(&if_id.decode, &ex_mem.mem, &mut stall);
                hazard::ex_hazard(&if_id.decode, &id_ex.execute, &mut stall);
                hazard::mem_hazard(&if_id.decode, &ex_mem.mem, &mut stall);
            }
        }

        // Forwarding
        if enable_forwarding {
            forward::reset_multiplexers(&mut forward_a, &mut forward_b);
            forward::ex_forward(
                &id_ex.decode,
                &ex_mem.execute,
                &mut forward_a,
                &mut forward_b,
            );
            forward::mem_hazard(&id_ex.decode, &mem_wb.mem, &mut forward_a, &mut forward_b);
            forward::load_use_forward(&id_ex.decode, &mem_wb.mem, &mut forward_a, &mut forward_b);
        }

        // Update register values for next iteration
        increment_program_counter(&mut pc, &pc_src, &stall);
        if !stall {
            fetch::update_for_decode(&mut if_id.fetch, &mut if_id.decode);
        }
        decode::update_for_execution(&mut id_ex.decode, &mut id_ex.execute, &reg);
        execute::update_for_memory(&mut ex_mem.execute, &mut ex_mem.mem);
        mem_access::update_for_writeback(&mut mem_wb.mem, &mut mem_wb.wb);

        // Update based on forwarding
        if enable_forwarding {
            forward::update_forward_a(&mut id_ex.execute, &ex_mem.execute, &mem_wb.mem, &forward_a);
            forward::update_forward_b(&mut id_ex.execute, &ex_mem.execute, &mem_wb.mem, &forward_b);
        }

        printer::print_registers_not_zero(reg);

        if stall {
            id_ex.execute = Default::default();
            id_ex.execute.instruction = 0x3000;
        }

        reg[0] = 0;

        *cycles += 1;

        if stepwise {
            let mut s = String::new();
            io::stdin().read_line(&mut s).expect("Did not read");
        }

        println!("______________________________________");
    }
    println!("Execution terminated.");
}

fn read_bytes_to_mem(filename: &String, mem: &mut [u8; 1048576]) -> usize {
    let content: Vec<u8> = fs::read(filename).unwrap();
    let mut count = 0;
    while count < content.len() {
        mem[count] = content[count];
        count = count + 1;
    }
    content.len()
}

pub fn increment_program_counter(pc: &mut usize, pc_src: &usize, stall: &bool) {
    if !*stall {
        *pc = *pc_src;
    }
}
