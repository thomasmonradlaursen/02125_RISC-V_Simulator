use crate::{
    control::{Control, Fields},
    decode::Decode,
    execute::Execute,
    mem_access::MemoryAccess,
    writeback::Writeback,
};

pub struct HazardDetectionUnit {
    pub id_ex_fields: Fields,
    pub ex_mem_fields: Fields,
    pub mem_wb_fields: Fields,
    pub ex_mem_control: Control,
    pub mem_wb_control: Control,
}

impl Default for HazardDetectionUnit {
    fn default() -> Self {
        Self {
            id_ex_fields: Fields::default(),
            ex_mem_fields: Fields::default(),
            mem_wb_fields: Fields::default(),
            ex_mem_control: Control::default(),
            mem_wb_control: Control::default(),
        }
    }
}

impl HazardDetectionUnit {
    pub fn initialize_fields(
        &mut self,
        execute: &Decode,
        mem_access: &Execute,
        writeback: &MemoryAccess,
    ) {
        self.id_ex_fields
            .set_fields(execute.rd, execute.rs1_address, execute.rs2_address);
        self.ex_mem_fields.set_fields(
            mem_access.rd,
            mem_access.rs1_address,
            mem_access.rs2_address,
        );
        self.mem_wb_fields
            .set_fields(writeback.rd, writeback.rs1_address, writeback.rs2_address);
    }

    pub fn detect_hazard(&self, decode: &mut Decode) {
        println!("HAZARD DETECTION UNIT:");
        if self.ex_mem_fields.rd != 0 {
            if self.ex_mem_fields.rd == self.id_ex_fields.rs1 {
                println!(
                    "Memory access - rd: {}, Execute - rs1: {}",
                    self.ex_mem_fields.rd, self.id_ex_fields.rs1
                );
                decode.instruction = 0x3000;
            }
            if self.ex_mem_fields.rd == self.id_ex_fields.rs2 {
                println!(
                    "Memory access - rd: {}, Execute - rs2: {}",
                    self.ex_mem_fields.rd, self.id_ex_fields.rs2
                );
                decode.instruction = 0x3000;
            }
        }
        if self.mem_wb_fields.rd != 0 {
            if self.mem_wb_fields.rd == self.id_ex_fields.rs1 {
                println!(
                    "Writeback - rd: {}, Execute - rs1: {}",
                    self.mem_wb_fields.rd, self.id_ex_fields.rs1
                );
                decode.instruction = 0x3000;
            }
            if self.mem_wb_fields.rd == self.id_ex_fields.rs2 {
                println!(
                    "Writeback - rd: {}, Execute - rs2: {}",
                    self.ex_mem_fields.rd, self.id_ex_fields.rs2
                );
                decode.instruction = 0x3000;
            }
        }
        println!();
    }

    pub fn print_values(&self) {
        println!("Hazard values of fields:");
        println!(
            "Ex - rd: {}, rs1: {}, rs2: {}",
            self.id_ex_fields.rd, self.id_ex_fields.rs1, self.id_ex_fields.rs2
        );
        println!(
            "Mem - rd: {}, rs1: {}, rs2: {}",
            self.ex_mem_fields.rd, self.ex_mem_fields.rs1, self.ex_mem_fields.rs2
        );
        println!(
            "Wb - rd: {}, rs1: {}, rs2: {}",
            self.mem_wb_fields.rd, self.mem_wb_fields.rs1, self.mem_wb_fields.rs2
        );
        println!();
    }
}
