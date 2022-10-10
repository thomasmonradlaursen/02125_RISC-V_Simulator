use crate::{control::Control, decode::Decoding, execute::Computation, mem_access::MemoryResult, printer};

pub struct PCReg {
    pub pc: PC,
    pub fetch: PC,
}

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

pub struct PC {
    pub instruction: i32,
    pub pc: usize,
}

pub struct IFID {
    pub instruction: i32,
    pub pc: usize,
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

impl Default for PC {
    fn default() -> Self {
        Self {
            instruction: 0,
            pc: 0,
        }
    }
}

impl Default for IFID {
    fn default() -> Self {
        Self {
            instruction: 0,
            pc: 0,
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

impl PC {
    pub fn print_state(&self) {
        println!("Program counter register");
        println!("Instruction: {}", printer::to_assembly(&self.instruction));
        println!("Program counter: {}", self.pc);
        println!();
    }
}

impl IFID {
    pub fn print_state(&self) {
        println!("Fetch/decode register");
        println!("Instruction: {}", printer::to_assembly(&self.instruction));
        println!("Program counter: {}", self.pc);
        println!();
    }
}

impl IDEX {
    pub fn print_state(&self) {
        println!("Decode/execute register");
        println!("Instruction: {}", printer::to_assembly(&self.instruction));
        println!("Program counter: {}, rd: {}, rs1: {}, rs2:{}", self.pc, self.rd, self.rs1, self.rs2);
        println!("Decoding: {:?}", self.decoding);
        println!();
    }
}

impl EXMEM {
    pub fn print_state(&self) {
        println!("Execute/memory register");
        println!("Instruction: {}", printer::to_assembly(&self.instruction));
        println!("Program counter: {}, rd: {}, rs1: {}, rs2:{}", self.pc, self.rd, self.rs1, self.rs2);
        println!("Computation: {:?}", self.computation);
        println!();
    }
}

impl MEMWB {
    pub fn print_state(&self) {
        println!("Memory/writeback register");
        println!("Instruction: {}", printer::to_assembly(&self.instruction));
        println!("Program counter: {}, rd: {}, rs1: {}, rs2:{}", self.pc, self.rd, self.rs1, self.rs2);
        println!("Memory result: {:?}", self.mem_result);
        println!();
    }
}