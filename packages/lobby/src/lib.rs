use model::{MemberJSON,SetCanJoin};
use presentational::{loading,SimpleCenteringSection, Heading2,HeadingDescription};
use yew::{Properties, function_component, html, use_state, Callback};
use firestore_hooks::{use_document, DataFetchState, use_collection_sync};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
    pub user_id: String,
}

#[function_component(Lobby)]
pub fn lobby(props: &Props) -> Html {
    let members_state = use_collection_sync::<MemberJSON>(&props.room_id);
    let you_state = use_document::<MemberJSON>(&props.room_id, props.user_id.as_str());

    let state = members_state.merge::<>(you_state);
    
    match state {
        DataFetchState::Loaded((members,you)) => {
            let is_host = you.is_host;
            let user_id = props.user_id.clone();
            html! {
                <section class="mx-auto w-full max-w-2xl py-2">
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
                                        <div class="relative bg-colored-light w-60 border-line border-solid border rounded-full py-0.5 text-center text-md text-black-light">
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
                    {{   
                        let room_id = props.room_id.clone();
                        is_host.then(|| {
                            html! {
                                <div class="absolute bottom-10 left-1/2 -translate-x-1/2">
                                    <div class="mx-auto flex justify-center w-full max-w-2xl py-2">
                                        <MemberClose room_id={room_id}/>
                                    </div>
                                </div>
                            }
                        }).unwrap_or_default()
                    }}
                </section>
                
            }
        },
        DataFetchState::Loading => {
            html! {
                <SimpleCenteringSection>
                    {loading()}
                </SimpleCenteringSection>
            }
        }
    }
}
#[derive(Properties, PartialEq)]
struct MemberCloseProps {
    pub room_id: String,
}

#[function_component[MemberClose]]
fn member_close(props: &MemberCloseProps) -> Html {
    enum State {
        Loading,
        Clickable
    }
    let state = use_state(|| State::Clickable);
    let room_id = props.room_id.clone();
    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(State::Loading);
            firestore::set_document(
                &(),
                room_id.as_str(), 
                &SetCanJoin { can_join: false },
                || {}, 
                || {}
            );
        })
    };
    match &*state {
        State::Loading => loading(),
        State::Clickable => {
            html! {
                <button 
                    {onclick}
                    class={"bg-feature hover:bg-feature-light text-white py-4 px-9 text-lg rounded-full"}
                >
                    {"締め切る"}
                </button>
            }
        }
    }
}