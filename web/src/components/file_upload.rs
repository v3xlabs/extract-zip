use log::info;
use web_sys::{File, HtmlInputElement};
use yew::prelude::*;

#[function_component]
pub fn FileUpload() -> Html {
    let input_node_ref = use_node_ref();

    let input_value_handle: UseStateHandle<Option<File>> = use_state(|| None);
    let input_value = (*input_value_handle).clone();

    info!("Render");

    let onchange = {
        let input_node_ref = input_node_ref.clone();

        Callback::from(move |_| {
            let input = input_node_ref.cast::<HtmlInputElement>();

            let input = input.unwrap().files();

            if let Some(file_list) = input {
                // let value = input.value();
                // input_value_handle.set(input.value());
                info!("input_value: {:?}", file_list);

                let file = file_list.get(0).unwrap();

                info!("Filename: {}", file.name());
            } else {
                info!("No file selected")
            }
        })
    };

    // info!("input_value: {}", input_value);

    html! {
        <>
            <input type="file" ref={input_node_ref} onchange={onchange} />
            // <p>{"v: "} { input_value }</p>
        </>
    }
}
