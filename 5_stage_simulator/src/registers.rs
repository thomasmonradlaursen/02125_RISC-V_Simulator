use crate::{control::Control, decode::Decoding, execute::Computation, mem_access::MemoryResult, printer};

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

impl IFIDReg {
    pub fn print_fetch(&self) {
        println!("IFID: Fetch");
        println!("Instruction: {}", printer::to_assembly(&self.fetch.instruction));
        println!("Program counter: {}", self.fetch.pc);
        println!();
    }
    pub fn print_decode(&self) {
        println!("IFID: Decode");
        println!("Instruction: {}", printer::to_assembly(&self.decode.instruction));
        println!("Program counter: {}", self.decode.pc);
        println!();
    }
}

impl IDEXReg {
    pub fn print_decode(&self) {
        println!("IDEX: Decode");
        println!("Instruction: {}", printer::to_assembly(&self.decode.instruction));
        println!("Program counter: {}, rd: {}, rs1: {}, rs2:{}", self.decode.pc, self.decode.rd, self.decode.rs1, self.decode.rs2);
        println!("Decoding: {:?}", self.decode.decoding);
        println!();
    }
    pub fn print_execute(&self) {
        println!("IDEX: Execute");
        println!("Instruction: {}", printer::to_assembly(&self.execute.instruction));
        println!("Program counter: {}, rd: {}, rs1: {}, rs2:{}", self.execute.pc, self.execute.rd, self.execute.rs1, self.execute.rs2);
        println!("Decoding: {:?}", self.execute.decoding);
        println!();
    }
}

impl EXMEMReg {
    pub fn print_execute(&self) {
        println!("EXMEM: Execute");
        println!("Instruction: {}", printer::to_assembly(&self.execute.instruction));
        println!("Program counter: {}, rd: {}, rs1: {}, rs2:{}", self.execute.pc, self.execute.rd, self.execute.rs1, self.execute.rs2);
        println!("Computation: {:?}", self.execute.computation);
        println!();
    }
    pub fn print_mem(&self) {
        println!("EXMEM: Memory");
        println!("Instruction: {}", printer::to_assembly(&self.mem.instruction));
        println!("Program counter: {}, rd: {}, rs1: {}, rs2:{}", self.mem.pc, self.mem.rd, self.mem.rs1, self.mem.rs2);
        println!("Computation: {:?}", self.mem.computation);
        println!();
    }
}

impl MEMWBReg {
    pub fn print_mem(&self) {
        println!("MEMWB: Memory");
        println!("Instruction: {}", printer::to_assembly(&self.mem.instruction));
        println!("Program counter: {}, rd: {}, rs1: {}, rs2:{}", self.mem.pc, self.mem.rd, self.mem.rs1, self.mem.rs2);
        println!("Memory result: {:?}", self.mem.mem_result);
        println!("Control: {:?}", self.mem.control);
        println!();
    }
    pub fn print_wb(&self) {
        println!("MEMWB: Writeback");
        println!("Instruction: {}", printer::to_assembly(&self.wb.instruction));
        println!("Program counter: {}, rd: {}, rs1: {}, rs2:{}", self.wb.pc, self.wb.rd, self.wb.rs1, self.wb.rs2);
        println!("Memory result: {:?}", self.wb.mem_result);
        println!("Control: {:?}", self.wb.control);
        println!();
    }
}