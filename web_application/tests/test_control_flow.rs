use web_application::misc::test_runner::{self, RegisterResult};

const HAZARD: bool = true;
const FORWARD: bool = true;
const TO_FILE: bool = true;

#[test]
fn beq() {
    let file = "instruction_tests/test_beq.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn bge() {
    let file = "instruction_tests/test_bge.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn bgeu() {
    let file = "instruction_tests/test_bgeu.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn blt() {
    let file = "instruction_tests/test_blt.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn bltu() {
    let file = "instruction_tests/test_bltu.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn bne() {
    let file = "instruction_tests/test_bne.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn jal() {
    let file = "instruction_tests/test_jal.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}

#[test]
fn jalr() {
    let file = "instruction_tests/test_jalr.bin";
    test_runner::result_to_csv(file, TO_FILE);
    let res: RegisterResult = test_runner::run_test(file, HAZARD, FORWARD);
    println!("ISA:");
    res.isa.iter().for_each(|f| print!("{}, ", f));
    println!("Pipeline:");
    res.pipeline.iter().for_each(|f| print!("{}, ", f));
    assert!(res.pipeline.iter().zip(res.isa.iter()).all(|(a,b)| a == b));
}