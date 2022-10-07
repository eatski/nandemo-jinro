use yew::{html, function_component, Callback, Properties, Children};

#[derive(Properties, PartialEq)]
pub struct RollButtonProps {
    pub onclick: Callback<()>,
    pub children: Children
}

#[function_component(RollButton)]
pub fn roll_button(props: &RollButtonProps) -> Html {
    html! {
        <div class="m-auto flex justify-center">
            <button class={"animate-bounce bg-feature hover:bg-feature-light text-white py-3 px-3 text-lg rounded-full h-16 h-16"} onclick={props.onclick.reform(|_| ())}>
                {props.children.clone()}
            </button>
        </div>
    }
}