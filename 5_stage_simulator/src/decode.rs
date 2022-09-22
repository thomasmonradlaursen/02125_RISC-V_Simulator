pub struct Decode {
    pub instruction: i32,
    pub next_instruction: i32,
    
    pub opcode: i32,
    pub funct3: i32,
    pub funct7: i32,
    pub rd: usize,
    pub rs1: i32,
    pub rs2: i32,
    pub imm3112: i32,
    pub imm110: i32,
    pub shamt: i32,
    pub s_offset: i32,
    pub sb_offset: i32,
    pub uj_offset: i32,
    
    pub next_opcode: i32,
    pub next_funct3: i32,
    pub next_funct7: i32,
    pub next_rd: usize,
    pub next_rs1: i32,
    pub next_rs2: i32,
    pub next_imm3112: i32,
    pub next_imm110: i32,
    pub next_shamt: i32,
    pub next_s_offset: i32,
    pub next_sb_offset: i32,
    pub next_uj_offset: i32,
}

impl Decode {
    pub fn decode_instruction(&mut self, reg: &[i32;32]) {
        self.opcode = self.instruction & 0x7f;
        self.funct3 = (self.instruction >> 12) & 0x07;
        self.funct7 = self.instruction >> 25;
        self.rd = ((self.instruction >> 7) & 0x01f) as usize;
        self.rs1 = reg[((self.instruction >> 15) & 0x01f) as usize];
        self.rs2 = reg[((self.instruction >> 20) & 0x01f) as usize];
        self.imm3112 = (self.instruction >> 12) << 12;
        self.imm110 = self.instruction >> 20;
        self.shamt = (self.instruction >> 20) & 0x01f;
        self.s_offset = get_s_offset(&self.instruction);
        self.sb_offset = get_sb_offset(&self.instruction);
        self.uj_offset = get_uj_offset(&self.instruction);
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
        self.next_s_offset = self.s_offset;
        self.next_sb_offset = self.sb_offset;
        self.next_uj_offset = self.uj_offset;
    }

    pub fn print_state(&self, instruction_string: &String) {
        println!("DECODE STAGE");
        println!("Instruction: {}\n", instruction_string);
    }
}

fn get_uj_offset(instruction: &i32) -> i32 {
    let bit20: i32 = (instruction >> 31) << 20; // 20
    let bit101: i32 = ((instruction >> 21) & 0x3ff) << 1; // 10 9 8 7 6 5 4 3 2 1
    let bit1912: i32 = instruction & 0xff000; // 19 18 17 16 15 14 13 12
    let bit11: i32 = (((instruction & 0x100000) as u32) >> 9) as i32; // 11
    ((bit101 | bit1912) | bit11) | bit20
}

fn get_sb_offset(instruction: &i32) -> i32 {
    let bit11 = (instruction & 0x80) << 4; // 11
    let bit12 = (instruction >> 31) << 12; // 12
    let bit41 = ((instruction >> 8) & 0x0f) << 1; // 4 3 2 1
    let bit105 = ((instruction >> 25) & 0x3f) << 5; // 10 9 8 7 6 5
    ((bit41 | bit105) | bit11) | bit12
}

fn get_s_offset(instruction: &i32) -> i32 {
    let bit40 = (instruction >> 7) & 0x1f;
    let bit115 = (instruction >> 25) << 5;
    bit40 | bit115
}