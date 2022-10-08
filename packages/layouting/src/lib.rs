use yew::{html, function_component, Properties, Children};

#[derive(Properties, PartialEq)]
pub struct ChildrenOnlyProps {
    pub children: Children, // the field name `children` is important!
}

#[function_component[BodyItems]]
pub fn body_items(props: &ChildrenOnlyProps) -> Html {
    html! {
        <div class="h-96 overflow-y-auto">
            {props.children.clone()}
        </div>
    }
}

#[function_component[BottomOperaton]]
pub fn bottom_operation(props: &ChildrenOnlyProps) -> Html {
    html! {
        <div class="flex justify-center mt-8">
            {props.children.clone()}
        </div>
    }
}