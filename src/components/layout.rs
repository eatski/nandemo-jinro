use atoms::button_link;
use yew::{Properties, Children, html, function_component};


#[derive(Properties, PartialEq)]
pub struct ChildrenOnlyProps {
    pub children: Children, // the field name `children` is important!
}

#[function_component(Layout)]
pub fn layout(props: &ChildrenOnlyProps) -> Html {
    html! {
        <>
            <header class="w-full border-line border-solid border-b px-4 py-3 ">
                <div class="flex items-center">
                    <a class="font-mono text-black text-lg" href="/">
                        {"なんでも人狼"}
                    </a>
                    <div class="ml-auto">
                        {button_link("報告","/")}
                    </div>
                </div>
            </header>
            <main class="px-7 py-8">
                {props.children.clone()}
            </main>
        </>
    }
}