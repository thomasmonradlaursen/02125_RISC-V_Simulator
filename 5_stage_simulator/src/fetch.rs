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

    pub fn update(&mut self, branch: &mut bool) {
        if !*branch {
            self.pc += 4;
        }
        self.next_instruction = self.instruction;
    }

    pub fn print_state(&self, instruction_string: &String) {
        println!("FETCH STAGE");
        println!("Program counter: {}", self.pc);
        println!("Instruction: {}\n", instruction_string);
    }
}