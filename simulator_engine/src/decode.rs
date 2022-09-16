pub struct Decode {
    pub instruction: i32,
    pub next_instruction: i32,
    
    pub opcode: i32,
    pub funct3: i32,
    pub funct7: i32,
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub imm3112: i32,
    pub imm110: i32,
    pub shamt: i32,
    
    pub next_opcode: i32,
    pub next_funct3: i32,
    pub next_funct7: i32,
    pub next_rd: usize,
    pub next_rs1: usize,
    pub next_rs2: usize,
    pub next_imm3112: i32,
    pub next_imm110: i32,
    pub next_shamt: i32,
}

impl Decode {
    pub fn decode_instruction(&mut self) {
        self.opcode = self.instruction & 0x7f;
        self.funct3 = (self.instruction >> 12) & 0x07;
        self.funct7 = self.instruction >> 25;
        self.rd = ((self.instruction >> 7) & 0x01f) as usize;
        self.rs1 = ((self.instruction >> 15) & 0x01f) as usize;
        self.rs2 = ((self.instruction >> 20) & 0x01f) as usize;
        self.imm3112 = (self.instruction >> 12) << 12;
        self.imm110 = self.instruction >> 20;
        self.shamt = (self.instruction >> 20) & 0x01f;
    }

    pub fn update(&mut self) {
        self.next_instruction = self.instruction;
        self.next_opcode = self.opcode;
        self.next_funct3 = self.funct3;
        self.next_funct7 = self.funct7;
        self.next_rd = self.rd;
        self.next_rs1 = self.rs1;
        self.next_rs2 = self.rs2;
        self.next_imm3112 = self.imm3112;
        self.next_imm110 = self.imm110;
        self.next_shamt = self.shamt;
    }

    pub fn print_state(&self, instruction_string: &String) {
        println!("DECODE STAGE");
        println!("Instruction: {}", instruction_string);
    }
}