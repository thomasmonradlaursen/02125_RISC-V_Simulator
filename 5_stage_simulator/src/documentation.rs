use crate::simulator_engine;

use std::fs;
use std::io::prelude::*;

pub fn test_documentation(filename: &String, stepwise: bool, hazard: bool, forward: bool, expected: &fn() -> [i32; 32]) {
    let sim_res = simulator_engine::run_simulation(filename, stepwise, hazard, forward);
    let reg = sim_res.0;
    let cycles = sim_res.1;

    let status = match reg.iter().zip(expected().iter()).all(|(a, b)| a == b) {
        true => "Success",
        false => "Fail",
    };

    // Write test results to csv
    let test_name = extract_test_name(filename);
    let path = create_result_file(&test_name, &hazard, &forward);
    let number = test_number(&test_name);
    write_result(&path, &cycles, &number, &status)

}

pub fn test_add() -> [i32; 32] {
    [0,0,0,0,0,-100,125,160,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,60,0,0,0]
}

pub fn test_addi() -> [i32; 32] {
    [0,0,0,0,0,0,125,160,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,60,0,0,0]
}

pub fn test_and() -> [i32; 32] {
    [0,0,0,0,0,251723761,1793,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_andi() -> [i32; 32] {
    [0,0,0,0,0,251723761,1793,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_auipc() -> [i32; 32] {
    [0,0,0,0,0,4194304,4194308,1883242504,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,12,0,0,0]
}

pub fn test_beq() -> [i32; 32] {
    [0,0,0,0,0,5,-10,0,0,1,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_bge() -> [i32; 32] {
    [0,0,0,0,0,5,-10,0,0,1,10,0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_bgeu() -> [i32; 32] {
    [0,0,0,0,0,5,-10,0,0,1,10,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_blt() -> [i32; 32] {
    [0,0,0,0,0,5,-10,0,1,0,10,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_bltu() -> [i32; 32] {
    [0,0,0,0,0,5,-10,0,1,0,10,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_bne() -> [i32; 32] {
    [0,0,0,0,0,5,-10,0,1,0,10,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_jal() -> [i32; 32] {
    [0,0,0,0,0,4,16,0,1,1,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_jalr() -> [i32; 32] {
    [0,0,0,0,0,4,20,0,1,1,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_lb() -> [i32; 32] {
    [0,0,0,0,0,102,-128,79,102,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_lbu() -> [i32; 32] {
    [0,0,0,0,0,102,128,79,102,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_lh() -> [i32; 32] {
    [0,0,0,0,0,13302,-32640,20303,13302,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_lhu() -> [i32; 32] {
    [0,0,0,0,0,13302,32896,20303,13302,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_li() -> [i32; 32] {
    [0,0,0,0,0,0,0,0,-1,29154821,10,0,0,0,0,0,0,0,-1301186204,-12,-1125213234,-16777216,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_lui() -> [i32; 32] {
    [0,0,0,0,0,4194304,259366912,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_lw() -> [i32; 32] {
    [0,0,0,0,0,1338802,-2139062144,1330597711,1338802,-2139062144,10,0,0,0,0,0,0,0,39,1330597711,103,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_or() -> [i32; 32] {
    [0,0,0,0,0,-1895759887,-1895759887,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_ori() -> [i32; 32] {
    [0,0,0,0,0,-1895759887,-1895759887,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_sb() -> [i32; 32] {
    [0,0,0,0,0,102,102,0,102,60,10,0,0,0,0,0,0,0,102,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_sh() -> [i32; 32] {
    [0,0,0,0,0,45,10232,0,52781,50,10,0,0,0,0,0,0,0,10232,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_sll() -> [i32; 32] {
    [0,0,0,0,0,251723520,-267390976,-16777216,16,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_slli() -> [i32; 32] {
    [0,0,0,0,0,251723520,-267390976,-16777216,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_slt() -> [i32; 32] {
    [0,0,0,0,0,-20,1,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0]
}

pub fn test_slti() -> [i32; 32] {
    [0,0,0,0,0,-20,1,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0]
}

pub fn test_sltiu() -> [i32; 32] {
    [0,0,0,0,0,-20,1,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0]
}

pub fn test_sltu() -> [i32; 32] {
    [0,0,0,0,0,-20,1,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0]
}

pub fn test_sra() -> [i32; 32] {
    [0,0,0,0,0,15794160,-15732721,-61456,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,987135,3855,0,0]
}

pub fn test_srai() -> [i32; 32] {
    [0,0,0,0,0,15794160,-15732721,-61456,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,987135,3855,0,0]
}

pub fn test_srl() -> [i32; 32] {
    [0,0,0,0,0,-251723536,252702735,987120,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_srli() -> [i32; 32] {
    [0,0,0,0,0,-251723536,252702735,987120,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}


pub fn test_sub() -> [i32; 32] {
    [0,0,0,0,0,-100,-125,-160,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-60,0,0,0]
}

pub fn test_sw() -> [i32; 32] {
    [0,0,0,0,0,45,-50,116,980733485,50,10,0,0,0,0,0,0,0,10232,10232,0,0,0,0,0,0,0,0,58,-12755,14964,29902]
}

pub fn test_xor() -> [i32; 32] {
    [0,0,0,0,0,251723760,1807,251721983,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_xori() -> [i32; 32] {
    [0,0,0,0,0,251723760,251721983,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn test_load_use() -> [i32; 32] {
    [0,1,2,1,3,-1,0,0,0,0,0,0,0,0,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
}

pub fn return_function(name: &str) -> fn() -> [i32; 32]{
    match name {
        "test_add" => test_add,
        "test_addi" => test_addi,
        "test_and" => test_and,
        "test_andi" => test_andi,
        "test_auipc" => test_auipc,
        "test_beq" => test_beq,
        "test_bge" => test_bge,
        "test_bgeu" => test_bgeu,
        "test_blt" => test_blt,
        "test_bltu" => test_bltu,
        "test_bne" => test_bne,
        "test_jal" => test_jal,
        "test_jalr" => test_jalr,
        "test_lb" => test_lb,
        "test_lbu" => test_lbu,
        "test_lh" => test_lh,
        "test_lhu" => test_lhu,
        "test_li" => test_li,
        "test_load_use" => test_load_use,
        "test_lui" => test_lui,
        "test_lw" => test_lw,
        "test_or" => test_or,
        "test_ori" => test_ori,
        "test_sb" => test_sb,
        "test_sh" => test_sh,
        "test_sll" => test_sll,
        "test_slli" => test_slli,
        "test_slt" => test_slt,
        "test_slti" => test_slti,
        "test_sltiu" => test_sltiu,
        "test_sltu" => test_sltu,
        "test_sra" => test_sra,
        "test_srai" => test_srai,
        "test_srl" => test_srl,
        "test_srli" => test_srli,
        "test_sub" => test_sub,
        "test_sw" => test_sw,
        "test_xor" => test_xor,
        "test_xori" => test_xori,
        _ => panic!("Expected result for comparison not found: {}", name),
    }
}

pub fn test_number(name: &str) -> i32{
    match name {
        "test_add" => 1,
        "test_addi" => 2,
        "test_and" => 3,
        "test_andi" => 4,
        "test_auipc" => 5,
        "test_beq" => 6,
        "test_bge" => 7,
        "test_bgeu" => 8,
        "test_blt" => 9,
        "test_bltu" => 10,
        "test_bne" => 11,
        "test_jal" => 12,
        "test_jalr" => 13,
        "test_lb" => 14,
        "test_lbu" => 15,
        "test_lh" => 16,
        "test_lhu" => 17,
        "test_li" => 18,
        "test_load_use" => 19,
        "test_lui" => 20,
        "test_lw" => 21,
        "test_or" => 22,
        "test_ori" => 23,
        "test_sb" => 24,
        "test_sh" => 25,
        "test_sll" => 26,
        "test_slli" => 27,
        "test_slt" => 28,
        "test_slti" => 29,
        "test_sltiu" => 30,
        "test_sltu" => 31,
        "test_sra" => 32,
        "test_srai" => 33,
        "test_srl" => 34,
        "test_srli" => 35,
        "test_sub" => 36,
        "test_sw" => 37,
        "test_xor" => 38,
        "test_xori" => 39,
        _ => panic!("Expected result for comparison not found: {}", name),
    }
}

fn create_result_file(name: &str, hazard: &bool, forward: &bool) -> String{
    let label = match (hazard, forward) {
        (true, true) => "hazard_forwarding",
        (true, false) => "hazard",
        (false, true) => "forwarding",
        (false, false) => "baseline",
    };
    let path = format!("results/{}/{}_{}.csv", name, label, name);
    match fs::create_dir(format!("results/{}", name)) {
        Ok(_) => (),
        Err(_) => (),
    }
    fs::write(&path, "CYCLES TEST STATUS\n").expect("Failed to create file");
    path
}

fn write_result(path: &String, cycles: &u32, test: &i32, status: &str) {

    let contents = format!("{} {} {}", cycles, test, status);
    let mut file = fs::OpenOptions::new().write(true)
    .append(true)
    .open(path)
    .unwrap();

    write!(file, "{}\n", contents).expect("Failed to write result");
}

fn extract_test_name(filename: &String) -> &str {
    
    let binary = match filename.split_once("/") {
        Some(binary) => binary.1,
        None => panic!("Failed to extract binary from filename"),
    };

    let name = match binary.split_once(".") {
        Some(name) => name.0,
        None => panic!("Failed to extract name from binary"),
    };

    name

}