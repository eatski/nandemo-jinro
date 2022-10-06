use atoms::loading;
use yew::{function_component, html, Callback, Properties};

use crate::use_roll::use_roll;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
}

#[function_component(RollButton)]
pub fn roll(props: &Props) -> Html {
    let roll = use_roll(props.room_id.as_str());
    match roll {
        Some(roll) => {
            let on_click = Callback::from(move |_| roll());
            html! {
                <button onclick={on_click}>{"Roll"}</button>
            }
        },
        None => loading(),
    }
}