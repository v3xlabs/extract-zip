use components::{center_box::CenterBox, file_upload::FileUpload};
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="w-full text-center flex items-center justify-center min-h-screen px-4">
            <CenterBox>
                <h1 class="text-2xl font-bold text-center pb-4">{ "extract.zip" }</h1>
                <FileUpload />
                <div>
                    {"This website is still work in progress"}
                </div>
                <div>
                    {"Built with 💖 in "}<a href="https://github.com/v3xlabs/extract-zip" class="text-pink-500 hover:underline">{"Rust"}</a>{" by "}<a href="https://github.com/v3xlabs/extract-zip" class="text-cyan-500 hover:underline">{"V3X Labs"}</a>
                </div>
            </CenterBox>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
