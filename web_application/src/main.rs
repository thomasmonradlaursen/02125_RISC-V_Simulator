use std::collections::HashMap;

use web_sys::{Event, HtmlInputElement};
use yew::{html, html::TargetCast, Component, Context, Html};

use gloo_file::callbacks::FileReader;
use gloo_file::File;

pub enum Msg {
    LoadedBytes(String, Vec<u8>),
    Files(Vec<File>),
}

pub struct Model {
    readers: HashMap<String, FileReader>,
    files: Vec<String>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LoadedBytes(file_name, data) => {
                let info = format!("file_name: {}, length: {}, data: {:?}", file_name, data.len(), data);
                self.files.push(info);
                self.readers.remove(&file_name);
                true
            }
            Msg::Files(file) => {
                    let file_name = file[0].name();
                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();

                        gloo_file::callbacks::read_as_bytes(&file[0], move |res| {
                            link.send_message(Msg::LoadedBytes(
                                file_name,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                    true
                }
            }
        }
        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <div>
                        <p>{ "Choose file for simulator" }</p>
                        <input type="file" multiple=true onchange={ctx.link().callback(move |e: Event| {
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
                                Msg::Files(result)
                            })}
                        />
                    </div>
                    //<p> { for self.files.iter().map(|f| Self::view_file(f)) } </p>
                    <p> { Self::view_some(self.files.get(0))} </p>
                </div>
            }
        }
    }


impl Model {
    fn view_file(data: &str) -> Html {
        html! {
            <p>{ data }</p>
        }
    }
    fn view_some(data: Option<&String>) -> Html {
        match data {
            Some(index) => html! {<p>{index}</p> },
            None => html! {<p>{"No index"}</p> },
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
