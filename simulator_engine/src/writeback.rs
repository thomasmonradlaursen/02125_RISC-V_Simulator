pub struct Writeback {
    pub instruction: i32,
    pub next_instruction: i32,
}

impl Writeback {
    pub fn writeback(&self, destination: &usize, content: &i32, reg: &mut [i32; 32]) {
        reg[*destination] = *content;
        println!("Writeback: reg[{}] = {}", *destination, *content);
    }

    pub fn update(&mut self) {
        self.next_instruction = self.instruction;
    }

    pub fn print_state(&self, instruction_string: &String) {
        println!("WRITEBACK STAGE");
        println!("Instruction: {}", instruction_string);
    }
}