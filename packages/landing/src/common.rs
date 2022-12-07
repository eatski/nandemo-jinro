use atoms::{Button, InputText};
use yew::{function_component, html, use_state, Callback, Html, Properties};

pub fn title() -> Html {
    html! {
        <div class="text-center p-8">
            <h1 class="text-3xl text-word font-mono font-bold">{"なんでも人狼"}</h1>
            <p class="text-word-2nd mt-2 text-sm">{"ゲームマスター不要でなんでも人狼ゲームにして遊べちゃうブラウザ配役アプリ"}</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]

pub struct JoinFormProps {
    pub onsubmit: Callback<String>,
    pub label: &'static str,
    pub form_label: &'static str,
    pub default: String,
    pub placeholder: &'static str,
}

#[function_component(JoinForm)]
pub fn join_form(props: &JoinFormProps) -> Html {
    let state = use_state(|| props.default.clone());
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
    let value = (*state).clone();
    let disabled = value.is_empty();
    html! {
        <form aria-label={props.form_label} class="flex justify-center gap-2">
            <InputText value={value} aria_label="あなたの名前" placeholder="あなたの名前" oninput={oninput} maxlength={12} />
            <Button disabled={disabled} onclick={onsubmit}>{props.label}</Button>
        </form>
    }
}
