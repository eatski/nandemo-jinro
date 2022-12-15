use atoms::{button_link, ButtonSub};
use yew::{function_component, html, Children, Properties, Html};

use use_stored_input::use_stored_string;

#[derive(Properties, PartialEq)]
pub struct ChildrenOnlyProps {
    pub children: Children, // the field name `children` is important!
}

#[function_component(Layout)]
pub fn layout(props: &ChildrenOnlyProps) -> Html {
    let (theme,set_theme) = use_stored_string("theme");
    let dark_mode = theme == Some("dark".to_owned());
    html! {
        <div class="transition-colors bg-screen min-h-screen h-max" data-theme={if dark_mode {"dark"} else { "light"}}>
            <header class="w-full border-separator border-solid border-b px-4 py-3 ">
                <div class="flex items-center">
                    <a class="font-mono text-word text-lg" href="/">
                        {"なんでも人狼"}
                    </a>
                    <div class="ml-auto">
                        <ButtonSub onclick={set_theme.reform(move |_| if dark_mode {"light".into()} else { "dark".into()})}>
                            {if dark_mode {"ライトモード"} else { "ダークモード"}}
                        </ButtonSub>
                        {button_link("報告","https://github.com/eatski/nandemo-jinro/issues")}
                    </div>

                </div>
            </header>
            <main class="px-7 py-10">
                {props.children.clone()}
            </main>
        </div>
    }
}
