use yew::{Callback, html, Html, MouseEvent};

pub fn button(label: &str,onclick: Callback<MouseEvent>) -> Html {
    html! {
        <button onclick={onclick} class={"bg-feature hover:bg-feature-light text-white py-2 px-4 rounded-md"}>{label}</button>
    }
}

pub fn loading() -> Html {
    html! {
        <div class="animate-spin h-10 w-10 border-4 border-quiet-500 rounded-full border-t-transparent"></div>
    }
}