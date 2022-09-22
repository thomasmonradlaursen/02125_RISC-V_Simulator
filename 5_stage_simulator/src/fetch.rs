pub struct Fetch {
    pub pc: usize,
    pub instruction: i32,
    pub next_instruction: i32,
}

impl Fetch {
    pub fn fetch_instruction(&mut self, mem: &[u8]) {
        println!("{}", self.pc);
        let instruction: [u8; 4] = [mem[3], mem[2], mem[1], mem[0]];
        self.instruction = i32::from_be_bytes(instruction);
    }

    pub fn update(&mut self, pc_update: usize) {
        self.pc += pc_update;
        self.next_instruction = self.instruction;
    }

    pub fn print_state(&self, instruction_string: &String) {
        print!("{}[2J", 27 as char);
        println!("FETCH STAGE");
        println!("Program counter: {}", self.pc);
        println!("Instruction: {}\n", instruction_string);
    }
}