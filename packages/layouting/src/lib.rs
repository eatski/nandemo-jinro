use yew::{html, function_component, Properties, Children};

#[derive(Properties, PartialEq)]
pub struct ChildrenOnlyProps {
    pub children: Children, // the field name `children` is important!
}


#[function_component(FixToBottom)]
pub fn fix_to_bottom(props: &ChildrenOnlyProps) -> Html{
    html! {
        <div class="fixed bottom-16 left-1/2 -translate-x-1/2">
            {props.children.clone()}
        </div>
    }
}