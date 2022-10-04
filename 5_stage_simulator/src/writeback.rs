use crate::mem_access::MemoryAccess;

pub struct Writeback {
    pub instruction: i32,
    pub next_instruction: i32,
    pub reg_write: bool,
    pub rs1_address: usize,
    pub rs2_address: usize,
    pub next_rs1_address: usize,
    pub next_rs2_address: usize,
    pub rd: usize,
    pub next_rd: usize,
}

impl Default for Writeback {
    fn default() -> Self {
        Self {
            instruction: Default::default(),
            next_instruction: Default::default(),
            reg_write: Default::default(),
            rs1_address: Default::default(),
            rs2_address: Default::default(),
            next_rs1_address: Default::default(),
            next_rs2_address: Default::default(),
            rd: Default::default(),
            next_rd: Default::default(),
        }
    }
}

impl Writeback {

    pub fn initialize_fields(&mut self, mem_access: &MemoryAccess) {
        self.instruction = mem_access.next_instruction;
        self.rs1_address = mem_access.next_rs1_address;
        self.rs2_address = mem_access.next_rs1_address;
        self.rd = mem_access.next_rd;
    }

    pub fn writeback(
        &self,
        destination: &usize,
        content: &i32,
        reg: &mut [i32; 32],
        running: &mut bool,
        reg_write: &bool,
    ) {
        if *content == -1 {
            *running = false;
        }
        if *reg_write {
            reg[*destination] = *content;
        }
    }

    pub fn update(&mut self) {
        self.next_instruction = self.instruction;
    }

    pub fn print_state(&self, instruction_string: &String) {
        println!("WRITEBACK STAGE");
        println!("Instruction: {}\n", instruction_string);
    }
}
