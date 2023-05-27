use yew::prelude::*;
use components::{
    center_box::CenterBox,
    file_upload::FileUpload,
};

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="w-full flex items-center justify-center min-h-screen">
            <CenterBox>
                <h1>{ "Hello World!" }</h1>
                <FileUpload />
            </CenterBox>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
