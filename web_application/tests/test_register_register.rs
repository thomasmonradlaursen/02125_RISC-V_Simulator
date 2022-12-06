use web_application::misc::test_runner::{self, RegisterResult};

const HAZARD: bool = true;
const FORWARD: bool = true;

#[test]
fn add() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_add.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn and() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_and.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn or() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_or.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn sll() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_sll.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn slt() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_slt.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn sltu() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_sltu.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}


#[test]
fn sra() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_sra.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn srl() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_srl.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn sub() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_sub.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn xor() {
    let res: RegisterResult = test_runner::run_test("instruction_tests/test_xor.bin", HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}