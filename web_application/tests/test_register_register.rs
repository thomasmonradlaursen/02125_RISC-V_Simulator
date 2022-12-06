use web_application::misc::test_runner::{self, RegisterResult};

const HAZARD: bool = true;
const FORWARD: bool = true;
const TO_FILE: bool = true;

#[test]
fn add() {
    let file = "instruction_tests/test_add.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn and() {
    let file = "instruction_tests/test_and.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn or() {
    let file = "instruction_tests/test_or.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn sll() {
    let file = "instruction_tests/test_sll.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn slt() {
    let file = "instruction_tests/test_slt.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn sltu() {
    let file = "instruction_tests/test_sltu.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}


#[test]
fn sra() {
    let file = "instruction_tests/test_sra.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn srl() {
    let file = "instruction_tests/test_srl.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn sub() {
    let file = "instruction_tests/test_sub.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn xor() {
    let file = "instruction_tests/test_xor.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}