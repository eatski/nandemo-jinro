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
                    <a class="rounded-md border-line border-solid border text-black hover:text-black-light py-2 px-3" href="/">
                        {"報告"}
                    </a>
                </div>
            </div>
        </header>
    }
}


pub fn title() -> Html {
    html! {
        <div class="text-center p-4">
            <h1 class="text-2xl lg:text-4xl text-black font-mono font-bold">{"なんでも人狼"}</h1>
            <p class="text-black-light mt-2">{"なんでも人狼ゲームにして遊べちゃうブラウザアプリ"}</p>
        </div>
    }
}

pub fn card_case() -> Html {
    html! {
        <section class="w-full p-4">
            <div class="h-80 rounded-md bg-colored p-3">
                <h2 class="w-full text-center text-2xl text-black">{"ルーム作成から"}</h2>
                <p class="w-full text-center text-sm text-black-light mt-1">{"オリジナルのルールで遊ぶ"}</p>
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}


#[function_component(CardListContainer)]
pub fn card_list_container(props:&Props) -> Html {
    html! {
        <div class="flex">
            {props.children.clone()}
        </div>
    }
}
