mod pages;
mod api;
mod types;
use pages::MainPage;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<MainPage>::new().render();
}
