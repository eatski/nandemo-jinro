use yew::{Callback, html, Html, MouseEvent, Properties, Children, function_component};

pub fn button(label: &str,onclick: Callback<MouseEvent>) -> Html {
    html! {
        <button onclick={onclick} class={"bg-feature hover:bg-feature-light text-white py-2 px-4 rounded-md"}>{label}</button>
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
        <section class="w-full max-w-2xl">
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



pub fn input_and_button(label: &'static str,placeholder: &'static str,onclick: Callback<String>) -> Html {
    html! {
        <div class="flex justify-center">
            <input class="border-line border-solid border rounded-md py-2 px-2 text-black mr-3" type="text" placeholder={placeholder}/>
            <button onclick={onclick.reform(|_| "aaa".to_owned())} class={"bg-feature hover:bg-feature-light text-white py-2 px-4 rounded-md"}>{label}</button>
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