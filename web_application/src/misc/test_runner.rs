use crate::engine::simulator::SimulatorEngine;
use crate::misc::isa_simulator;
use std::fs;
use std::io::prelude::*;

pub struct RegisterResult {
    pub isa: [i32; 32],
    pub pipeline: [i32; 32],
    pub pipeline_cycles: u32,
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

    RegisterResult { isa: isa_reg, pipeline: pipeline_simulator.reg, pipeline_cycles: pipeline_simulator.cycles }
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

pub fn result_to_csv(file: &str, to_file: bool) {
    if !to_file {
        return;
    }
    let res_without: RegisterResult = run_test(file, true, false);
    if !(res_without.pipeline.iter().zip(res_without.isa.iter()).all(|(a,b)| a == b)) {
        return;
    }
    let res_with: RegisterResult = run_test(file, true, true);
    if !(res_with.pipeline.iter().zip(res_with.isa.iter()).all(|(a,b)| a == b)) {
        return;
    }

    let mut res_file = fs::OpenOptions::new().write(true)
    .append(true)
    .open("results.csv")
    .unwrap();

    let filename = match file.split_once("/") {
        Some(with_extension) => match with_extension.1.split_once(".") {
            Some(name) => name.0,
            None => panic!("Failed to split a \"/\""),
        },
        None => panic!("Failed to split a \".\""),
    };

    write!(res_file, "{};{};{}\n", filename, res_without.pipeline_cycles, res_with.pipeline_cycles).expect("Failed to write result");
}