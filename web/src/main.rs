use yew::prelude::*;
use components::{
    center_box::{CenterBox},
};

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <CenterBox>
                <h1>{ "Hello World!" }</h1>
            </CenterBox>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
