use crate::components;

pub fn simple_pipeline() -> Vec<f32> {
    let (width, height) = (1160.0, 600.0);

    let mut components: Vec<Vec<f32>> = vec![];
    let mut vertices: Vec<f32> = vec![];
    // Create registers
    let pc: Vec<f32> = components::pc(60.0, 260.0, width, height);
    let if_id: Vec<f32> = components::register(220.0, 110.0, width, height);
    let id_ex: Vec<f32> = components::register(450.0, 110.0, width, height);
    let ex_mem: Vec<f32> = components::register(690.0, 110.0, width, height);
    let mem_wb: Vec<f32> = components::register(920.0, 110.0, width, height);

    // Create memories
    let instruction_mem: Vec<f32> = components::mem(120.0, 220.0, width, height);
    let registers: Vec<f32> = components::mem(320.0, 220.0, width, height);
    let data_mem: Vec<f32> = components::mem(780.0, 160.0, width, height);

    // Immidiate generator
    let imm_gen: Vec<f32> = components::imm_gen(320.0, 150.0, width, height);

    // Create logic gates
    let alu: Vec<f32> = components::alu(620.0, 220.0, width, height);
    let mux_in: Vec<f32> = components::multiplexer(570.0, 200.0, width, height);
    let mux_wb: Vec<f32> = components::multiplexer(1000.0, 290.0, width, height);

    // Wires
    // Fetch
    let wire_pc_mem: Vec<f32> = components::wire((90.0, 300.0), (120.0, 300.0), width, height);
    let wire_mem_ifid: Vec<f32> = components::wire((190.0, 300.0), (220.0, 300.0), width, height);

    // Decode
    let wire_ifid_split: Vec<f32> = components::wire((240.0, 300.0), (280.0, 300.0), width, height);
    let wire_ifid_bar: Vec<f32> = components::wire((280.0, 175.0), (280.0, 360.0), width, height);
    let wire_ifid_rs1: Vec<f32> = components::wire((280.0, 360.0), (320.0, 360.0), width, height);
    let wire_ifid_rs2: Vec<f32> = components::wire((280.0, 320.0), (320.0, 320.0), width, height);
    let wire_ifid_imm: Vec<f32> = components::wire((280.0, 175.0), (320.0, 175.0), width, height);
    let wire_reg_rs1: Vec<f32> = components::wire((390.0, 360.0), (450.0, 360.0), width, height);
    let wire_reg_rs2: Vec<f32> = components::wire((390.0, 260.0), (450.0, 260.0), width, height);
    
    // Execute
    let wire_idex_alu: Vec<f32> = components::wire((470.0, 360.0), (620.0, 360.0), width, height);
    let wire_idex_mux: Vec<f32> = components::wire((470.0, 260.0), (570.0, 260.0), width, height);
    let wire_imm_idex: Vec<f32> = components::wire((390.0, 175.0), (450.0, 175.0), width, height);
    let wire_imm_rs2_1: Vec<f32> = components::wire((470.0, 175.0), (500.0, 175.0), width, height);
    let wire_imm_rs2_2: Vec<f32> = components::wire((500.0, 175.0), (500.0, 210.0), width, height);
    let wire_imm_rs2_3: Vec<f32> = components::wire((500.0, 210.0), (570.0, 210.0), width, height);
    let wire_rs2_exmem_1: Vec<f32> = components::wire((540.0, 260.0), (540.0, 175.0), width, height);
    let wire_rs2_exmem_2: Vec<f32> = components::wire((540.0, 175.0), (690.0, 175.0), width, height);
    let wire_mux_alu: Vec<f32> = components::wire((590.0, 235.0), (620.0, 235.0), width, height);
    let wire_alu_exmem: Vec<f32> = components::wire((660.0, 300.0), (690.0, 300.0), width, height);
    
    // Memory access
    let wire_alu_memwb_1: Vec<f32> = components::wire((740.0, 300.0), (740.0, 350.0), width, height);
    let wire_alu_memwb_2: Vec<f32> = components::wire((740.0, 350.0), (920.0, 350.0), width, height);
    let wire_exmem_mem: Vec<f32> = components::wire((710.0, 300.0), (780.0, 300.0), width, height);
    let wire_rs2_mem: Vec<f32> = components::wire((710.0, 175.0), (780.0, 175.0), width, height);
    let wire_mem_memwb: Vec<f32> = components::wire((850.0, 300.0), (920.0, 300.0), width, height);

    // Writeback
    let wire_memwb_mux: Vec<f32> =
        components::wire((940.0, 300.0), (1000.0, 300.0), width, height);
    let wire_alu_mux: Vec<f32> = components::wire((940.0, 350.0), (1000.0, 350.0), width, height);
    let wire_mux_reg_1: Vec<f32> =
        components::wire((1020.0, 325.0), (1040.0, 325.0), width, height);
    let wire_mux_reg_2: Vec<f32> =
        components::wire((1040.0, 325.0), (1040.0, 100.0), width, height);
    let wire_mux_reg_3: Vec<f32> = components::wire((1040.0, 100.0), (250.0, 100.0), width, height);
    let wire_mux_reg_4: Vec<f32> = components::wire((250.0, 100.0), (250.0, 240.0), width, height);
    let wire_mux_reg_5: Vec<f32> = components::wire((250.0, 240.0), (320.0, 240.0), width, height);

    // Add registers to list of components
    components.push(pc);
    components.push(if_id);
    components.push(id_ex);
    components.push(ex_mem);
    components.push(mem_wb);

    // Add logic
    components.push(instruction_mem);
    components.push(alu);
    components.push(mux_in);
    components.push(imm_gen);
    components.push(registers);
    components.push(data_mem);
    components.push(mux_wb);

    // Add wires
    // Fetch
    components.push(wire_pc_mem);
    components.push(wire_mem_ifid);
    
    // Decode
    components.push(wire_ifid_split);
    components.push(wire_ifid_bar);
    components.push(wire_ifid_rs1);
    components.push(wire_ifid_rs2);
    components.push(wire_ifid_imm);
    components.push(wire_reg_rs1);
    components.push(wire_reg_rs2);
    
    //Execute
    components.push(wire_idex_alu);
    components.push(wire_idex_mux);
    components.push(wire_imm_idex);
    components.push(wire_imm_rs2_1);
    components.push(wire_imm_rs2_2);
    components.push(wire_imm_rs2_3);
    components.push(wire_rs2_exmem_1);
    components.push(wire_rs2_exmem_2);
    components.push(wire_mux_alu);
    
    // Memory access
    components.push(wire_alu_memwb_1);
    components.push(wire_alu_memwb_2);
    components.push(wire_alu_exmem);
    components.push(wire_exmem_mem);
    components.push(wire_rs2_mem);
    components.push(wire_mem_memwb);
    
    // Writeback
    components.push(wire_memwb_mux);
    components.push(wire_alu_mux);
    components.push(wire_mux_reg_1);
    components.push(wire_mux_reg_2);
    components.push(wire_mux_reg_3);
    components.push(wire_mux_reg_4);
    components.push(wire_mux_reg_5);

    for mut component in components {
        vertices.append(&mut component);
    }

    vertices
}
