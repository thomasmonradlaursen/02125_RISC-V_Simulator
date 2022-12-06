use web_application::misc::test_runner::{self, RegisterResult};

const HAZARD: bool = true;
const FORWARD: bool = true;
const TO_FILE: bool = true;

#[test]
fn addi() {
    let file = "instruction_tests/test_addi.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn andi() {
    let file = "instruction_tests/test_andi.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn auipc() {
    let file = "instruction_tests/test_auipc.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn ori() {
    let file = "instruction_tests/test_ori.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn slli() {
    let file = "instruction_tests/test_slli.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn slti() {
    let file = "instruction_tests/test_slti.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn sltiu() {
    let file = "instruction_tests/test_sltiu.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn srai() {
    let file = "instruction_tests/test_srai.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn srli() {
    let file = "instruction_tests/test_srli.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn xori() {
    let file = "instruction_tests/test_xori.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}