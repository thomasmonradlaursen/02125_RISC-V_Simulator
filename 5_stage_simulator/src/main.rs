use mylib::simulator_engine;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    simulator_engine::run_simulation(&filename, true);
}