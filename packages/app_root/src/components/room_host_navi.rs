use yew::{Properties, function_component, Children, Html, html, classes, Callback};


#[derive(Clone, PartialEq)]
pub enum LinkStatus {
    Current,
    Clickable {
        onclick: Callback<()>,
    },
    Disabled,
}

#[derive(Properties, Clone, PartialEq)]
struct IconWrapperProps {
    children: Children,
    status: LinkStatus,
}

#[function_component]
fn IconWrapper (props: &IconWrapperProps) -> Html {
    let status_classname = match props.status {
        LinkStatus::Current => "text-action",
        LinkStatus::Clickable { .. } => "m-auto w-6 w-6 text-word-2nd hover:text-word-hover hover:cursor-pointer", //TODO: text-word-hoverはまだない
        LinkStatus::Disabled => "m-auto w-6 w-6 text-word-disable",
    };
    let onclick = match props.status {
        LinkStatus::Clickable { ref onclick } => Some(onclick.clone()),
        _ => None,
    };
    html! {
        <a class={classes!("h-8","w-8", "flex")} onclick={onclick.map(|onclick| onclick.reform(|_| ()))}>
            <div class={status_classname}>
                {props.children.clone()}
            </div>
            
        </a>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct RoomHostNaviProps {
    pub lobby: LinkStatus,
    pub make_rule: LinkStatus,
    pub confirm: LinkStatus,
}

#[function_component]
pub fn RoomHostNavi(props: &RoomHostNaviProps) -> Html {
    html! {
        <nav class="flex justify-center gap-12 text-word-2nd mb-4">
            <IconWrapper status={props.lobby.clone()}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-full h-full">
                    <path d="M4.5 6.375a4.125 4.125 0 118.25 0 4.125 4.125 0 01-8.25 0zM14.25 8.625a3.375 3.375 0 116.75 0 3.375 3.375 0 01-6.75 0zM1.5 19.125a7.125 7.125 0 0114.25 0v.003l-.001.119a.75.75 0 01-.363.63 13.067 13.067 0 01-6.761 1.873c-2.472 0-4.786-.684-6.76-1.873a.75.75 0 01-.364-.63l-.001-.122zM17.25 19.128l-.001.144a2.25 2.25 0 01-.233.96 10.088 10.088 0 005.06-1.01.75.75 0 00.42-.643 4.875 4.875 0 00-6.957-4.611 8.586 8.586 0 011.71 5.157v.003z" />
                </svg>
            </IconWrapper>
            <IconWrapper status={props.make_rule.clone()}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-full h-full">
                    <path d="M21.731 2.269a2.625 2.625 0 00-3.712 0l-1.157 1.157 3.712 3.712 1.157-1.157a2.625 2.625 0 000-3.712zM19.513 8.199l-3.712-3.712-8.4 8.4a5.25 5.25 0 00-1.32 2.214l-.8 2.685a.75.75 0 00.933.933l2.685-.8a5.25 5.25 0 002.214-1.32l8.4-8.4z" />
                    <path d="M5.25 5.25a3 3 0 00-3 3v10.5a3 3 0 003 3h10.5a3 3 0 003-3V13.5a.75.75 0 00-1.5 0v5.25a1.5 1.5 0 01-1.5 1.5H5.25a1.5 1.5 0 01-1.5-1.5V8.25a1.5 1.5 0 011.5-1.5h5.25a.75.75 0 000-1.5H5.25z" />
                </svg>
            </IconWrapper>
            <IconWrapper status={props.confirm.clone()}>
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-full h-full">
                    <path fill-rule="evenodd" d="M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12zm13.36-1.814a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd" />
                </svg>
            </IconWrapper>
        </nav>
    }
}