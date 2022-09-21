pub struct MemoryAccess {
    pub instruction: i32,
    pub next_instruction: i32,

    pub loaded_memory: i32,
    pub next_loaded_memory: i32,

    pub destination: usize,
    pub next_destination: usize,

    pub content: i32,
    pub next_content: i32,
}

impl MemoryAccess {
    pub fn access_memory(
        &mut self,
        mem: &mut [u8; 1048576],
        address: &usize,
        content: &i32,
        opcode: &i32,
        funct3: &i32,
        destination: &usize,
    ) {
        self.destination = *destination;
        self.content = *content;
        match opcode {
            0x03 => match funct3 {
                0x00 => {
                    // LB - Load byte
                    self.loaded_memory = mem[*address] as i32;
                }
                0x01 => {
                    // LH - Load halfword
                    self.loaded_memory =
                        i32::from_be_bytes([mem[*address], mem[*address + 1], 0, 0]);
                }
                0x02 => {
                    // LW - Load word
                    self.loaded_memory = i32::from_be_bytes([
                        mem[*address],
                        mem[*address + 1],
                        mem[*address + 2],
                        mem[*address + 3],
                    ]);
                }
                0x04 => {
                    // LHU - Load halfword unsigned
                    self.loaded_memory =
                    (u32::from_be_bytes([mem[*address], mem[*address + 1], 0, 0])) as i32;
                }
                0x05 => {
                    // LWU - Load word unsigned
                    self.loaded_memory = (u32::from_be_bytes([
                        mem[*address],
                        mem[*address + 1],
                        mem[*address + 2],
                        mem[*address + 3],
                    ])) as i32;
                }
                _ => (),
            },
            0x23 => match funct3 {
                0x00 => {
                    // SB - Store byte
                    let bytes = i32::to_be_bytes(*content);
                    mem[*address] = bytes[0];
                }
                0x01 => {
                    // SH - Store halfword
                    let bytes = i32::to_be_bytes(*content);
                    mem[*address] = bytes[0];
                    mem[*address + 1] = bytes[1];
                }
                0x02 => {
                    // SW - Store word
                    let bytes = i32::to_be_bytes(*content);
                    mem[*address] = bytes[0];
                    mem[*address + 1] = bytes[1];
                    mem[*address + 2] = bytes[2];
                    mem[*address + 3] = bytes[3];
                }
                _ => (),
            },
            _ => (),
        }
    }

    pub fn update(&mut self) {
        self.next_instruction = self.instruction;
        self.next_loaded_memory = self.loaded_memory;
        self.next_destination = self.destination;
        self.next_content = self.content;
    }

    pub fn print_state(&self, instruction_string: &String) {
        println!("MEMORY ACCESS STAGE");
        println!("Instruction: {}\n", instruction_string);
    }
}
