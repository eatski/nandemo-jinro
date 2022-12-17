use atoms::{loading, ButtonLarge, Heading2, HeadingDescription, unexpected_error};
use firestore_hooks::{use_collection_sync, use_document, NotFetched};
use layouting::{BodyItems, BottomOperaton};
use model::{MemberJSON, RoomEditAction, RoomEditBody};
use web_sys::window;
use yew::{function_component, html, Properties, Callback, Html, use_state};
use use_historical::{use_historical, YewHistorical};

mod clipboard;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
    pub user_id: String,
    pub on_complete: Option<Callback<()>>,
}



#[function_component(Lobby)]
pub fn lobby(props: &Props) -> Html {
    let members_state = use_collection_sync::<MemberJSON>(&props.room_id);
    let you_state = use_document::<MemberJSON>(&props.room_id, props.user_id.as_str());
    let state = (|| Ok((members_state?,you_state?)) )();
    match state {
        Ok((members, you)) => {
            let is_host = you.is_host;
            let user_id = props.user_id.clone();
            html! {
                <section class="mx-auto w-full max-w-2xl py-2">
                    <BodyItems>
                        {
                            if is_host {
                                html! {
                                    <>
                                        <Heading2>{"メンバーを集めましょう"}</Heading2>
                                        <div class="flex justify-center">
                                            <HeadingDescription>
                                                {"このページのURLを一緒に遊ぶメンバーに共有しましょう"}
                                            </HeadingDescription>
                                            <button aria-label="クリップボードにURLを保存" title="クリップボードにURLを保存" onclick={Callback::from(|_| clipboard::wirteClickBoard(&window().unwrap().location().href().unwrap()))}>
                                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="text-word hover:text-word-2nd w-5 h-5">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 002.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 00-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75 2.25 2.25 0 00-.1-.664m-5.8 0A2.251 2.251 0 0113.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25zM6.75 12h.008v.008H6.75V12zm0 3h.008v.008H6.75V15zm0 3h.008v.008H6.75V18z" />
                                                </svg>
                                            </button>
                                        </div>
                                    </>
                                }
                            } else {
                                html! {
                                    <>
                                        <Heading2>{"部屋に参加しました"}</Heading2>
                                        <HeadingDescription>{"ホストがゲームを始めるのを待ちましょう"}</HeadingDescription>
                                    </>
                                }
                            }
                        }
                        <ul class="flex flex-col gap-2 py-8">
                            {
                                for members.iter().map(|member| {
                                    let is_you = member.id == *user_id;
                                    html! {
                                        <li class="flex justify-center">
                                            <div class="relative transition-colors bg-layer-2nd w-60 border-separator border-solid border rounded-full py-1 text-center text-md text-word-2nd">
                                                {member.name.as_str()}
                                                <span class="absolute top-1/2 right-5">
                                                    {is_you.then(|| html! {
                                                        <span role="img" aria-label={"あなた"} class="relative h-2 w-2">
                                                            <span class="animate-pulse absolute -translate-y-1/2 top-0 left-0 rounded-full transition-colors bg-action h-2 w-2" />
                                                        </span>
                                                    }).unwrap_or_default()}
                                                </span>
                                            </div>
                                        </li>
                                    }
                                })
                            }
                        </ul>
                    </BodyItems>
                    {{
                        let room_id = props.room_id.clone();
                        is_host.then(|| {
                            html! {
                                <BottomOperaton>
                                    <MemberClose room_id={room_id} on_complete={props.on_complete.clone()}/>
                                </BottomOperaton>
                            }
                        }).unwrap_or_default()
                    }}
                </section>

            }
        }
        Err(NotFetched::Loading) => loading(),
        Err(NotFetched::Error)  => unexpected_error()
    }
}
#[derive(Properties, PartialEq)]
struct MemberCloseProps {
    pub room_id: String,
    pub on_complete: Option<Callback<()>>,
}

#[function_component[MemberClose]]
fn member_close(props: &MemberCloseProps) -> Html {
    let room = use_historical::<RoomEditAction,RoomEditBody>(props.room_id.clone(), |signature,body| RoomEditAction {signature, body});
    let members = use_collection_sync::<MemberJSON>(&props.room_id);
    let updateing = use_state(|| false);
    if !*updateing {
        match (|| Ok((room?,members?)))() {
            Err(NotFetched::Loading) => loading(),
            Ok((YewHistorical {push_with_callback,latest},members)) => {
                if latest.can_join {
                    let on_complete = props.on_complete.clone();
                    let onclick = push_with_callback.reform(move |_| {
                        let on_complete = on_complete.clone();
                        updateing.set(true);
                        let updateing = updateing.clone();
                        (RoomEditBody::SetCanJoin(false),Box::new(move || {
                            on_complete.map(|callback| callback.emit(()));
                            updateing.set(false);
                        }))
                    });
                    html! {
                        <ButtonLarge
                            onclick={onclick}
                            disabled={members.len() <= 1}
                        >
                            {"締め切る"}
                        </ButtonLarge>
                    }
                } else {
                    html! {
                        <ButtonLarge
                            onclick={push_with_callback.reform(|_| (RoomEditBody::SetCanJoin(true),Box::new(|| {})))}
                        >
                            {"募集を再開"}
                        </ButtonLarge>
                    }
                }   
            },
            Err(NotFetched::Error) => unexpected_error()
        }
    } else {
        loading()
    }
    
}
