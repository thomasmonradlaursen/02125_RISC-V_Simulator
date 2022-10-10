#[derive(Copy, Clone, Debug)]
pub struct Control {
    pub reg_write: bool,
    pub alu_src: bool,
    pub mem_read: bool,
    pub mem_write: bool,
    pub mem_to_reg: bool,
    pub branch: bool,
}

impl Default for Control {
    fn default() -> Self {
        Self {
            reg_write: Default::default(),
            alu_src: Default::default(),
            mem_read: Default::default(),
            mem_write: Default::default(),
            mem_to_reg: Default::default(),
            branch: Default::default(),
        }
    }
}

impl Control {
    pub fn compute_control(opcode: &i32) -> Control {
        match opcode {
            // Load instructions
            0x03 => Control {
                reg_write: true,
                alu_src: false,
                mem_read: true,
                mem_write: false,
                mem_to_reg: true,
                branch: false,
            },
            // Immidiate instructions
            0x13 => Control {
                reg_write: true,
                alu_src: false,
                mem_read: false,
                mem_write: false,
                mem_to_reg: false,
                branch: false,
            },
            // AUIPC
            0x17 => Control {
                reg_write: true,
                alu_src: false,
                mem_read: false,
                mem_write: false,
                mem_to_reg: false,
                branch: false,
            },
            // Memory instructions
            0x23 => Control {
                reg_write: false,
                alu_src: false,
                mem_read: false,
                mem_write: true,
                mem_to_reg: false,
                branch: false,
            },
            // Add, sub...
            0x33 => Control {
                reg_write: true,
                alu_src: false,
                mem_read: false,
                mem_write: false,
                mem_to_reg: false,
                branch: false,
            },
            // LUI
            0x37 => Control {
                reg_write: true,
                alu_src: false,
                mem_read: false,
                mem_write: false,
                mem_to_reg: false,
                branch: false,
            },
            // Branch
            0x63 => Control {
                reg_write: false,
                alu_src: true,
                mem_read: false,
                mem_write: false,
                mem_to_reg: false,
                branch: true,
            },
            // JAL
            0x67 => Control {
                reg_write: true,
                alu_src: false,
                mem_read: false,
                mem_write: false,
                mem_to_reg: false,
                branch: true,
            },
            // JALR
            0x6F => Control {
                reg_write: true,
                alu_src: false,
                mem_read: false,
                mem_write: false,
                mem_to_reg: false,
                branch: true,
            },
            // ECALL
            0x73 => Control {
                reg_write: false,
                alu_src: false,
                mem_read: false,
                mem_write: false,
                mem_to_reg: false,
                branch: false,
            },
            _ => Control {
                ..Default::default()
            },
        }
    }
}