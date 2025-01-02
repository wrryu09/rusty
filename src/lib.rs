use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct RustProj;

impl Component for RustProj {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        true
    }
    

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! { <span>{"Hi......"}</span> }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<RustProj>::new().render();
}
