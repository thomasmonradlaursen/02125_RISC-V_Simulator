use crate::components;

pub fn hazard_forwarding_unit() -> Vec<f32> {
    let mut res: Vec<f32> = vec![];
    let mut hazard_unit: Vec<f32> = hazard_unit();
    let mut forwarding_unit: Vec<f32> = forwarding_unit();
    res.append(&mut hazard_unit);
    res.append(&mut forwarding_unit);
    res
}

pub fn hazard_only_unit() -> Vec<f32> {
    let mut res: Vec<f32> = vec![];
    let mut hazard_unit: Vec<f32> = hazard_unit();
    let mut hazard_replace_unit: Vec<f32> = forwarding_unit();
    res.append(&mut hazard_unit);
    res.append(&mut hazard_replace_unit);
    res
}

pub fn hazard_unit() -> Vec<f32> {
    let (width, height) = (1160.0, 600.0);
    let hazard_unit: Vec<f32> = components::config_unit(290.0, 510.0, width, height);
    hazard_unit
}

pub fn forwarding_unit() -> Vec<f32> {
    let (width, height) = (1160.0, 600.0);
    let forwarding_unit: Vec<f32> = components::config_unit(655.0, 10.0, width, height);
    forwarding_unit
}

pub fn simple_pipeline(hazard: bool, forwarding: bool) -> Vec<f32> {
    let (width, height) = (1160.0, 600.0);

    let mut components: Vec<Vec<f32>> = vec![];
    let mut vertices: Vec<f32> = vec![];
    // Create registers
    let pc: Vec<f32> = components::pc(70.0, 260.0, width, height);
    let if_id: Vec<f32> = components::register(220.0, 110.0, width, height);
    let id_ex: Vec<f32> = components::register(450.0, 110.0, width, height);
    let ex_mem: Vec<f32> = components::register(690.0, 110.0, width, height);
    let mem_wb: Vec<f32> = components::register(920.0, 110.0, width, height);

    // Create memories
    let instruction_mem: Vec<f32> = components::mem(125.0, 220.0, width, height);
    let registers: Vec<f32> = components::mem(340.0, 220.0, width, height);
    let data_mem: Vec<f32> = components::mem(780.0, 160.0, width, height);

    // Immidiate generator
    let imm_gen: Vec<f32> = components::imm_gen(340.0, 160.0, width, height);

    // Control
    let control: Vec<f32> = components::imm_gen(340.0, 400.0, width, height);

    // Create logic gates
    let alu: Vec<f32> = components::alu(620.0, 220.0, width, height);
    let mux_in: Vec<f32> = components::multiplexer(580.0, 200.0, width, height);
    let mux_rs1: Vec<f32> = components::multiplexer(580.0, 325.0, width, height);
    let mux_rs1_forward: Vec<f32> = components::multiplexer(540.0, 305.0, width, height);
    let mux_rs2_forward: Vec<f32> = components::multiplexer(540.0, 225.0, width, height);
    let mux_wb: Vec<f32> = components::multiplexer(1000.0, 290.0, width, height);
    let pc_adder: Vec<f32> = components::adder(150.0, 395.0, width, height);
    let pc_mux: Vec<f32> = components::multiplexer(30.0, 265.0, width, height);
    let pc_update_adder: Vec<f32> = components::adder(620.0, 510.0, width, height);
    let pc_update_mux: Vec<f32> = components::multiplexer(580.0, 485.0, width, height);
    let pc_imm_mux: Vec<f32> = components::reverse_multiplexer(20.0, 485.0, width, height);

    // Wires
    // Fetch
    let wire_mux_pc: Vec<f32> = components::wire((50.0, 300.0), (70.0, 300.0), width, height);
    let wire_pc_mem: Vec<f32> = components::wire((100.0, 300.0), (125.0, 300.0), width, height);
    let wire_mem_ifid: Vec<f32> = components::wire((195.0, 300.0), (220.0, 300.0), width, height);
    let wire_add_plus_4: Vec<f32> = components::wire((140.0, 455.0), (150.0, 455.0), width, height);
    let wire_pc_adder_1: Vec<f32> = components::wire((115.0, 300.0), (115.0, 405.0), width, height);
    let wire_pc_adder_2: Vec<f32> = components::wire((115.0, 405.0), (150.0, 405.0), width, height);
    let wire_pc_idex_0: Vec<f32> = components::wire((115.0, 390.0), (220.0, 390.0), width, height);
    let wire_adder_mux_1: Vec<f32> = components::wire((170.0, 430.0), (210.0, 430.0), width, height);
    let wire_adder_mux_2: Vec<f32> = components::wire((180.0, 430.0), (180.0, 475.0), width, height);
    let wire_adder_mux_3: Vec<f32> = components::wire((180.0, 475.0), (20.0, 475.0), width, height);
    let wire_adder_mux_4: Vec<f32> = components::wire((20.0, 475.0), (20.0, 325.0), width, height);
    let wire_adder_mux_5: Vec<f32> = components::wire((20.0, 325.0), (30.0, 325.0), width, height);
    let wire_pc4_idex_1: Vec<f32> = components::wire((210.0, 430.0), (210.0, 470.0), width, height);
    let wire_pc4_idex_2: Vec<f32> = components::wire((210.0, 470.0), (220.0, 470.0), width, height);

    // Decode
    let wire_ifid_split: Vec<f32> = components::wire((240.0, 300.0), (280.0, 300.0), width, height);
    let wire_ifid_bar: Vec<f32> = components::wire((280.0, 120.0), (280.0, 425.0), width, height);
    let wire_ifid_rs1: Vec<f32> = components::wire((280.0, 360.0), (340.0, 360.0), width, height);
    let wire_ifid_rs2: Vec<f32> = components::wire((280.0, 320.0), (340.0, 320.0), width, height);
    let wire_ifid_imm: Vec<f32> = components::wire((280.0, 185.0), (340.0, 185.0), width, height);
    let wire_ifid_rd_idex: Vec<f32> = components::wire((280.0, 120.0), (450.0, 120.0), width, height);
    let wire_ifid_control: Vec<f32> = components::wire((280.0, 425.0), (340.0, 425.0), width, height);
    let wire_control_idex_1: Vec<f32> = components::wire((410.0, 425.0), (450.0, 425.0), width, height);
    let wire_control_idex_2: Vec<f32> = components::wire((430.0, 425.0), (430.0, 450.0), width, height);
    let wire_control_idex_3: Vec<f32> = components::wire((430.0, 450.0), (450.0, 450.0), width, height);
    let wire_control_idex_4: Vec<f32> = components::wire((430.0, 425.0), (430.0, 400.0), width, height);
    let wire_control_idex_5: Vec<f32> = components::wire((430.0, 400.0), (450.0, 400.0), width, height);
    let wire_reg_rs1: Vec<f32> = components::wire((410.0, 340.0), (450.0, 340.0), width, height);
    let wire_reg_rs2: Vec<f32> = components::wire((410.0, 260.0), (450.0, 260.0), width, height);
    let wire_imm_idex: Vec<f32> = components::wire((410.0, 185.0), (450.0, 185.0), width, height);
    let wire_idex_pc_exmem: Vec<f32> = components::wire((240.0, 390.0), (450.0, 390.0), width, height);
    let wire_idex_pc4_exmem: Vec<f32> = components::wire((240.0, 470.0), (450.0, 470.0), width, height);
    
    // Execute
    let wire_rs1_mux: Vec<f32> = components::wire((470.0, 340.0), (540.0, 340.0), width, height);
    let wire_rs1_nof_mux: Vec<f32> = components::wire((470.0, 340.0), (570.0, 340.0), width, height);
    let wire_pc_mux_1: Vec<f32> = components::wire((470.0, 390.0), (560.0, 390.0), width, height);
    let wire_pc_mux_2: Vec<f32> = components::wire((560.0, 390.0), (560.0, 380.0), width, height);
    let wire_pc_mux_3: Vec<f32> = components::wire((560.0, 380.0), (580.0, 380.0), width, height);
    let wire_rs2_mux: Vec<f32> = components::wire((470.0, 260.0), (580.0, 260.0), width, height);
    let wire_rs2_mux_f: Vec<f32> = components::wire((470.0, 260.0), (540.0, 260.0), width, height);
    let wire_mux_rs2_mux_f: Vec<f32> = components::wire((560.0, 260.0), (580.0, 260.0), width, height);
    let wire_imm_rs2_1: Vec<f32> = components::wire((470.0, 185.0), (560.0, 185.0), width, height);
    let wire_imm_rs2_2: Vec<f32> = components::wire((560.0, 185.0), (560.0, 210.0), width, height);
    let wire_imm_rs2_3: Vec<f32> = components::wire((560.0, 210.0), (580.0, 210.0), width, height);
    let wire_rs2_exmem_1: Vec<f32> = components::wire((570.0, 260.0), (570.0, 175.0), width, height);
    let wire_rs2_exmem_2: Vec<f32> = components::wire((570.0, 175.0), (690.0, 175.0), width, height);
    let wire_mux_rs2_alu: Vec<f32> = components::wire((600.0, 235.0), (620.0, 235.0), width, height);
    let wire_alu_exmem: Vec<f32> = components::wire((660.0, 300.0), (690.0, 300.0), width, height);
    let wire_idex_rd_exmem: Vec<f32> = components::wire((470.0, 120.0), (690.0,120.0), width, height);
    let wire_idex_control_exmem: Vec<f32> = components::wire((470.0, 425.0), (690.0, 425.0), width, height);
    let wire_idex_control_memwb: Vec<f32> = components::wire((470.0, 450.0), (690.0, 450.0), width, height);
    
    let wire_idex_pc_3: Vec<f32> = components::wire((20.0, 520.0), (10.0, 520.0), width, height);
    let wire_idex_pc_4: Vec<f32> = components::wire((10.0, 520.0), (10.0, 275.0), width, height);
    let wire_idex_pc_5: Vec<f32> = components::wire((10.0, 275.0), (30.0, 275.0), width, height);

    let wire_idex_pc_adder_1: Vec<f32> = components::wire((470.0, 470.0), (480.0, 470.0), width, height);
    let wire_idex_pc_adder_2: Vec<f32> = components::wire((480.0, 470.0), (480.0, 580.0), width, height);
    let wire_idex_pc_adder_3: Vec<f32> = components::wire((480.0, 580.0), (70.0, 580.0), width, height);
    let wire_idex_pc_adder_4: Vec<f32> = components::wire((70.0, 580.0), (70.0, 500.0), width, height);
    let wire_idex_pc_adder_5: Vec<f32> = components::wire((70.0, 500.0), (40.0, 500.0), width, height);
    let wire_mux_forward_mux: Vec<f32> = components::wire((560.0, 340.0), (580.0, 340.0), width, height);
    let wire_mux_rs1_alu: Vec<f32> = components::wire((600.0, 360.0), (620.0, 360.0), width, height);

    // Write back to pc - different configurations
    let wire_branch_pc_adder_1: Vec<f32> = components::wire((560.0, 390.0), (560.0, 545.0), width, height);
    let wire_branch_pc_adder_2: Vec<f32> = components::wire((560.0, 545.0), (580.0, 545.0), width, height);

    let wire_branch_rs1_adder_1: Vec<f32> = components::wire((570.0, 340.0), (570.0, 495.0), width, height);
    let wire_branch_rs1_adder_2: Vec<f32> = components::wire((570.0, 495.0), (580.0, 495.0), width, height);

    let wire_branch_mux_adder: Vec<f32> = components::wire((600.0, 520.0), (620.0, 520.0), width, height);

    let wire_imm_pc_1: Vec<f32> = components::wire((500.0, 185.0), (500.0, 570.0), width, height);
    let wire_imm_pc_2: Vec<f32> = components::wire((500.0, 570.0), (620.0, 570.0), width, height);
    
    let wire_branch_adder_1: Vec<f32> = components::wire((640.0, 545.0), (660.0, 545.0), width, height);
    let wire_branch_adder_2: Vec<f32> = components::wire((660.0, 545.0), (660.0, 590.0), width, height);
    let wire_branch_adder_3: Vec<f32> = components::wire((660.0, 590.0), (60.0, 590.0), width, height);
    let wire_branch_adder_4: Vec<f32> = components::wire((60.0, 590.0), (60.0, 540.0), width, height);
    let wire_branch_adder_5: Vec<f32> = components::wire((60.0, 540.0), (40.0, 540.0), width, height);

    components.push(wire_branch_pc_adder_1);
    components.push(wire_branch_pc_adder_2);
    components.push(wire_branch_rs1_adder_1);
    components.push(wire_branch_rs1_adder_2);
    components.push(wire_branch_mux_adder);
    components.push(wire_branch_adder_1);
    components.push(wire_branch_adder_2);
    components.push(wire_branch_adder_3);
    components.push(wire_branch_adder_4);
    components.push(wire_branch_adder_5);
    
    // Memory access
    let wire_alu_memwb_1: Vec<f32> = components::wire((740.0, 300.0), (740.0, 350.0), width, height);
    let wire_alu_memwb_2: Vec<f32> = components::wire((740.0, 350.0), (920.0, 350.0), width, height);
    let wire_exmem_mem: Vec<f32> = components::wire((710.0, 300.0), (780.0, 300.0), width, height);
    let wire_rs2_mem: Vec<f32> = components::wire((710.0, 175.0), (780.0, 175.0), width, height);
    let wire_mem_memwb: Vec<f32> = components::wire((850.0, 300.0), (920.0, 300.0), width, height);
    let wire_exmem_rd_memwb: Vec<f32> = components::wire((710.0, 120.0), (920.0, 120.0), width, height);
    let wire_exmem_control_memwb: Vec<f32> = components::wire((710.0, 450.0), (920.0, 450.0), width, height);

    // Writeback
    let wire_memwb_mux: Vec<f32> =
        components::wire((940.0, 300.0), (1000.0, 300.0), width, height);
    let wire_alu_mux: Vec<f32> = components::wire((940.0, 350.0), (1000.0, 350.0), width, height);
    let wire_mux_reg_1: Vec<f32> =
        components::wire((1020.0, 325.0), (1040.0, 325.0), width, height);
    let wire_mux_reg_2: Vec<f32> =
        components::wire((1040.0, 325.0), (1040.0, 90.0), width, height);
    let wire_mux_reg_3: Vec<f32> = components::wire((1040.0, 90.0), (260.0, 90.0), width, height);
    let wire_mux_reg_4: Vec<f32> = components::wire((260.0, 90.0), (260.0, 240.0), width, height);
    let wire_mux_reg_5: Vec<f32> = components::wire((260.0, 240.0), (340.0, 240.0), width, height);
    let wire_memwb_rd_reg_1: Vec<f32> = components::wire((940.0, 120.0), (960.0, 120.0), width, height);
    let wire_memwb_rd_reg_2: Vec<f32> = components::wire((960.0, 120.0), (960.0, 100.0), width, height);
    let wire_memwb_rd_reg_3: Vec<f32> = components::wire((960.0, 100.0), (270.0, 100.0), width, height);
    let wire_memwb_rd_reg_4: Vec<f32> = components::wire((270.0, 100.0), (270.0, 275.0), width, height);
    let wire_memwb_rd_reg_5: Vec<f32> = components::wire((270.0, 275.0), (340.0, 275.0), width, height);
    
    // Forwarding / Hazards
    let wire_ifid_rs1_idex_1: Vec<f32> = components::wire((310.0, 360.0), (310.0, 150.0), width, height);
    let wire_ifid_rs1_idex_2: Vec<f32> = components::wire((310.0, 150.0), (450.0, 150.0), width, height);
    let wire_ifid_rs2_idex_1: Vec<f32> = components::wire((300.0, 320.0), (300.0, 140.0), width, height);
    let wire_ifid_rs2_idex_2: Vec<f32> = components::wire((300.0, 140.0), (450.0, 140.0), width, height);

    let wire_idex_rs1_forward_1: Vec<f32> = components::wire((470.0, 150.0), (500.0, 150.0), width, height);
    let wire_idex_rs1_forward_2: Vec<f32> = components::wire((500.0, 150.0), (500.0, 60.0), width, height);
    let wire_idex_rs1_forward_3: Vec<f32> = components::wire((500.0, 60.0), (655.0, 60.0), width, height);

    let wire_idex_rs2_forward_1: Vec<f32> = components::wire((470.0, 140.0), (490.0, 140.0), width, height);
    let wire_idex_rs2_forward_2: Vec<f32> = components::wire((490.0, 140.0), (490.0, 50.0), width, height);
    let wire_idex_rs2_forward_3: Vec<f32> = components::wire((490.0, 50.0), (655.0, 50.0), width, height);
    
    let wire_exmem_rd_forward_1: Vec<f32> = components::wire((765.0, 120.0), (765.0, 60.0), width, height);
    let wire_exmem_rd_forward_2: Vec<f32> = components::wire((765.0, 60.0), (745.0, 60.0), width, height);
    let wire_memwb_rd_forward_1: Vec<f32> = components::wire((960.0, 120.0), (960.0, 40.0), width, height);
    let wire_memwb_rd_forward_2: Vec<f32> = components::wire((960.0, 40.0), (745.0, 40.0), width, height);

    // Forwarding
    let wire_exmem_res_forward_1: Vec<f32> = components::wire((740.0, 350.0), (740.0, 520.0), width, height);
    let wire_exmem_res_forward_2: Vec<f32> = components::wire((740.0, 520.0), (660.0, 520.0), width, height);
    let wire_exmem_res_forward_3: Vec<f32> = components::wire((660.0, 520.0), (660.0, 460.0), width, height);
    let wire_exmem_res_forward_4: Vec<f32> = components::wire((660.0, 460.0), (520.0, 460.0), width, height);
    let wire_exmem_res_forward_5: Vec<f32> = components::wire((520.0, 460.0), (520.0, 285.0), width, height);
    let wire_exmem_res_forward_6: Vec<f32> = components::wire((520.0, 365.0), (540.0, 365.0), width, height);
    let wire_exmem_res_forward_7: Vec<f32> = components::wire((520.0, 285.0), (540.0, 285.0), width, height);

    let wire_memwb_res_forward_1: Vec<f32> = components::wire((530.0, 90.0), (530.0, 315.0), width, height);
    let wire_memwb_res_forward_2: Vec<f32> = components::wire((530.0, 235.0), (540.0, 235.0), width, height);
    let wire_memwb_res_forward_3: Vec<f32> = components::wire((530.0, 315.0), (540.0, 315.0), width, height);

    let wire_forward_mux_rs1_1: Vec<f32> = components::wire((610.0, 30.0), (655.0, 30.0), width, height);
    let wire_forward_mux_rs1_2: Vec<f32> = components::wire((610.0, 30.0), (610.0, 300.0), width, height);
    let wire_forward_mux_rs1_3: Vec<f32> = components::wire((610.0, 300.0), (550.0, 300.0), width, height);
    let wire_forward_mux_rs1_4: Vec<f32> = components::wire((550.0, 300.0), (550.0, 315.0), width, height);

    let wire_forward_mux_rs2_1: Vec<f32> = components::wire((550.0, 40.0), (655.0, 40.0), width, height);
    let wire_forward_mux_rs2_2: Vec<f32> = components::wire((550.0, 40.0), (550.0, 235.0), width, height);

    // Hazard
    let wire_ifid_rs1_hazard: Vec<f32> = components::wire((315.0, 360.0), (315.0, 510.0), width, height);
    let wire_ifid_rs2_hazard: Vec<f32> = components::wire((295.0, 320.0), (295.0, 510.0), width, height);

    let wire_idex_control_hazard_1: Vec<f32> = components::wire((490.0,425.0), (490.0,540.0), width, height);
    let wire_idex_control_hazard_2: Vec<f32> = components::wire((490.0,540.0), (380.0,540.0), width, height);

    let wire_idex_rd_hazard_1: Vec<f32> = components::wire((510.0,120.0), (510.0,550.0), width, height);
    let wire_idex_rd_hazard_2: Vec<f32> = components::wire((510.0,550.0), (380.0,550.0), width, height);

    let wire_hazard_idex_1: Vec<f32> = components::wire((380.0,530.0), (460.0,530.0), width, height); 
    let wire_hazard_idex_2: Vec<f32> = components::wire((460.0,530.0), (460.0,490.0), width, height);
    
    let wire_hazard_ifid_1: Vec<f32> = components::wire((290.0,530.0), (230.0,530.0), width, height); 
    let wire_hazard_ifid_2: Vec<f32> = components::wire((230.0,530.0), (230.0,490.0), width, height);

    let wire_hazard_pc_1: Vec<f32> = components::wire((290.0,540.0), (85.0,540.0), width, height); 
    let wire_hazard_pc_2: Vec<f32> = components::wire((85.0,540.0), (85.0,340.0), width, height);

    let wire_alu_branch_hazard_1: Vec<f32> = components::wire((660.0, 280.0), (675.0, 280.0), width, height);
    let wire_alu_branch_hazard_2: Vec<f32> = components::wire((675.0, 280.0), (675.0, 80.0), width, height);
    let wire_alu_branch_hazard_3: Vec<f32> = components::wire((675.0, 80.0), (250.0, 80.0), width, height);
    let wire_alu_branch_hazard_4: Vec<f32> = components::wire((250.0, 80.0), (250.0, 520.0), width, height);
    let wire_alu_branch_hazard_5: Vec<f32> = components::wire((250.0, 520.0), (290.0, 520.0), width, height);

    let wire_hazard_replace_idex_1: Vec<f32> = components::wire((460.0, 40.0), (655.0, 40.0), width, height);
    let wire_hazard_replace_idex_2: Vec<f32> = components::wire((460.0, 40.0), (460.0, 110.0), width, height);

    let wire_hazard_replace_ifid_1: Vec<f32> = components::wire((230.0, 30.0), (655.0, 30.0), width, height);
    let wire_hazard_replace_ifid_2: Vec<f32> = components::wire((230.0, 30.0), (230.0, 110.0), width, height);

    let wire_hazard_replace_pc_1: Vec<f32> = components::wire((85.0, 20.0), (655.0, 20.0), width, height);
    let wire_hazard_replace_pc_2: Vec<f32> = components::wire((85.0, 20.0), (85.0, 260.0), width, height);

    // Hazard replacement for forwarding
    let wire_exmem_control_hazard_1: Vec<f32> = components::wire((710.0, 425.0), (740.0, 425.0), width, height);
    let wire_exmem_control_hazard_2: Vec<f32> = components::wire((740.0, 425.0), (740.0, 520.0), width, height);
    let wire_exmem_control_hazard_3: Vec<f32> = components::wire((740.0, 520.0), (660.0, 520.0), width, height);
    let wire_exmem_control_hazard_4: Vec<f32> = components::wire((660.0, 520.0), (660.0, 460.0), width, height);
    let wire_exmem_control_hazard_5: Vec<f32> = components::wire((660.0, 460.0), (520.0, 460.0), width, height);
    let wire_exmem_control_hazard_6: Vec<f32> = components::wire((520.0, 460.0), (520.0, 560.0), width, height);
    let wire_exmem_control_hazard_7: Vec<f32> = components::wire((520.0, 560.0), (380.0, 560.0), width, height);
    
    // Add forwarding / hazard components
    if forwarding || hazard {
        components.push(wire_exmem_rd_forward_1);
        components.push(wire_exmem_rd_forward_2);
        components.push(wire_memwb_rd_forward_1);
        components.push(wire_memwb_rd_forward_2);
        components.push(wire_ifid_rs1_idex_1);
        components.push(wire_ifid_rs1_idex_2);
        components.push(wire_ifid_rs2_idex_1);
        components.push(wire_ifid_rs2_idex_2);
        components.push(wire_idex_rs1_forward_1);
        components.push(wire_idex_rs1_forward_2);
        components.push(wire_idex_rs1_forward_3);
        components.push(wire_idex_rs2_forward_1);
        components.push(wire_idex_rs2_forward_2);
        components.push(wire_idex_rs2_forward_3);
    }

    if forwarding {
        components.push(wire_exmem_res_forward_1);
        components.push(wire_exmem_res_forward_2);
        components.push(wire_exmem_res_forward_3);
        components.push(wire_exmem_res_forward_4);
        components.push(wire_exmem_res_forward_5);
        components.push(wire_exmem_res_forward_6);
        components.push(wire_exmem_res_forward_7);
        components.push(wire_memwb_res_forward_1);
        components.push(wire_memwb_res_forward_2);
        components.push(wire_memwb_res_forward_3);
        components.push(wire_forward_mux_rs1_1);
        components.push(wire_forward_mux_rs1_2);
        components.push(wire_forward_mux_rs1_3);
        components.push(wire_forward_mux_rs1_4);
        components.push(wire_forward_mux_rs2_1);
        components.push(wire_forward_mux_rs2_2);
    }
    
    if hazard {
        components.push(wire_ifid_rs1_hazard);
        components.push(wire_ifid_rs2_hazard);
        components.push(wire_idex_control_hazard_1);
        components.push(wire_idex_control_hazard_2);
        components.push(wire_idex_rd_hazard_1);
        components.push(wire_idex_rd_hazard_2);
        components.push(wire_hazard_idex_1);
        components.push(wire_hazard_idex_2);
        components.push(wire_hazard_ifid_1);
        components.push(wire_hazard_ifid_2);
        components.push(wire_hazard_pc_1);
        components.push(wire_hazard_pc_2);
        
        components.push(wire_alu_branch_hazard_1);
        components.push(wire_alu_branch_hazard_2);
        components.push(wire_alu_branch_hazard_3);
        components.push(wire_alu_branch_hazard_4);
        components.push(wire_alu_branch_hazard_5);
    }

    if hazard && !forwarding {
        components.push(wire_exmem_control_hazard_1);
        components.push(wire_exmem_control_hazard_2);
        components.push(wire_exmem_control_hazard_3);
        components.push(wire_exmem_control_hazard_4);
        components.push(wire_exmem_control_hazard_5);
        components.push(wire_exmem_control_hazard_6);
        components.push(wire_exmem_control_hazard_7);
        components.push(wire_hazard_replace_idex_1);
        components.push(wire_hazard_replace_idex_2);
        components.push(wire_hazard_replace_ifid_1);
        components.push(wire_hazard_replace_ifid_2);
        components.push(wire_hazard_replace_pc_1);
        components.push(wire_hazard_replace_pc_2);
    }

    // Add registers to list of components
    components.push(pc);
    components.push(if_id);
    components.push(id_ex);
    components.push(ex_mem);
    components.push(mem_wb);

    // Add logic
    components.push(instruction_mem);
    components.push(alu);
    components.push(mux_rs1);
    if forwarding {
        components.push(mux_rs1_forward);
        components.push(mux_rs2_forward);
    }
    components.push(mux_in);
    components.push(imm_gen);
    components.push(control);
    components.push(registers);
    components.push(data_mem);
    components.push(mux_wb);
    components.push(pc_adder);
    components.push(pc_mux);
    //components.push(pc_mux_2);
    components.push(pc_update_adder);
    components.push(pc_update_mux);
    components.push(pc_imm_mux);

    // Add wires
    // Fetch
    components.push(wire_mux_pc);
    components.push(wire_pc_mem);
    components.push(wire_mem_ifid);
    components.push(wire_add_plus_4);
    components.push(wire_pc_adder_1);
    components.push(wire_pc_adder_2);
    components.push(wire_pc_idex_0);
    components.push(wire_adder_mux_1);
    components.push(wire_adder_mux_2);
    components.push(wire_adder_mux_3);
    components.push(wire_adder_mux_4);
    components.push(wire_adder_mux_5);
    components.push(wire_pc4_idex_1);
    components.push(wire_pc4_idex_2);
    
    // Decode
    components.push(wire_ifid_split);
    components.push(wire_ifid_bar);
    components.push(wire_ifid_rs1);
    components.push(wire_ifid_rs2);
    components.push(wire_ifid_imm);
    components.push(wire_reg_rs1);
    components.push(wire_reg_rs2);
    components.push(wire_ifid_rd_idex);
    components.push(wire_imm_idex);
    components.push(wire_ifid_control);
    components.push(wire_control_idex_1);
    components.push(wire_control_idex_2);
    components.push(wire_control_idex_3);
    components.push(wire_control_idex_4);
    components.push(wire_control_idex_5);
    components.push(wire_idex_pc_exmem);
    components.push(wire_idex_pc4_exmem);
    
    //Execute
    components.push(wire_rs1_mux);
    if !forwarding {
        components.push(wire_rs1_nof_mux);
        components.push(wire_rs2_mux);
    }
    components.push(wire_rs2_mux_f);
    components.push(wire_mux_rs2_mux_f);
    components.push(wire_pc_mux_1);
    components.push(wire_pc_mux_2);
    components.push(wire_pc_mux_3);
    components.push(wire_imm_rs2_1);
    components.push(wire_imm_rs2_2);
    components.push(wire_imm_rs2_3);
    components.push(wire_rs2_exmem_1);
    components.push(wire_rs2_exmem_2);
    components.push(wire_mux_rs2_alu);
    components.push(wire_idex_rd_exmem);
    components.push(wire_idex_control_exmem);
    components.push(wire_idex_control_memwb);
    components.push(wire_imm_pc_1);
    components.push(wire_imm_pc_2);
    components.push(wire_idex_pc_3);
    components.push(wire_idex_pc_4);
    components.push(wire_idex_pc_5);
    components.push(wire_idex_pc_adder_1);
    components.push(wire_idex_pc_adder_2);
    components.push(wire_idex_pc_adder_3);
    components.push(wire_idex_pc_adder_4);
    components.push(wire_idex_pc_adder_5);
    components.push(wire_mux_forward_mux);
    components.push(wire_mux_rs1_alu);
    
    
    // Memory access
    components.push(wire_alu_memwb_1);
    components.push(wire_alu_memwb_2);
    components.push(wire_alu_exmem);
    components.push(wire_exmem_mem);
    components.push(wire_rs2_mem);
    components.push(wire_mem_memwb);
    components.push(wire_exmem_rd_memwb);
    components.push(wire_exmem_control_memwb);
    
    // Writeback
    components.push(wire_memwb_mux);
    components.push(wire_alu_mux);
    components.push(wire_mux_reg_1);
    components.push(wire_mux_reg_2);
    components.push(wire_mux_reg_3);
    components.push(wire_mux_reg_4);
    components.push(wire_mux_reg_5);
    components.push(wire_memwb_rd_reg_1);
    components.push(wire_memwb_rd_reg_2);
    components.push(wire_memwb_rd_reg_3);
    components.push(wire_memwb_rd_reg_4);
    components.push(wire_memwb_rd_reg_5);

    // Hazard and forwarding units
    if hazard && forwarding {
        components.push(hazard_forwarding_unit());
    }
    else if hazard {
        components.push(hazard_only_unit());
    }
    else if forwarding {
        components.push(forwarding_unit());
    }

    for mut component in components {
        vertices.append(&mut component);
    }

    vertices
}
