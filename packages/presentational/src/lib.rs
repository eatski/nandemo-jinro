use yew::{Callback, html, Html, MouseEvent, Properties, Children, function_component, use_state,TargetCast};
use web_sys::{HtmlInputElement, InputEvent};

pub fn button(label: &str,onclick: Callback<MouseEvent>) -> Html {
    html! {
        <button onclick={onclick} class={"bg-feature hover:bg-feature-light text-white py-1 px-4 rounded-md"}>{label}</button>
    }
}

pub fn loading() -> Html {
    html!  {
        <div class="animate-spin h-10 w-10 border-4 border-quiet-500 rounded-full border-t-transparent"></div>
    }
}

pub fn header() -> Html {
    html! {
        <header class="w-full border-line border-solid border-b px-4 py-3 ">
            <div class="flex items-center">
                <a class="font-mono text-black hover:text-black-light text-md" href="/">
                    {"なんでも人狼"}
                </a>
                <div class="ml-auto">
                   {button_link("報告","/")}
                </div>
            </div>
        </header>
    }
}

pub fn button_link(label: &str,href: &str) -> Html {
    html! {
        <a class="rounded-md border-line border-solid border text-black bg-white hover:text-black-light py-2 px-3" href={href.to_owned()}>
            {label}
        </a>
    }
}


pub fn title() -> Html {
    html! {
        <div class="text-center p-8">
            <h1 class="text-2xl md:text-4xl text-black font-mono font-bold">{"なんでも人狼"}</h1>
            <p class="text-black-light mt-2">{"なんでも人狼ゲームにして遊べちゃうブラウザアプリ"}</p>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct Heading2WithDescriptionProps {
    pub description: String,
    pub title: String,
}

#[function_component(Heading2)]
pub fn heading2(props: &ChildrenOnlyProps) -> Html {
    html! {
        <h2 class="w-full text-center text-2xl text-black">{props.children.clone()}</h2>
    }
}

#[function_component(Heading2WithDescription)]
pub fn heading2_with_description(props: &Heading2WithDescriptionProps) -> Html {
    html! {
        <>
            <h2 class="w-full text-center text-2xl text-black">{&props.title}</h2>
            <p class="w-full text-center text-sm text-black-light mt-1">{&props.description}</p>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub children: Children,
    pub bg_type: CardBgType,
}

#[derive(PartialEq)]
pub enum CardBgType {
    White,
    Colored
}

impl CardBgType {
    fn to_class(&self) -> &str {
        match self {
            CardBgType::White => "bg-white",
            CardBgType::Colored => "bg-colored"
        }
    }
}

#[function_component(Card)]
pub fn card(props:&CardProps) -> Html {
    html! {
        <section class="w-full max-w-xl">
            <div class={format!("h-full rounded-md p-3 {}", props.bg_type.to_class())}>
                {props.children.clone()}
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct ChildrenOnlyProps {
    pub children: Children, // the field name `children` is important!
}

#[function_component(CardListContainer)]
pub fn card_list_container(props:&ChildrenOnlyProps) -> Html {
    html! {
        <div class="flex gap-7 flex-col md:flex-row justify-center">
            {props.children.clone()}
        </div>
    }
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
            <input onclick={on_input_click} value={value.clone()} {oninput} class="w-3/5 border-line border-solid border focus:border-feature rounded-md py-2 px-2 text-black mr-3 outline-none" type="text" placeholder={props.placeholder}/>
            <button onclick={props.onsubmit.reform(move |_| value.clone())} {disabled} class={"bg-feature transition-colors hover:bg-feature-light disabled:bg-quiet text-white py-2 px-4 rounded-md"}>{&props.label}</button>
        </div>
    }
}

#[function_component(CardContent)]
pub fn card_content(props:&ChildrenOnlyProps) -> Html {
    html! {
        <div class="py-2">
            {props.children.clone()}
        </div>
    }
}

pub fn tag_list() -> Html {
    html! {
        <div class="flex flex-wrap justify-center gap-2">
            {button_link("スプラトゥーン", "/tags/spatoon")}
            {button_link("汎用 4人", "/tags/monster_hunter")}
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

pub fn list(title: &'static str,items: Vec<&'static str>) -> Html {
    html! {
        <>
            <h2 class="w-full text-center text-2xl text-black mb-2">{title}</h2>
            <div  class="w-full flex justify-center">
                <ol class="text-black-light text-sm list-decimal space-y-1">
                    {for items.iter().map(|item| html! {<li class="">{item}</li>})}
                </ol>
            </div>
            
        </>
    }
}

pub fn point(text: &str) -> Html {
    html! {
        <span role="img" aria-label={text.to_owned()} class="relative h-2 w-2">
            <span class="animate-pulse absolute -translate-y-1/2 top-0 left-0 rounded-full bg-feature-light h-2 w-2" />
        </span>
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

pub fn item_box(label: &str,subtext: Option<&str>) -> Html {
    html! {
        <div class="flex justify-center">
            <div class="relative bg-colored-light w-4/5 border-line border-solid border rounded-full py-0.5 text-center text-md text-black-light">
                {label}<span class="absolute top-1/2 right-5">{subtext.map(point).unwrap_or(html!{})}</span>
            </div>
        </div>
    }
}

#[function_component(BoxListContainer)]
pub fn box_list_conatiner(props: &ChildrenOnlyProps) -> Html {
    html! {
        <ul class="flex flex-col gap-2">
            {props.children.clone()}
        </ul>
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
        <input oninput={oninput} value={props.value.clone()} class="w-3/5 border-line border-solid border focus:border-feature rounded-md py-2 px-2 text-black outline-none" type="text" placeholder={props.placeholder}/>
    }
}


#[derive(Properties, PartialEq)]
pub struct InputNumberProps {
    pub oninput: Callback<u32>,
    pub value: u32,
}
#[function_component(InputSmallNumber)]
pub fn input_small_number(props: &InputNumberProps) -> Html {
    let oninput = {
        let oninput = props.oninput.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input {
                if let Ok(value) = input.value().parse::<u32>() {
                    oninput.emit(value);
                }
            }
        })
    };
    html! {
        <input {oninput} min="0" value={props.value.to_string()} class="w-14 border-line border-solid border focus:border-feature rounded-md py-2 px-2 text-black outline-none" type="number" />
    }
}

#[derive(Properties, PartialEq)]
pub struct AddProps {
    pub onclick: Callback<()>
}

#[function_component(AddButton)]
pub fn add_button(props: &AddProps) -> Html {
    html! {
        <div class="h-6 w-6 relative">
            <button onclick={props.onclick.reform(|_| ())} class="
                absolute -translate-y-1/2 -translate-x-1/2 top-1/2 left-1/2
                w-5 h-5 rounded-full
                bg-feature transition-colors hover:bg-feature-light
            " 
            >
                <span role="img" aria-label="追加" style="top:45%;" class="absolute -translate-y-1/2 -translate-x-1/2 left-1/2 text-white">{"+"}</span>
            </button>
        </div>
        
    }
}



#[function_component(ListItemRow)]
pub fn list_item_row(props: &ChildrenOnlyProps) -> Html {
    html! {
        <li class="flex justify-center gap-2 w-full">
            {props.children.clone()}
        </li>
    }
}

#[function_component(ListContainer)]
pub fn lsit_container(props: &ChildrenOnlyProps) -> Html {
    html! {
        <ul class="flex flex-col gap-2">
            {props.children.clone()}
        </ul>
    }
}