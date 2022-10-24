use atoms::{loading, ButtonLarge, Heading2, HeadingDescription, unexpected_error};
use firestore_hooks::{use_collection_sync, use_document, DataFetchState};
use layouting::{BodyItems, BottomOperaton};
use model::{MemberJSON, RoomEditAction, RoomEditBody};
use yew::{function_component, html, Callback, Properties};
use use_historical::{use_historical, YewHistorical};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
    pub user_id: String,
}

#[function_component(Lobby)]
pub fn lobby(props: &Props) -> Html {
    let members_state = use_collection_sync::<MemberJSON>(&props.room_id);
    let you_state = use_document::<MemberJSON>(&props.room_id, props.user_id.as_str());

    let state = members_state.merge(you_state);

    match state {
        DataFetchState::Loaded((members, you)) => {
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
                                        <HeadingDescription>{"このページのURLを一緒に遊ぶメンバーに共有しましょう"}</HeadingDescription>
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
                                            <div class="relative bg-colored-light w-60 border-line border-solid border rounded-full py-1 text-center text-md text-black-light">
                                                {member.name.as_str()}
                                                <span class="absolute top-1/2 right-5">
                                                    {is_you.then(|| html! {
                                                        <span role="img" aria-label={"あなた"} class="relative h-2 w-2">
                                                            <span class="animate-pulse absolute -translate-y-1/2 top-0 left-0 rounded-full bg-feature-light h-2 w-2" />
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
                                    <MemberClose room_id={room_id}/>
                                </BottomOperaton>
                            }
                        }).unwrap_or_default()
                    }}
                </section>

            }
        }
        DataFetchState::Loading => loading(),
        DataFetchState::Error => unexpected_error()
    }
}
#[derive(Properties, PartialEq)]
struct MemberCloseProps {
    pub room_id: String,
}

#[function_component[MemberClose]]
fn member_close(props: &MemberCloseProps) -> Html {
    let state = use_historical::<RoomEditAction>(props.room_id.clone(), |signature| RoomEditAction {signature, body: RoomEditBody::SetCanJoin(false)});
    match state {
        DataFetchState::Loading => loading(),
        DataFetchState::Loaded(YewHistorical {current: _,push}) => {
            html! {
                <ButtonLarge
                    onclick={Callback::from(move |_| {
                        push();
                    })}
                >
                    {"締め切る"}
                </ButtonLarge>
            }
        },
        DataFetchState::Error => unexpected_error()
    }
}
