use crate::{registers::{EXMEM, MEMWB}, execute::Computation};

pub struct MemoryResult {
    pub read_mem: i32,
    pub alu_result: i32,
    pub alu_carry: i32,
}

pub fn update_register(ex_mem: &EXMEM, mem_wb: &mut MEMWB, memory_result: MemoryResult) {
    mem_wb.instruction = ex_mem.instruction;
    mem_wb.pc = ex_mem.pc;
    mem_wb.rd = ex_mem.rd;
    mem_wb.rs1 = ex_mem.rs1;
    mem_wb.rs2 = ex_mem.rs2;
    mem_wb.control = ex_mem.control;
    mem_wb.mem_result = memory_result;
}

pub fn access_memory(mem: &mut [u8], computation: &Computation) -> MemoryResult {
    let memory_result = MemoryResult{ read_mem: 0, alu_result: computation.result, alu_carry: computation.carry };
    match computation.mem_opcode {
        0x03 => match computation.mem_funct3 {
            0x00 => {
                // LB - Load byte
                memory_result.read_mem = (mem[computation.result as usize] as i8) as i32;
            }
            0x01 => {
                // LH - Load halfword
                memory_result.read_mem =
                    (i16::from_be_bytes([mem[computation.result as usize], mem[computation.result as usize + 1]])) as i32;
            }
            0x02 => {
                // LW - Load word
                memory_result.read_mem = i32::from_be_bytes([
                    mem[computation.result as usize],
                    mem[computation.result as usize + 1],
                    mem[computation.result as usize + 2],
                    mem[computation.result as usize + 3],
                ]);
            }
            0x04 => {
                // LBU - Load byte unsigned
                memory_result.read_mem = mem[computation.result as usize] as i32;
            }
            0x05 => {
                // LHU - Load halfword unsigned
                memory_result.read_mem =
                    (u16::from_be_bytes([mem[computation.result as usize], mem[computation.result as usize + 1]])) as i32;
            }
            _ => (),
        },
        0x23 => match computation.mem_funct3 {
            0x00 => {
                // SB - Store byte
                let bytes = i32::to_be_bytes(computation.carry);
                mem[computation.result as usize] = bytes[0];
            }
            0x01 => {
                // SH - Store halfword
                let bytes = i32::to_be_bytes(computation.carry);
                mem[computation.result as usize] = bytes[0];
                mem[computation.result as usize + 1] = bytes[1];
            }
            0x02 => {
                // SW - Store word
                let bytes = i32::to_be_bytes(computation.carry);
                mem[computation.result as usize] = bytes[0];
                mem[computation.result as usize + 1] = bytes[1];
                mem[computation.result as usize + 2] = bytes[2];
                mem[computation.result as usize + 3] = bytes[3];
            }
            _ => (),
        },
        _ => (),
    }
    memory_result
}
