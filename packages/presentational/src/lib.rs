use yew::{Callback, html, Html, MouseEvent, Properties, Children, function_component, use_state,TargetCast};
use web_sys::{HtmlInputElement, InputEvent};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

pub fn button(label: &str,onclick: Callback<MouseEvent>) -> Html {
    html! {
        <button onclick={onclick} class={"bg-feature hover:bg-feature-light text-white py-1 px-4 rounded-md"}>{label}</button>
    }
}

#[function_component[ButtonLarge]]
pub fn button_large(props: &ButtonProps) -> Html {
    html! {
        <button onclick={props.onclick.clone()} class={"bg-feature hover:bg-feature-light text-white py-4 px-9 text-lg rounded-full"}>{props.children.clone()}</button>
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

#[function_component(InputAndButton)]
pub fn input_and_button(props: &InputAndButtonProps) -> Html {
    let state = use_state(|| "".to_string());
    let oninput = {
        let state = state.clone();
         Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input {
                state.set(input.value());
            }
        })
    };
    let input_clicked = use_state(|| false);
    let on_input_click = {
        let input_clicked = input_clicked.clone();
        Callback::from(move |_| {
            input_clicked.set(true);
        })
    };
    let value = {
        let state = state.clone();
        let value = match (props.default, &*input_clicked) {
            (Some(default), false) => default,
            _ => &*state
        };
        value.to_string()
    };
    let disabled = value.is_empty();
    html! {
        <div class="flex justify-center">
            <input onclick={on_input_click} value={value.clone()} {oninput} class="w-48 border-line border-solid border focus:border-feature rounded-md py-2 px-2 text-black mr-3 outline-none" type="text" placeholder={props.placeholder}/>
            <button onclick={props.onsubmit.reform(move |_| value.clone())} {disabled} class={"bg-feature transition-colors hover:bg-feature-light disabled:bg-quiet text-white py-2 px-4 rounded-md"}>{&props.label}</button>
        </div>
    }
}

#[function_component(Main)]
pub fn main(props: &ChildrenOnlyProps) -> Html {
    html! {
        <main class="px-7 py-8">
            {props.children.clone()}
        </main>
    }
}

pub fn footer() -> Html {
    html! {
        <footer class="w-full border-line border-solid border-t px-4 py-3">
            
        </footer>
    }
}

#[function_component(SimpleCenteringSection)]
pub fn simple_centering_section(props:&ChildrenOnlyProps) -> Html {
    html! {
        <section class="mx-auto flex justify-center w-full max-w-2xl py-2">
            <div>
                {props.children.clone()}
            </div>
        </section>
    }
}

#[function_component(SimpleCenteringDiv)]
pub fn simple_centering_div(props: &ChildrenOnlyProps) -> Html {
    html! {
        <div class="mx-auto flex justify-center w-full max-w-2xl py-2">
            <div>
                {props.children.clone()}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct InputTextProps {
    pub placeholder: &'static str,
    pub oninput: Callback<String>,
    pub value: String
}

#[function_component(InputText)]
pub fn input_text(props: &InputTextProps) -> Html {
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
        <input oninput={oninput} value={props.value.clone()} class="w-64 border-line border-solid border focus:border-feature rounded-md py-2 px-2 text-black outline-none" type="text" placeholder={props.placeholder}/>
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
