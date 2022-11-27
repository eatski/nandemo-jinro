use atoms::ButtonRounded;
use yew::{function_component, html, Callback, Children, Properties, Html};

#[derive(Properties, PartialEq)]
pub struct RollButtonProps {
    pub onclick: Callback<()>,
    pub children: Children,
}

#[function_component(RollButton)]
pub fn roll_button(props: &RollButtonProps) -> Html {
    html! {
        <div class="animate-bounce">
            <ButtonRounded aria_label="役職を配布する" onclick={props.onclick.reform(|_| ())}>
                {props.children.clone()}
            </ButtonRounded>
        </div>

    }
}
