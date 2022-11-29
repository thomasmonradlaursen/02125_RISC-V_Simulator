use simulator_engine::SimulatorEngine;
use graphics::{components, pipelines};
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
pub mod simulator_engine;

pub enum Msg {
    LoadedBytes(String, Vec<u8>),
    Files(File),
    RunSimulator(bool),
    Hazard,
    Forwarding,
    Reset,
    Render,
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
                self.render_datapath(ctx.link(), include_str!("graphics/basic.frag"));
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
            {Self::display_registers(&self.engine.reg)}
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
        if *hazard && *forwarding {
            return html!{
                <>
                <div class="hazard">{"Hazard"}<br/>{"unit"}</div>
                <div class="forwarding">{"Forwarding"}<br/>{"unit"}</div>
                <div class="forward_mux_rs1">{"M"}<br/>{"U"}<br/>{"X"}</div>
                <div class="forward_mux_rs2">{"M"}<br/>{"U"}<br/>{"X"}</div>
                </>
            }
        }
        if *hazard {
            return html!{
                <>
                <div class="hazard">{"Hazard"}<br/>{"unit"}</div>
                <div class="hazard_replace">{"Hazard"}<br/>{"unit"}</div>
                </>
            }
        }
        if *forwarding {
            return html!{
                <>
                <div class="forwarding">{"Forwarding"}<br/>{"unit"}</div>
                <div class="forward_mux_rs1">{"M"}<br/>{"U"}<br/>{"X"}</div>
                <div class="forward_mux_rs2">{"M"}<br/>{"U"}<br/>{"X"}</div>
                </>
            }
        }
        html!{}
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
        <div class = "instructions">
            <p>{String::from("Instructions")}</p>
            <table border="1">
            {
                instructions.into_iter().enumerate().map(|(address, instruction)| {
                    html!{
                        <tr>
                            <td width="20%">{format!("{:04x}", address*4)}</td>
                            <td width="80%">{instruction}</td>
                        </tr>
                    }
                }).collect::<Html>()
            }
            </table>
        </div>
        }
    } 
    fn display_registers(registers: &[i32; 32]) -> Html {
        html!{
            <div class = "registers">
            <p>{ format!("Register values:")}</p>
            <table border = "1">
            {
                registers.into_iter().enumerate().map(|(count, register)| {
                    html!{
                        <>
                        <tr>
                            <td width="20%">{format!("x{}", count)}</td>
                            <td width="80%">{register}</td>
                        </tr>
                        </>
                    }
                }).collect::<Html>()
            }
            </table>
            </div>
        }
    }
    fn render_datapath(&mut self, link: &Scope<Self>, frag: &str) {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        let vert_code = include_str!("graphics/basic.vert");
        let frag_code = frag;

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
