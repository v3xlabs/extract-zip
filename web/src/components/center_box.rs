use yew::{function_component, html, Children, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct CenterBoxProps {
    pub children: Children,
}

#[function_component(CenterBox)]
pub fn center_box(CenterBoxProps { children }: &CenterBoxProps) -> Html {
    html! {
        <div class="w-full max-w-xl px8 py-8 gap-4 border rounded-2xl mx-auto flex flex-col justify-center">{for children.iter()}</div>
    }
}
