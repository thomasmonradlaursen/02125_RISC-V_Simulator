pub struct Writeback {
    pub instruction: i32,
    pub next_instruction: i32,
    pub reg_write: bool,
}

impl Writeback {
    pub fn writeback(&self, destination: &usize, content: &i32, reg: &mut [i32; 32], running: &mut bool, reg_write: &bool) {
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