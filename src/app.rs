use yew::prelude::*;
use yaml_rust::{YamlLoader, YamlEmitter};

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
        let s: &str = 
r"foo:
    - list1
    - list2
bar:
    - 1
    - 2.0
";
        let y = YamlLoader::load_from_str(s).unwrap();

        html! {
            <p>{ &y[0]["foo"][0].as_str().unwrap() }</p>
        }
    }

    fn change(&mut self, _: <Self as yew::html::Component>::Properties) -> bool { 
        todo!()
    }
}
