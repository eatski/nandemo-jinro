use yew::{html, function_component, Callback, Properties, Children};
use atoms::{ButtonRounded};

#[derive(Properties, PartialEq)]
pub struct RollButtonProps {
    pub onclick: Callback<()>,
    pub children: Children
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