use mylib::{documentation, simulator_engine};
use std::env;

fn main() {
    println!("Input format: [FILENAME] [STEPWISE] [HAZARD] [FORWARDING]");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let stepwise: bool = match args[2].as_str() {
        "true" => true,
        _ => false,
    };
    let hazard = match args[3].as_str() {
        "true" => true,
        _ => false,
    };
    let forward = match args[4].as_str() {
        "true" => true,
        _ => false,
    };
    
    simulator_engine::run_simulation(&filename, stepwise, hazard, forward);
    //documentation::test_documentation(&filename, stepwise, hazard, forward, &documentation::sw());
}