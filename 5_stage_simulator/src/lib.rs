// Pipeline stages
pub mod fetch;
pub mod decode;
pub mod execute;
pub mod mem_access;
pub mod writeback;

// Control logic
pub mod registers;
pub mod control;
pub mod hazard;
pub mod forward;

// Engine
pub mod simulator_engine;

// Utility
pub mod printer;
pub mod documentation;