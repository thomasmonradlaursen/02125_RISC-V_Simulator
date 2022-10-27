use simulator::{simulator_engine, simulator_engine::SimulatorEngine};
use std::env;

fn main() {
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

    let mut simulator_engine: SimulatorEngine = SimulatorEngine {
        ..Default::default()
    };

    simulator_engine::run_simulation(&filename, &mut simulator_engine, stepwise, hazard, forward);
}
