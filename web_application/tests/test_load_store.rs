use web_application::misc::test_runner::{self, RegisterResult};

const HAZARD: bool = true;
const FORWARD: bool = true;
const TO_FILE: bool = true;

#[test]
fn lb() {
    let file = "instruction_tests/test_lb.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn lbu() {
    let file = "instruction_tests/test_lbu.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn lh() {
    let file = "instruction_tests/test_lh.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn lhu() {
    let file = "instruction_tests/test_lhu.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn li() {
    let file = "instruction_tests/test_li.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn lui() {
    let file = "instruction_tests/test_lui.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn lw() {
    let file = "instruction_tests/test_lw.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn load_use() {
    let file = "instruction_tests/test_load_use.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn sb() {
    let file = "instruction_tests/test_sb.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn sh() {
    let file = "instruction_tests/test_sh.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn sw() {
    let file = "instruction_tests/test_sw.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}