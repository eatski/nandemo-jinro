use yew::{Html, html, Callback, function_component, Properties, use_state};
use atoms::{InputText,Button};

pub fn title() -> Html {
    html! {
        <div class="text-center p-8">
            <h1 class="text-2xl md:text-4xl text-black font-mono font-bold">{"なんでも人狼"}</h1>
            <p class="text-black-light mt-2">{"なんでも人狼ゲームにして遊べちゃうブラウザアプリ"}</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]

pub struct JoinFormProps {
    pub onsubmit: Callback<String>,
    pub label: &'static str,
    pub default: String,
    pub placeholder: &'static str,
}

#[function_component(JoinForm)]
pub fn join_form(props: &JoinFormProps) -> Html {
    let state = use_state(|| "".to_string());
    let oninput = {
        let state = state.clone();
        Callback::from(move |e: String| {
            state.set(e);
        })
    };
    let onsubmit = {
        let state = state.clone();
        props.onsubmit.reform(move |_| (*state).clone())
    };
    html! {
        <form class="flex justify-center">
            <InputText value={(*state).clone()} placeholder="あなたの名前" oninput={oninput} />
            <Button onclick={onsubmit}>{props.label}</Button>
        </form>
    }
}