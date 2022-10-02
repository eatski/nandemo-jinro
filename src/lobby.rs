use firestore::{MemberJSON, future::sync_collection};
use presentational::{loading,SimpleCenteringSection,Heading2WithDescription, SimpleCenteringDiv,item_box, button,BoxListContainer};
use yew::{Properties, function_component, html, UseStateHandle, use_state, use_effect_with_deps, Callback};
use crate::{hook::{MemberState, use_document}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
    pub user_id: String,
}

enum MembersState {
    Loading,
    Loaded(Vec<MemberJSON>),
}

enum State {
    Loading,
    Loaded {
        members: Vec<MemberJSON>,
        you: MemberJSON,
    },
}

#[function_component(Lobby)]
pub fn lobby(props: &Props) -> Html {
    let members_state: UseStateHandle<MembersState>  = use_state(|| (MembersState::Loading));
    {
        let state = members_state.clone();
        let room_id = props.room_id.clone();
        use_effect_with_deps(
            |room_id| {
                sync_collection::<MemberJSON>(
                    room_id,
                    move |members| {
                        state.set(MembersState::Loaded(members))
                    },
                    || {},
                )
            },
            room_id,
        );
    }
    let you_state = use_document::<MemberJSON>(&props.room_id, props.user_id.as_str());

    let state = {
        let members_state = &*members_state;
        let you_state = you_state;
        match (members_state, you_state) {
            (MembersState::Loading, MemberState::Loading) => State::Loading,
            (MembersState::Loading, MemberState::Loaded(_)) => State::Loading,
            (MembersState::Loaded(_), MemberState::Loading) => State::Loading,
            (MembersState::Loaded(members), MemberState::Loaded(you)) => State::Loaded { members: members.clone(), you },
        }
    };
    
    match state {
        State::Loaded{members,you} => {
            let is_host = you.is_host;
            let user_id = props.user_id.clone();
            html! {
                <>
                    <SimpleCenteringSection>
                        {
                            if is_host {
                                html! {
                                    <Heading2WithDescription title={"メンバーを集めましょう"} description={"このページのURLを一緒に遊ぶメンバーに共有しましょう"}/>
                                }
                            } else {
                                html! {
                                    <Heading2WithDescription title={"部屋に参加しました"} description={"ホストがゲームを始めるのを待ちましょう"}/>
                                }
                            }
                        }
                        <BoxListContainer>
                            {
                                for members.iter().map(|member| {
                                    let is_you = member.id == *user_id;
                                    html! {
                                        <li>
                                            {item_box(member.name.as_str(),is_you.then(|| "あなた"))}
                                        </li>
                                    }
                                })
                            }
                        </BoxListContainer>
                        {{   
                            let room_id = props.room_id.clone();
                            is_host.then(|| {
                                html! {
                                    <SimpleCenteringDiv>
                                        <MemberClose room_id={room_id}/>
                                    </SimpleCenteringDiv>
                                }
                            }).unwrap_or_default()
                        }}
                    </SimpleCenteringSection>
                </>
            }
        },
        State::Loading => {
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

    match &*state {
        State::Loading => loading(),
        State::Clickable => {
            button("締め切る", Callback::from(move |_| {
                state.set(State::Loading);
                firestore::set_can_join_false(room_id.as_str(), || {}, || {});
            }))
        }
    }
}