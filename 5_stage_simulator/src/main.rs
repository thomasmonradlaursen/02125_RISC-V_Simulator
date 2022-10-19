use mylib::simulator_engine;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let hazard = match args[2].as_str() {
        "true" => true,
        _ => false,
    };
    let forward = match args[3].as_str() {
        "true" => true,
        _ => false,
    };
    simulator_engine::run_simulation(&filename, true, hazard, forward);
}