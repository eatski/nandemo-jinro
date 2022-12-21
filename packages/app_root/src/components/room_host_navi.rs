use yew::{Properties, function_component, Children, Html, html, classes, Callback};


#[derive(Clone, PartialEq)]
pub enum LinkStatus {
    Current {
        done: bool,
    },
    Clickable {
        onclick: Callback<()>,
        done: bool,
    },
    Disabled,
}


#[derive(Properties, Clone, PartialEq)]
struct IconWrapperProps {
    children: Children,
    status: LinkStatus,
    label: &'static str,
}

fn checked_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="w-4 h-4">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" />
        </svg>
    }
}

#[function_component]
fn IconWrapper (props: &IconWrapperProps) -> Html {
    let status_classname = match props.status {
        LinkStatus::Current { .. }  => "text-action cursor-default",
        LinkStatus::Clickable { .. } => "m-auto w-6 w-6 text-word-2nd hover:text-word-hover", //TODO: text-word-hoverはまだない
        LinkStatus::Disabled => "m-auto w-6 w-6 text-word-disable cursor-default",
    };
    let onclick = match props.status {
        LinkStatus::Clickable { ref onclick,.. } => Some(onclick.clone()),
        _ => None,
    };
    let done = match props.status {
        LinkStatus::Clickable { done, .. } => done,
        LinkStatus::Current { done } => done,
        _ => false,
    };
    html! {
        <button class={classes!("h-8","w-8", "flex", "relative")} onclick={onclick.map(|onclick| onclick.reform(|_| ()))} title={props.label}>
            <div class={status_classname} role="img" aria-label={props.label}>
                {props.children.clone()}
            </div>
            {done.then(
                || html!{<div class={classes!("absolute", "-bottom-1", "-right-1", "text-accents")} aria-label={"済"} >
                    {checked_icon()}
                </div>}
            )}
        </button>
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
            <IconWrapper status={props.lobby.clone()} label="メンバーを集める">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-full h-full">
                    <path d="M4.5 6.375a4.125 4.125 0 118.25 0 4.125 4.125 0 01-8.25 0zM14.25 8.625a3.375 3.375 0 116.75 0 3.375 3.375 0 01-6.75 0zM1.5 19.125a7.125 7.125 0 0114.25 0v.003l-.001.119a.75.75 0 01-.363.63 13.067 13.067 0 01-6.761 1.873c-2.472 0-4.786-.684-6.76-1.873a.75.75 0 01-.364-.63l-.001-.122zM17.25 19.128l-.001.144a2.25 2.25 0 01-.233.96 10.088 10.088 0 005.06-1.01.75.75 0 00.42-.643 4.875 4.875 0 00-6.957-4.611 8.586 8.586 0 011.71 5.157v.003z" />
                </svg>
            </IconWrapper>
            <IconWrapper status={props.make_rule.clone()} label="ルールを決める">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-full h-full">
                    <path d="M21.731 2.269a2.625 2.625 0 00-3.712 0l-1.157 1.157 3.712 3.712 1.157-1.157a2.625 2.625 0 000-3.712zM19.513 8.199l-3.712-3.712-8.4 8.4a5.25 5.25 0 00-1.32 2.214l-.8 2.685a.75.75 0 00.933.933l2.685-.8a5.25 5.25 0 002.214-1.32l8.4-8.4z" />
                    <path d="M5.25 5.25a3 3 0 00-3 3v10.5a3 3 0 003 3h10.5a3 3 0 003-3V13.5a.75.75 0 00-1.5 0v5.25a1.5 1.5 0 01-1.5 1.5H5.25a1.5 1.5 0 01-1.5-1.5V8.25a1.5 1.5 0 011.5-1.5h5.25a.75.75 0 000-1.5H5.25z" />
                </svg>
            </IconWrapper>
            <IconWrapper status={props.confirm.clone()} label="役職を配る">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-full h-full">
                    <path d="M3.478 2.405a.75.75 0 00-.926.94l2.432 7.905H13.5a.75.75 0 010 1.5H4.984l-2.432 7.905a.75.75 0 00.926.94 60.519 60.519 0 0018.445-8.986.75.75 0 000-1.218A60.517 60.517 0 003.478 2.405z" />
                </svg>
            </IconWrapper>
        </nav>
    }
}