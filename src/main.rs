#[macro_use]
extern crate lazy_static;

mod util;

use yew::prelude::*;
use wasm_bindgen::{JsCast};
use web_sys::HtmlTextAreaElement;
use web_sys::{HtmlInputElement, InputEvent};

use util::demangle;

#[function_component(App)]
fn app() -> Html {
    let output_ref = use_node_ref();

    let on_input = {
        let output_ref = output_ref.clone();
        move |e: InputEvent| {
            if let Some(output) = output_ref.cast::<HtmlTextAreaElement>() {
                let input = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
                output.set_value(&demangle(&input));
            }
        }
    };

    html! {
        <div class="content">
            <div class="textboxes">
                <div class="border">
                    <textarea placeholder="Input" oninput={on_input}></textarea>
                </div>
                <div class="border">
                    <textarea ref={output_ref} readonly=true placeholder="Output"></textarea>
                </div>
            </div>
        </div>
    }
}


fn main() {
    yew::start_app::<App>();
}