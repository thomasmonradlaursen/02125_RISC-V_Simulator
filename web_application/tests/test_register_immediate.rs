use web_application::misc::test_runner::{self, RegisterResult};

const HAZARD: bool = true;
const FORWARD: bool = true;

#[test]
fn addi() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_addi.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn andi() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_andi.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn auipc() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_auipc.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn ori() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_ori.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn slli() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_slli.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn slti() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_slti.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn sltiu() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_sltiu.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn srai() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_srai.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn srli() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_srli.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn xori() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_xori.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}