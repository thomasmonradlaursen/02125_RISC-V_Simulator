use crate::{registers::{EXMEM, MEMWB}, execute::Computation};

#[derive(Debug, Clone, Copy)]
pub struct MemoryResult {
    pub read_mem: i32,
    pub alu_result: i32,
    pub alu_carry: i32,
}

impl Default for MemoryResult {
    fn default() -> Self {
        Self { read_mem: 0, alu_result: 0, alu_carry: 0 }
    }
}

pub fn update_for_writeback(mem: &mut MEMWB, wb: &mut MEMWB) {
    wb.instruction = mem.instruction;
    wb.pc = mem.pc;
    wb.rd = mem.rd;
    wb.rs1 = mem.rs1;
    wb.rs2 = mem.rs2;
    wb.control = mem.control;
    wb.mem_result = mem.mem_result;
}

pub fn memory_to_register(mem_a: &mut EXMEM, mem_b: &mut MEMWB, mem: &mut [u8; 1048576]) {
    mem_b.instruction = mem_a.instruction;
    mem_b.pc = mem_a.pc;
    mem_b.rd = mem_a.rd;
    mem_b.rs1 = mem_a.rs1;
    mem_b.rs2 = mem_a.rs2;
    mem_b.mem_result = access_memory(mem, &mem_a.computation);
    mem_b.control = mem_a.control;
}

pub fn access_memory(mem: &mut [u8], computation: &Computation) -> MemoryResult {
    let mut memory_result = MemoryResult{ read_mem: 0, alu_result: computation.result, alu_carry: computation.carry };
    match computation.mem_opcode {
        0x03 => match computation.mem_funct3 {
            0x00 => {
                // LB - Load byte
                println!("LOAD BYTE - Address: {}", computation.result);
                println!("LOAD BYTE - Address as usize: {}", computation.result as usize);
                memory_result.read_mem = (mem[computation.result as usize] as i8) as i32;
            }
            0x01 => {
                // LH - Load halfword
                memory_result.read_mem =
                    (i16::from_le_bytes([mem[computation.result as usize], mem[computation.result as usize + 1]])) as i32;
            }
            0x02 => {
                // LW - Load word
                memory_result.read_mem = i32::from_le_bytes([
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
                    (u16::from_le_bytes([mem[computation.result as usize], mem[computation.result as usize + 1]])) as i32;
            }
            _ => (),
        },
        0x23 => match computation.mem_funct3 {
            0x00 => {
                // SB - Store byte
                let bytes = i32::to_le_bytes(computation.carry);
                mem[computation.result as usize] = bytes[0];
            }
            0x01 => {
                // SH - Store halfword
                let bytes = i32::to_le_bytes(computation.carry);
                mem[computation.result as usize] = bytes[0];
                mem[computation.result as usize + 1] = bytes[1];
            }
            0x02 => {
                // SW - Store word
                let bytes = i32::to_le_bytes(computation.carry);
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
