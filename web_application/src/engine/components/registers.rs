use crate::engine::{components::{control::Control}, stages::{decode::Decoding, execute::Computation, mem_access::MemoryResult}};

pub struct IFIDReg {
    pub fetch: IFID,
    pub decode: IFID,
}

pub struct IDEXReg {
    pub decode: IDEX,
    pub execute: IDEX,
}

pub struct EXMEMReg {
    pub execute: EXMEM,
    pub mem: EXMEM,
}

pub struct MEMWBReg {
    pub mem: MEMWB,
    pub wb: MEMWB,
}


pub struct IFID {
    pub instruction: i32,
    pub pc: usize,
    pub rs1: usize,
    pub rs2: usize,
}

pub struct IDEX {
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

impl Default for IFID {
    fn default() -> Self {
        Self {
            instruction: 0,
            pc: 0,
            rs1: 0,
            rs2: 0,
        }
    }
}

impl Default for IDEX {
    fn default() -> Self {
        Self {
            instruction: 0,
            pc: 0,
            rd: 0,
            rs1: 0,
            rs2: 0,
            control: Control::default(),
            decoding: Decoding::default(),
        }
    }
}

impl Default for EXMEM {
    fn default() -> Self {
        Self {
            instruction: 0,
            pc: 0,
            rd: 0,
            rs1: 0,
            rs2: 0,
            control: Control::default(),
            computation: Computation::default(),
        }
    }
}

impl Default for MEMWB {
    fn default() -> Self {
        Self {
            instruction: 0,
            pc: 0,
            rd: 0,
            rs1: 0,
            rs2: 0,
            control: Control::default(),
            mem_result: MemoryResult::default(),
        }
    }
}