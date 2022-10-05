use crate::{control::Control, decode::Decoding, execute::Computation, mem_access::MemoryResult};

pub struct PC {
    pub instruction: i32,
    pub pc: usize,
}

pub struct IFID {
    pub instruction: i32,
    pub pc: usize,
}

pub struct  IDEX {
    pub instruction: i32,
    pub pc: usize,
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub control: Control,
    pub decoding: Decoding,
}

pub struct EXMEM {
    pub instruction: i32,
    pub pc: usize,
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub control: Control,
    pub computation: Computation,
}

pub struct MEMWB {
    pub instruction: i32,
    pub pc: usize,
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub control: Control,
    pub mem_result: MemoryResult,
}