use web_sys::{HtmlInputElement, InputEvent};
use yew::{
    function_component, html, use_state, Callback, Children, Html, MouseEvent, Properties,
    TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
    pub label: Option<&'static str>,
    pub disabled: Option<bool>,
}

const BUTTON_FEATURE_COLOR: &str =
    "transition-colors bg-action hover:bg-action-hover disabled:bg-action-disable text-action-label disabled:text-action-label-disable";

#[function_component[Button]]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button aria-label={props.label} title={props.label} onclick={props.onclick.clone()} disabled={props.disabled.unwrap_or_else(|| false)} class={format!("{} py-2 px-4 rounded-md",BUTTON_FEATURE_COLOR)}>
            {props.children.clone()}
        </button>
    }
}



#[function_component[ButtonLarge]]
pub fn button_large(props: &ButtonProps) -> Html {
    html! {
        <button aria-label={props.label} title={props.label} onclick={props.onclick.clone()} disabled={props.disabled.unwrap_or_else(|| false)} class={format!("{} py-4 px-9 text-lg rounded-full",BUTTON_FEATURE_COLOR)}>
            {props.children.clone()}
        </button>
    }
}

#[function_component(ButtonRounded)]
pub fn button_rounded(props: &ButtonProps) -> Html {
    html! {
        <button aria-label={props.label} title={props.label} class={format!("{} py-3 px-3 text-lg rounded-full h-16 h-16",BUTTON_FEATURE_COLOR)}  disabled={props.disabled.unwrap_or_else(|| false)} onclick={props.onclick.clone()}>
            {props.children.clone()}
        </button>
    }
}



pub fn loading() -> Html {
    html! {
        <div role="img" aria-label="ローディング" class="animate-spin h-10 w-10 border-4 border-action-disable-500 rounded-full border-t-transparent"></div>
    }
}

const BUTTON_SUB_COLOR: &str = "transition-colors rounded-md border-separator text-word bg-screen hover:text-word-2nd py-2 px-3";

pub fn button_link(label: &str, href: &str) -> Html {
    html! {
        <a class={BUTTON_SUB_COLOR} href={href.to_owned()}>
            {label}
        </a>
    }
}

#[function_component]
pub fn ButtonSub(props: &ButtonProps) -> Html {
    html! {
        <button aria-label={props.label} title={props.label} onclick={props.onclick.clone()} disabled={props.disabled.unwrap_or_else(|| false)} class={format!("{} py-2 px-4",BUTTON_SUB_COLOR)}>
            {props.children.clone()}
        </button>
    }
}

#[function_component(Heading2)]
pub fn heading2(props: &ChildrenOnlyProps) -> Html {
    html! {
        <h2 class="w-full text-center text-xl text-word mb-1">{props.children.clone()}</h2>
    }
}

#[function_component(HeadingDescription)]
pub fn heading_descriotion(props: &ChildrenOnlyProps) -> Html {
    html! {
        <p class="text-center text-sm text-word-2nd">{props.children.clone()}</p>
    }
}

#[derive(Properties, PartialEq)]
pub struct ChildrenOnlyProps {
    pub children: Children, // the field name `children` is important!
}

#[derive(Properties, PartialEq)]
pub struct InputAndButtonProps {
    pub label: &'static str,
    pub placeholder: &'static str,
    pub onsubmit: Callback<String>,
    pub default: Option<&'static str>,
}

#[derive(Properties, PartialEq)]
pub struct InputTextProps {
    pub placeholder: &'static str,
    pub oninput: Callback<String>,
    pub value: String,
    pub maxlength: usize,
    pub label: Option<&'static str>,
}

#[function_component(InputText)]
pub fn input_text(props: &InputTextProps) -> Html {
    let input_clicked = use_state(|| false);
    let on_input_click = (!*input_clicked).then(|| {
        let on_input = props.oninput.clone();
        let input_clicked = input_clicked.clone();
        Callback::from(move |_| {
            on_input.emit("".to_string());
            input_clicked.set(true);
        })
    });
    let oninput = {
        let oninput = props.oninput.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input {
                oninput.emit(input.value());
            }
        })
    };
    html! {
        <input
            oninput={oninput}
            value={props.value.clone()}
            onclick={on_input_click}
            maxlength={props.maxlength.to_string()}
            aria-label={props.label}
            class="w-52 border-separator border-solid border focus:border-action rounded-md py-2 px-2 transition-colors bg-screen-2nd text-word outline-none" type="text" placeholder={props.placeholder}
        />
    }
}

#[derive(Properties, PartialEq)]
pub struct InputNumberProps {
    pub oninput: Callback<usize>,
    pub value: usize,
}
#[function_component(InputSmallNumber)]
pub fn input_small_number(props: &InputNumberProps) -> Html {
    let oninput = {
        let oninput = props.oninput.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input {
                if let Ok(value) = input.value().parse::<usize>() {
                    oninput.emit(value);
                }
            }
        })
    };

    html! {
        <input {oninput} min="1" value={props.value.to_string()} class="w-14 border-separator border-solid border focus:border-action rounded-md py-2 px-2 bg-screen-2nd text-word outline-none" type="number" />
    }
}

pub fn unexpected_error() -> Html {
    html! {
        <p>{"予期せぬエラーが発生しました。"}</p>
    }
}