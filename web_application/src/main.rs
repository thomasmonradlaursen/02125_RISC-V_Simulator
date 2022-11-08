use mylib::{simulator_engine::SimulatorEngine, printer};

use web_sys::{Event, HtmlInputElement};
use yew::{html, html::TargetCast, Component, Context, Html};

use gloo_file::callbacks::FileReader;
use gloo_file::File;

pub enum Msg {
    LoadedBytes(String, Vec<u8>),
    Files(File),
    RunSimulator(bool),
}

pub struct Model {
    reader: Option<FileReader>,
    file: (String, Vec<u8>),
    engine: SimulatorEngine,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            reader: Default::default(),
            file: (String::from("Empty"), vec![]),
            engine: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadedBytes(file_name, data) => {
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
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div class = "container">
                <div class = "controls">
                    <input type = "file" onchange={ctx.link().callback(move |e: Event| {
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
                </div>
            <div class = "registers">
            <p>{ format!("Register values:")}</p>
            {Self::display_registers(&self.engine.reg)}
            </div>
            {Self::display_instructions(&self.file)}
            <div class="stages">
            {Self::display_stage("Decode", &printer::to_assembly(&self.engine.if_id.decode.instruction))}
            {Self::display_stage("Execute", &printer::to_assembly(&self.engine.id_ex.execute.instruction))}
            {Self::display_stage("Memory access", &printer::to_assembly(&self.engine.ex_mem.mem.instruction))}
            {Self::display_stage("Writeback", &printer::to_assembly(&self.engine.mem_wb.wb.instruction))}
            </div>
            <div class="datapath">
            <p>{"Datapath"}</p>
            </div>
            </div>
            </>
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
                instructions.into_iter().map(|instruction| {
                    html!{
                        <tr>
                            <td>{instruction}</td>
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
                registers.into_iter().map(|register| {
                    html!{
                        <tr>
                            <td>{register}</td>
                        </tr>
                    }
                }).collect::<Html>()
            }
            </table>
        }
    } 
}

fn main() {
    yew::start_app::<Model>();
}
