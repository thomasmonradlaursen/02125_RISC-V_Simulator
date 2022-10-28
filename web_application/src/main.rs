use mylib::{simulator_engine, simulator_engine::SimulatorEngine};

use web_sys::{Event, HtmlInputElement};
use yew::{html, html::TargetCast, Component, Context, Html};

use gloo_file::callbacks::FileReader;
use gloo_file::File;

pub enum Msg {
    LoadedBytes(String, Vec<u8>),
    Files(File),
    RunSimulator,
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
            file: (String::new(), vec![]),
            engine: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadedBytes(file_name, data) => {
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
            Msg::RunSimulator => {
                self.engine.run_engine(&self.file.1.len(), false, true, true);
                true
            }
            /*Msg::RunSimulator(simulator_engine, file) => {
                true
            }*/
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <div>
                <div>
                    <p>{ "Choose file for simulator" }</p>
                    <input type="file" onchange={ctx.link().callback(move |e: Event| {
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
                <p> { Self::view_some(&self.file.0)} </p>
            </div>
            <div>
            <p>{ "Run simulator" }</p>
            <button onclick={ctx.link().callback(|_| Msg::RunSimulator)}>{ "Run simulation" }</button>
            <p>{ format!("{} {} {:?}", self.file.0, self.engine.cycles, self.engine.reg) }</p>
            </div>
            </>
        }
    }
}

impl Model {
    fn view_some(data: &String) -> Html {
        html! {<p>{data}</p> }
    }
}

fn main() {
    yew::start_app::<Model>();
}
