// NOTE: Yew has a bunch of great examples, two of which have been used for this project.
// Source code from "file_upload" and "webgl" have been implemented in this project, and modicifed to fit the use.
// Both can be found on the following link: https://github.com/yewstack/yew/tree/master/examples

use engine::simulator::SimulatorEngine;
use graphics::pipelines;
use misc::printer;

use web_sys::{Event, HtmlInputElement, HtmlCanvasElement, WebGlRenderingContext as GL};
use yew::{html, html::Scope, html::TargetCast, Component, Context, Html, NodeRef};
use wasm_bindgen::JsCast;

use gloo_file::callbacks::FileReader;
use gloo_file::File;

use gloo_render::{request_animation_frame, AnimationFrame};

pub mod engine;
pub mod graphics;
pub mod misc;

pub enum Msg {
    LoadedBytes(String, Vec<u8>),
    Files(File),
    RunSimulator(bool),
    Hazard,
    Forwarding,
    Reset,
    Render,
    MemReg,
    HexDec,
}

pub struct Model {
    reader: Option<FileReader>,
    file: (String, Vec<u8>),
    engine: SimulatorEngine,
    hazard: bool,
    forwarding: bool,
    gl: Option<GL>,
    node_ref: NodeRef,
    render_loop: Option<AnimationFrame>,
    mem_reg: bool,
    hex_dec: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            reader: Default::default(),
            file: (String::from("Empty"), vec![]),
            engine: Default::default(),
            hazard: true,
            forwarding: true,
            gl: None,
            node_ref: NodeRef::default(),
            render_loop: None,
            mem_reg: true,
            hex_dec: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadedBytes(file_name, data) => {
                if data.len() > self.engine.mem.len() {
                    let message = format!("The file is too large. Try another that is less than {} bytes", self.engine.mem.len());
                    gloo_dialogs::alert(&message[..]);
                    return false;
                }
                self.engine = Default::default();
                self.engine.read_bytes_to_mem(&data);
                self.file = (file_name, data);
                true
            }
            Msg::Files(file) => {
                let file_name = file.name();
                let task = {
                    let file_name = file_name.clone();
                    let link = ctx.link().clone();

                    gloo_file::callbacks::read_as_bytes(&file, move |res| {
                        link.send_message(Msg::LoadedBytes(
                            file_name,
                            res.expect("failed to read file"),
                        ))
                    })
                };
                self.reader = Some(task);
                true
            }
            Msg::RunSimulator(stepwise) => {
                self.engine.run_engine(stepwise, self.hazard, self.forwarding);
                true
            }
            Msg::Reset => {
                self.engine = Default::default();
                self.engine.read_bytes_to_mem(&self.file.1);
                true
            }
            Msg::Render => {
                self.render_datapath(ctx.link());
                false
            }
            Msg::Forwarding => {
                self.forwarding = !self.forwarding;
                true
            }
            Msg::Hazard => {
                self.hazard = !self.hazard;
                true
            }
            Msg::MemReg => {
                self.mem_reg = !self.mem_reg;
                true
            }
            Msg::HexDec => {
                self.hex_dec = !self.hex_dec;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {    
        html! {
            <>
                <div class = "container">
                    <div class = "controls">
                        <input lang="en" type = "file" onchange={ctx.link().callback(move |e: Event| {
                                let mut result = Vec::new();
                                let input: HtmlInputElement = e.target_unchecked_into();
                                if let Some(files) = input.files() {
                                    let files = js_sys::try_iter(&files)
                                        .unwrap()
                                        .unwrap()
                                        .map(|v| web_sys::File::from(v.unwrap()))
                                        .map(File::from);
                                    result.extend(files);
                                }
                                Msg::Files(match result.get(0) {Some(file) => file.clone(), None => File::new("", "")})
                            })}
                        />
                        <button onclick={ctx.link().callback(|_| Msg::RunSimulator(false))}>{ "Full execution" }</button>
                        <button onclick={ctx.link().callback(|_| Msg::RunSimulator(true))}>{ "Stepwise" }</button>
                        <button onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Reset" }</button>
                        <button class={if self.hazard {"config_active"} else {"config_inactive"}} onclick={ctx.link().callback(|_| Msg::Hazard)}>{"Hazard detection"}</button>
                        <button class={if self.forwarding {"config_active"} else {"config_inactive"}} onclick={ctx.link().callback(|_| Msg::Forwarding)}>{"Data forwarding"}</button>
                    </div>
                    <div class="registers">
                    if self.mem_reg {
                        {Self::display_registers(&self.engine.reg, &self.hex_dec)}
                    } else {
                        {Self::display_memory(&self.engine.mem, &self.hex_dec)}
                    }
                    <div style="grid-row:9/10; display:grid; grid-template-columns: repeat(2, 1fr)">
                    <button onclick={ctx.link().callback(|_| Msg::MemReg)}>{ if self.mem_reg {"Memory"} else {"Registers"} }</button>
                    <button onclick={ctx.link().callback(|_| Msg::HexDec)}>{ if self.hex_dec {"Hexadecimals"} else {"Signed integers"} }</button>
                    </div>
                    </div>
                    {Self::display_instructions(&self.file)}
                    <div class="stages">
                        {Self::display_stage("Fetch", &printer::to_assembly(&self.engine.pc_instruction))}
                        {Self::display_stage("Decode", &printer::to_assembly(&self.engine.if_id.decode.instruction))}
                        {Self::display_stage("Execute", &printer::to_assembly(&self.engine.id_ex.execute.instruction))}
                        {Self::display_stage("Memory access", &printer::to_assembly(&self.engine.ex_mem.mem.instruction))}
                        {Self::display_stage("Writeback", &printer::to_assembly(&self.engine.mem_wb.wb.instruction))}
                    </div>
                    <div class="datapath">
                        {Self::label_datapath()}
                        {Self::label_configs(&self.hazard, &self.forwarding)}
                        <canvas width="1160pt" height="600pt" ref={self.node_ref.clone()}/>
                    </div>
                    <div class = "pc-cycles">
                        <p class="pc_display">{"Program counter"}<br/>{self.engine.pc}</p>
                        <p class="cycles_display">{"Clock cycles"}<br/>{self.engine.cycles}</p>
                    </div>
                </div>
            </>
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();

        let gl: GL = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.gl = Some(gl);

        if first_render {
            let handle = {
                let link = ctx.link().clone();
                request_animation_frame(move |_time| link.send_message(Msg::Render))
            };
            self.render_loop = Some(handle);
        }
    }
}

impl Model {
    fn label_configs(hazard: &bool, forwarding: &bool) -> Html {
        let hazard_html: Html = html!{
            <>
            <div class="hazard">{"Hazard"}<br/>{"unit"}</div>
            </>
        };
        let forwarding_html: Html = html!{
            <>
            <div class="forwarding">{"Forwarding"}<br/>{"unit"}</div>
            <div class="forward_mux_rs1">{"M"}<br/>{"U"}<br/>{"X"}</div>
            <div class="forward_mux_rs2">{"M"}<br/>{"U"}<br/>{"X"}</div>
            </>
        };
        if *hazard && *forwarding {
            return html!{<>{hazard_html} {forwarding_html}</>};
        }
        else if *hazard {
            return hazard_html;
        } 
        else if *forwarding {
            return forwarding_html;
        } else {
            return html!{};
        }
    }
    
    fn label_datapath() -> Html {
        html!{
            <>
            <div class="ifid">{"IF/ID"}</div>
            <div class="idex">{"ID/EX"}</div>
            <div class="exmem">{"EX/MEM"}</div>
            <div class="memwb">{"MEM/WB"}</div>
            <div class="pc">{"PC"}</div>
            <div class="alu">{"ALU"}</div>
            <div class="mux-alu_rs1">{"M"}<br/>{"U"}<br/>{"X"}</div>
            <div class="mux-alu_rs2">{"M"}<br/>{"U"}<br/>{"X"}</div>
            <div class="mux-wb">{"M"}<br/>{"U"}<br/>{"X"}</div>
            <div class="reg">{"Registers"}</div>
            <div class="data-mem">{"Data"}<br/>{"memory"}</div>
            <div class="instr-mem">{"Instruction"}<br/>{"memory"}</div>
            <div class="imm-gen">{"Immediate"}<br/>{"generator"}</div>
            <div class="control">{"Control"}</div>
            <div class="mux_pc_imm">{"M"}<br/>{"U"}<br/>{"X"}</div>
            <div class="mux_rs1_imm">{"M"}<br/>{"U"}<br/>{"X"}</div>
            <div class="mux_pc">{"M"}<br/>{"U"}<br/>{"X"}</div>
            <div class="adder_imm_rs1">{"A"}<br/>{"D"}<br/>{"D"}</div>
            <div class="plus-4">{"+ 4"}</div>
            </>
        }
    }
    fn display_stage(stage: &str, instruction: &String) -> Html {
        html!{
            <p>{stage}<br/>{instruction}</p>
        }
    }
    fn display_instructions(file: &(String, Vec<u8>)) -> Html {
        let instructions = printer::instructions_as_assembly(&file.1);
        html!{
            <>
            <div class="instructions">
            <table border="1">
            <thead><tr>
            <th colspan="2" position="fixed">{"Instructions"}</th>
            </tr>
            <tr>
            <th width="30%">{"Address"}</th>
            <th width="70%">{"Instruction"}</th>
            </tr>
            </thead>
            <tbody>
            {
                instructions.into_iter().enumerate().map(|(address, instruction)| {
                    html!{
                        <tr>
                        <td width="30%">{format!("{:04x}", address*4)}</td>
                        <td width="70%">{instruction}</td>
                        </tr>
                    }
                }).collect::<Html>()
            }
            </tbody>
            </table>
        </div>
        </>
        }
    } 
    fn display_registers(registers: &[i32; 32], hex_dec: &bool) -> Html {
        html!{
            <>
            <div style="overflow-y:auto;grid-row:1/9; border-bottom: 1pt solid black">
            <table border = "1">
            <thead><tr>
            <th colspan="2" position="fixed">{"Registers"}</th>
            </tr>
            <tr>
            <th width="30%">{"Register"}</th>
            <th width="70%">{"Value"}</th>
            </tr>
            </thead>
            <tbody>
            {
                registers.into_iter().enumerate().map(|(count, register)| {
                    if *hex_dec {
                        html!{
                            <>
                            <tr>
                            <td width="32%">{format!("x{}", count)}</td>
                            <td width="68%">{format!("{}", register)}</td>
                            </tr>
                            </>
                        }
                    } else {
                        html!{
                            <>
                            <tr>
                            <td width="32%">{format!("x{}", count)}</td>
                            <td width="68%">{format!("{:x}", register)}</td>
                            </tr>
                            </>
                        }
                    }
                }).collect::<Html>()
            }
            </tbody>
            </table>
            </div>
            </>
        }
    }
    fn display_memory(memory: &[u8; 262144], hex_dec: &bool) -> Html {
        html!{
            <>
            <div style="overflow-y:auto;grid-row:1/9; border-bottom: 1pt solid black">
            <table class="table-border">
            <thead>
            <tr>
            <th colspan="5" position="fixed">{"Memory"}</th>
            </tr>
            <tr>
            <th width="32%">{"Address"}</th>
            <th width="17%">{"+0"}</th>
            <th width="17%">{"+1"}</th>
            <th width="17%">{"+2"}</th>
            <th width="17%">{"+3"}</th>
            </tr>
            </thead>
            <tbody>
            {
                memory[0..512].chunks(4).into_iter().enumerate().rev().map(|(address, mem2)| {
                    if *hex_dec {
                        html!{
                            <>
                            <tr>
                                <td width="32%">{format!("{:04x}", address*4)}</td>
                                <td width="17%">{format!("{}", mem2[0])}</td>
                                <td width="17%">{format!("{}", mem2[1])}</td>
                                <td width="17%">{format!("{}", mem2[2])}</td>
                                <td width="17%">{format!("{}", mem2[3])}</td>
                            </tr>
                            </>
                        }
                    } else {
                        html!{
                            <>
                            <tr>
                                <td width="32%">{format!("{:04x}", address*4)}</td>
                                <td width="17%">{format!("{:02x}", mem2[0])}</td>
                                <td width="17%">{format!("{:02x}", mem2[1])}</td>
                                <td width="17%">{format!("{:02x}", mem2[2])}</td>
                                <td width="17%">{format!("{:02x}", mem2[3])}</td>
                            </tr>
                            </>
                        }
                    }
                }).collect::<Html>()
            }
            </tbody>
            </table>
            </div>
            </>
        }
    }
    fn render_datapath(&mut self, link: &Scope<Self>) {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        let vert_code = "precision mediump float;
        attribute vec2 a_position;
        void main() {
            gl_Position = vec4(a_position, 0.0, 1.0);
        }";
        let frag_code = "precision mediump float;
        void main() {
            float r = 0.0;
            float g = 0.0;
            float b = 0.0;
            gl_FragColor = vec4(r, g, b, 1.0);
        }";

        // This list of vertices will draw two triangles to cover the entire canvas.

        let vertices: Vec<f32> = pipelines::simple_pipeline(self.hazard, self.forwarding);

        let vertex_buffer = gl.create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        // Attach the position vector as an attribute for the GL context.
        let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);
        
        gl.draw_arrays(GL::LINES, 0, vertices.len() as i32 / 2);

        let handle = {
            let link = link.clone();
            request_animation_frame(move |_time| link.send_message(Msg::Render))
        };

        // A reference to the new handle must be retained for the next render to run.
        self.render_loop = Some(handle);
    } 
}

fn main() {
    yew::start_app::<Model>();
}
