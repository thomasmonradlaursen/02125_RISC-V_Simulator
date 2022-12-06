use crate::engine::simulator::SimulatorEngine;
use crate::misc::isa_simulator;
use std::fs;

pub struct RegisterResult {
    pub isa: [i32; 32],
    pub pipeline: [i32; 32],
}

pub fn run_test(file: &str, hazard: bool, forward: bool) -> RegisterResult {
    let mut mem: [u8; 262144] = [0; 262144];
    let program_len = read_bytes_to_mem(file, &mut mem);

    let mut pipeline_simulator: SimulatorEngine = Default::default();
    pipeline_simulator.mem = mem;
    pipeline_simulator.program_len = program_len;
    pipeline_simulator.run_engine(false, hazard, forward);

    let mut isa_reg: [i32; 32] = [0; 32];
    isa_simulator::simulate(&mut isa_reg, &mut mem, &program_len);

    RegisterResult { isa: isa_reg, pipeline: pipeline_simulator.reg }
}

fn read_bytes_to_mem(file: &str, mem: &mut [u8; 262144]) -> usize {
    let content = match fs::read(file) {
        Ok(bytes) => bytes,
        Err(err) => panic!("{}: {}", file, err),
    };
    let mut count = 0;
    while count < content.len() {
        mem[count] = content[count];
        count = count + 1;
    }
    content.len()
}