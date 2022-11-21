use mylib::{simulator_engine::SimulatorEngine, printer, pipelines};

use web_sys::{Event, HtmlInputElement, HtmlCanvasElement, WebGlRenderingContext as GL};
use yew::{html, html::Scope, html::TargetCast, Component, Context, Html, NodeRef};
use wasm_bindgen::JsCast;

use gloo_file::callbacks::FileReader;
use gloo_file::File;

use gloo_render::{request_animation_frame, AnimationFrame};

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
                self.engine.run_engine(stepwise, true, true);
                true
            }
            Msg::Reset => {
                self.engine = Default::default();
                self.engine.read_bytes_to_mem(&self.file.1);
                true
            }
            Msg::Render => {
                self.render_datapath(ctx.link(), include_str!("./basic.frag"));
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
                    <button class={if self.hazard {"forward_active"} else {"forward_inactive"}} onclick={ctx.link().callback(|_| Msg::Hazard)}>{"Hazard detection"}</button>
                    <button class={if self.forwarding {"forward_active"} else {"forward_inactive"}} onclick={ctx.link().callback(|_| Msg::Forwarding)}>{"Data forwarding"}</button>
                </div>
            <div class = "registers">
            <p>{ format!("Register values:")}</p>
            {Self::display_registers(&self.engine.reg)}
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
        }
    }
    fn render_datapath(&mut self, link: &Scope<Self>, frag: &str) {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        let vert_code = include_str!("./basic.vert");
        let frag_code = frag;

        // This list of vertices will draw two triangles to cover the entire canvas.

        let vertices: Vec<f32> = pipelines::simple_pipeline();

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
