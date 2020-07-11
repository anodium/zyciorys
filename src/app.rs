use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[path = "resume.rs"]
mod resume;

pub struct App {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }
    
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
    
    fn view(&self) -> Html {
        let r = resume::get_sample_resume();
        log(&r.name);

        html! {
            <>
                <p>{ r.name }</p>
                <p>{ if let Some(s) = r.midname {s} else {"".to_string()} }</p>
                <p>{ if let Some(s) = r.lastname {s} else {"".to_string()} }</p>
            </>
        }
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool { 
        todo!()
    }
}
