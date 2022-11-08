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
                <div class = "item-a">
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
                </div>
            <div class = "item-b">
            <button onclick={ctx.link().callback(|_| Msg::RunSimulator(false))}>{ "Full execution" }</button>
            <button onclick={ctx.link().callback(|_| Msg::RunSimulator(true))}>{ "Stepwise" }</button>
            <p>{ format!("Name of binary: {}", self.file.0) }</p>
            <p>{ format!("Length of program: {}", self.engine.program_len) }</p>
            </div>
            <div class = "registers">
            <p>{ format!("Register values:")}</p>
            {Self::display_registers(&self.engine.reg)}
            </div>
            {Self::display_decode(&printer::to_assembly(&self.engine.if_id.decode.instruction))}
            {Self::display_execute(&printer::to_assembly(&self.engine.id_ex.execute.instruction))}
            {Self::display_mem(&printer::to_assembly(&self.engine.ex_mem.mem.instruction))}
            {Self::display_writeback(&printer::to_assembly(&self.engine.mem_wb.wb.instruction))}
            {Self::display_instructions(&self.file)}
            </div>
            </>
        }
    }
}

impl Model {
    fn display_decode(instruction: &String) -> Html {
        html!{
            <div class = "decode">
                <p>{ format!("Decode: {}", instruction) }</p>
            </div>
        }
    }
    fn display_execute(instruction: &String) -> Html {
        html!{
            <div class = "execute">
                <p>{ format!("Execute: {}", instruction) }</p>
            </div>
        }
    }
    fn display_mem(instruction: &String) -> Html {
        html!{
            <div class = "memory">
                <p>{ format!("Memory access: {}", instruction) }</p>
            </div>
        }
    }
    fn display_writeback(instruction: &String) -> Html {
        html!{
            <div class = "writeback">
                <p>{ format!("Writeback: {}", instruction) }</p>
            </div>
        }
    }
    fn display_instructions(file: &(String, Vec<u8>)) -> Html {
        let instructions = printer::instructions_as_assembly(&file.1);
        html!{
        <div class = "instructions">
            <table>
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
            <table>
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
