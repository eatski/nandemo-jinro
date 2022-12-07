use atoms::{button_link, ButtonSub};
use yew::{function_component, html, Children, Properties, Html, Callback, use_state};

#[derive(Properties, PartialEq)]
pub struct ChildrenOnlyProps {
    pub children: Children, // the field name `children` is important!
}

#[function_component(Layout)]
pub fn layout(props: &ChildrenOnlyProps) -> Html {
    let dark_mode = use_state(|| false);
    html! {
        <div class="bg-screen min-h-screen h-max" data-theme={if *dark_mode {"dark"} else { "light"}}>
            <header class="w-full border-separator border-solid border-b px-4 py-3 ">
                <div class="flex items-center">
                    <a class="font-mono text-word text-lg" href="/">
                        {"なんでも人狼"}
                    </a>
                    <div class="ml-auto">
                        <ButtonSub onclick={Callback::from(move |_| dark_mode.set(!*dark_mode))}>
                            {"ダークモード"}
                        </ButtonSub>
                        {button_link("報告","https://github.com/eatski/roleroll/issues/new")}
                    </div>

                </div>
            </header>
            <main class="px-7 py-12">
                {props.children.clone()}
            </main>
        </div>
    }
}
