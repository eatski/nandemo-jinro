use yew::{Callback, html, Html, MouseEvent, Properties, Children, function_component, use_state,TargetCast};
use web_sys::{HtmlInputElement, InputEvent};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
    pub disabled: Option<bool>,
}


const BUTTON_FEATURE_COLOR: &str = "transition-colors bg-feature hover:bg-feature-light text-white";

#[function_component[Button]]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button onclick={props.onclick.clone()} disabled={props.disabled.unwrap_or_else(|| false)} class={format!("{} py-2 px-4 rounded-md",BUTTON_FEATURE_COLOR)}>{props.children.clone()}</button>
    }
}

#[function_component[ButtonLarge]]
pub fn button_large(props: &ButtonProps) -> Html {
    html! {
        <button onclick={props.onclick.clone()} class={format!("{} py-4 px-9 text-lg rounded-full",BUTTON_FEATURE_COLOR)}>{props.children.clone()}</button>
    }
}

#[function_component(ButtonRounded)]
pub fn button_rounded(props: &ButtonProps) -> Html {
    html! {
        <button class={format!("{} py-3 px-3 text-lg rounded-full h-16 h-16",BUTTON_FEATURE_COLOR)} onclick={props.onclick.clone()}>
            {props.children.clone()}
        </button>
    }
}

pub fn loading() -> Html {
    html!  {
        <div class="animate-spin h-10 w-10 border-4 border-quiet-500 rounded-full border-t-transparent"></div>
    }
}

pub fn button_link(label: &str,href: &str) -> Html {
    html! {
        <a class="rounded-md border-line border-solid border text-black bg-white hover:text-black-light py-2 px-3" href={href.to_owned()}>
            {label}
        </a>
    }
}

#[function_component(Heading2)]
pub fn heading2(props: &ChildrenOnlyProps) -> Html {
    html! {
        <h2 class="w-full text-center text-2xl text-black mb-1">{props.children.clone()}</h2>
    }
}

#[function_component(HeadingDescription)]
pub fn heading_descriotion(props: &ChildrenOnlyProps) -> Html {
    html! {
        <p class="w-full text-center text-sm text-black-light">{props.children.clone()}</p>
    }
}

#[derive(Properties, PartialEq)]
pub struct ChildrenOnlyProps {
    pub children: Children, // the field name `children` is important!
}

#[derive(Properties, PartialEq)]
pub struct InputAndButtonProps {
    pub label:  &'static str,
    pub placeholder: &'static str,
    pub onsubmit: Callback<String>,
    pub default: Option<&'static str>
}

#[derive(Properties, PartialEq)]
pub struct InputTextProps {
    pub placeholder: &'static str,
    pub oninput: Callback<String>,
    pub value: String,
    pub maxlength: usize,
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
            class="w-52 border-line border-solid border focus:border-feature rounded-md py-2 px-2 text-black outline-none" type="text" placeholder={props.placeholder}
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
        <input {oninput} min="0" value={props.value.to_string()} class="w-14 border-line border-solid border focus:border-feature rounded-md py-2 px-2 text-black outline-none" type="number" />
    }
}
