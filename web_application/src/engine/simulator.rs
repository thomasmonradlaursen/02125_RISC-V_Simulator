use crate::engine::components::{forward, hazard, registers::{EXMEMReg, IDEXReg, IFIDReg, MEMWBReg}};
use crate::engine::stages::{decode, execute, fetch, mem_access, writeback};

use gloo_dialogs;

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
    pub pc_instruction: i32,
    pub pc_src: usize,
    pub branch: bool,
    pub running: bool,
    pub program_len: usize,
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
            pc_instruction: 0,
            pc_src: 0,
            branch: false,
            running: true,
            program_len: 0,
        }
    }
}

impl SimulatorEngine {
    pub fn run_engine(&mut self, stepwise: bool, hazard: bool, forward: bool) {
        let mut ebreak: bool = false;
        while self.running {
            // Reset for next cycle
            self.branch = false;
            self.stall = false;
            self.pc_src = self.pc + 4;

            let next_instruction = &self.mem[(self.pc)..(self.pc + 4)];

            self.pc_instruction = i32::from_le_bytes([
                self.mem[self.pc],
                self.mem[self.pc + 1],
                self.mem[self.pc + 2],
                self.mem[self.pc + 3],
            ]);

            // Run pipeline

            if !self.stall {
                fetch::fetch_to_register(
                    &mut self.pc,
                    &mut self.if_id.fetch,
                    &next_instruction,
                    &self.program_len,
                );
                decode::decode_to_register(
                    &mut self.if_id.decode,
                    &mut self.id_ex.decode,
                    &self.reg,
                    &mut self.pc_src,
                    &mut self.program_len,
                    &mut ebreak,
                    &mut self.if_id.fetch,
                );
            }

            execute::execute_to_register(
                &mut self.id_ex.execute,
                &mut self.ex_mem.execute,
                &mut self.pc_src,
                &mut self.branch,
                &mut self.running
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
                &self.program_len,
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
                    hazard::ex_hazard(&self.if_id.decode, &self.id_ex.execute, &mut self.stall);
                    hazard::mem_hazard(
                        &self.if_id.decode,
                        &self.ex_mem.mem,
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
                forward::mem_forward(
                    &self.id_ex.decode,
                    &self.mem_wb.mem,
                    &mut self.forward_a,
                    &mut self.forward_b,
                );
                forward::load_forward(
                    &self.id_ex.decode,
                    &self.mem_wb.mem,
                    &mut self.forward_a,
                    &mut self.forward_b,
                );
            }

            // Update register values for next iteration
            Self::increment_program_counter(self);
            if !self.running {
                self.unexpected_termination();
                break;
            }

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

            if self.stall {
                self.id_ex.execute = Default::default();
                self.id_ex.execute.instruction = 0x3000;
            }

            self.cycles += 1;

            // NEEDS TO BE FIXED:
            self.pc_instruction = i32::from_le_bytes([
                self.mem[self.pc],
                self.mem[self.pc + 1],
                self.mem[self.pc + 2],
                self.mem[self.pc + 3],
            ]);

            if stepwise || ebreak {
                break;
            }
        }
    }

    pub fn read_bytes_to_mem(&mut self, data: &Vec<u8>) {
        let mut count = 0;
        while count < data.len() {
            self.mem[count] = data[count];
            count = count + 1;
        }
        self.program_len = data.len();
        // NEEDS TO BE FIXED:
        self.pc_instruction = i32::from_le_bytes([
            self.mem[self.pc],
            self.mem[self.pc + 1],
            self.mem[self.pc + 2],
            self.mem[self.pc + 3],
        ]);
    }

    pub fn increment_program_counter(&mut self) {
        if self.pc_src > self.mem.len() {
            let mut message = 
            format!("WARNING: Updated PC to {}, which is out of bound as the current size of memory is {} bytes.\n", self.pc_src, self.mem.len());
            message.push_str("To prevent the program from crashing, the simulation will terminate.");
            gloo_dialogs::alert(&message[..]);
            self.running = false;
        }
        if !self.stall && self.pc < self.program_len {
            self.pc = self.pc_src;
        }
    }

    fn unexpected_termination(&mut self) {
        self.pc_instruction = 0x4000;
        self.if_id.decode.instruction = 0x4000;
        self.id_ex.execute.instruction = 0x4000;
        self.ex_mem.mem.instruction = 0x4000;
        self.mem_wb.wb.instruction = 0x4000;
    }
}
