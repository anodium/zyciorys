use yew::prelude::*;
use yaml_rust::YamlLoader;

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

        html! {
            <>
                <p>{ r.name }</p>
            </>
        }
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool { 
        todo!()
    }
}
