
use crate::forward;
use crate::hazard;
use crate::registers::{EXMEMReg, IDEXReg, IFIDReg, MEMWBReg};
use crate::{decode, execute, fetch, mem_access, printer, writeback};

pub struct SimulatorEngine {
    // Registers and memory
    pub reg: [i32; 32],
    pub mem: [u8; 262144],
    // Pipeline register
    pub if_id: IFIDReg,
    pub id_ex: IDEXReg,
    pub ex_mem: EXMEMReg,
    pub mem_wb: MEMWBReg,
    // Hazard and forwarding parameters
    pub stall: bool,
    pub forward_a: u8,
    pub forward_b: u8,
    // Control parameters
    pub cycles: u32,
    pub pc: usize,
    pub pc_src: usize,
    pub branch: bool,
    pub running: bool,
}

impl Default for SimulatorEngine {
    fn default() -> Self {
        Self {
            reg: [0; 32],
            mem: [0; 262144],
            if_id: IFIDReg {
                fetch: Default::default(),
                decode: Default::default(),
            },
            id_ex: IDEXReg {
                decode: Default::default(),
                execute: Default::default(),
            },
            ex_mem: EXMEMReg {
                execute: Default::default(),
                mem: Default::default(),
            },
            mem_wb: MEMWBReg {
                mem: Default::default(),
                wb: Default::default(),
            },
            stall: false,
            forward_a: 0,
            forward_b: 0,
            cycles: 0,
            pc: 0,
            pc_src: 0,
            branch: false,
            running: true,
        }
    }
}

impl SimulatorEngine {
    pub fn run_engine(&mut self, program_len: &usize, stepwise: bool, hazard: bool, forward: bool) {
        while self.running {
            // Reset for next cycle
            self.branch = false;
            self.stall = false;
            self.pc_src = self.pc + 4;

            let next_instruction = &self.mem[(self.pc)..(self.pc + 4)];

            println!("Cycle number: {}", self.cycles);

            // Print state of pipeline registers
            println!(
                "Fetch:\nInstruction: {}\nPC: {}\n",
                printer::to_assembly(&fetch::fetch_instruction(&next_instruction)),
                self.pc
            );
            self.if_id.print_decode();
            self.id_ex.print_execute();
            self.ex_mem.print_mem();
            self.mem_wb.print_wb();

            // Run pipeline

            if !self.stall {
                fetch::fetch_to_register(
                    &mut self.pc,
                    &mut self.if_id.fetch,
                    &next_instruction,
                    program_len,
                );
                decode::decode_to_register(
                    &mut self.if_id.decode,
                    &mut self.id_ex.decode,
                    &self.reg,
                );
            } else {
                println!("Stalling fetch and decode");
            }

            execute::execute_to_register(
                &mut self.id_ex.execute,
                &mut self.ex_mem.execute,
                &mut self.pc_src,
                &mut self.branch,
            );

            mem_access::memory_to_register(
                &mut self.ex_mem.mem,
                &mut self.mem_wb.mem,
                &mut self.mem,
            );
            writeback::writeback(
                &self.mem_wb.wb,
                &mut self.reg,
                &mut self.running,
                program_len,
            );

            // Hazard
            if hazard {
                hazard::load_use_hazard(&self.if_id.decode, &self.id_ex.execute, &mut self.stall);
                hazard::control_hazard(&mut self.if_id.fetch, &mut self.id_ex.decode, &self.branch);
                if !forward {
                    hazard::load_use_hazard_extended(
                        &self.if_id.decode,
                        &self.ex_mem.mem,
                        &mut self.stall,
                    );
                    hazard::ex_hazard(&self.id_ex.decode, &self.ex_mem.execute, &mut self.stall);
                    hazard::mem_hazard(
                        &self.id_ex.decode,
                        &self.ex_mem.execute,
                        &self.mem_wb.mem,
                        &mut self.stall,
                    );
                }
            }

            // Forwarding
            if forward {
                forward::reset_multiplexers(&mut self.forward_a, &mut self.forward_b);
                forward::ex_forward(
                    &self.id_ex.decode,
                    &self.ex_mem.execute,
                    &mut self.forward_a,
                    &mut self.forward_b,
                );
                forward::mem_hazard(
                    &self.id_ex.decode,
                    &self.mem_wb.mem,
                    &mut self.forward_a,
                    &mut self.forward_b,
                );
                forward::load_use_forward(
                    &self.if_id.decode,
                    &self.ex_mem.mem,
                    &mut self.forward_a,
                    &mut self.forward_b,
                );
            }

            // Update register values for next iteration
            increment_program_counter(&mut self.pc, &self.pc_src, &self.stall);
            if !self.stall {
                fetch::update_for_decode(&mut self.if_id.fetch, &mut self.if_id.decode);
            }
            decode::update_for_execution(
                &mut self.id_ex.decode,
                &mut self.id_ex.execute,
                &self.reg,
            );
            execute::update_for_memory(&mut self.ex_mem.execute, &mut self.ex_mem.mem);
            mem_access::update_for_writeback(&mut self.mem_wb.mem, &mut self.mem_wb.wb);

            // Update based on forwarding
            if forward {
                forward::update_forward_a(
                    &mut self.id_ex.execute,
                    &self.ex_mem.execute,
                    &self.mem_wb.mem,
                    &self.forward_a,
                );
                forward::update_forward_b(
                    &mut self.id_ex.execute,
                    &self.ex_mem.execute,
                    &self.mem_wb.mem,
                    &self.forward_b,
                );
            }

            printer::print_registers_not_zero(&self.reg);

            if self.stall {
                self.id_ex.execute = Default::default();
                self.id_ex.execute.instruction = 0x3000;
            }

            self.cycles += 1;

            /*if stepwise {
                let mut s = String::new();
                io::stdin().read_line(&mut s).expect("Did not read");
            }*/

            println!("______________________________________");
        }
        println!("Execution terminated.");
    }
}


pub fn read_bytes_to_mem(file: &(String, Vec<u8>), mem: &mut [u8]) -> usize {
    let mut count = 0;
    while count < file.1.len() {
        mem[count] = file.1[count];
        count = count + 1;
    }
    file.1.len()
}

pub fn increment_program_counter(pc: &mut usize, pc_src: &usize, stall: &bool) {
    if !*stall {
        *pc = *pc_src;
    }
}


pub fn run_simulation(
    file: &(String, Vec<u8>),
    simulator_engine: &mut SimulatorEngine,
    stepwise: bool,
    hazard: bool,
    forward: bool,
) {
    let program_len = read_bytes_to_mem(file, &mut simulator_engine.mem);

    printer::print_program_info(&file.0, &program_len);

    simulator_engine.run_engine(&program_len, stepwise, hazard, forward);

    printer::print_registers(&simulator_engine.reg);
}

