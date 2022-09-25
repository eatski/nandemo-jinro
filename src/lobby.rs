use firestore::{MemberJSON, sync_members};
use presentational::{loading,SimpleCenteringSection,Heading2WithDescription, SimpleCenteringDiv,item_box, button,BoxListContainer};
use yew::{Properties, function_component, html, UseStateHandle, use_state, use_effect_with_deps, Callback};
use crate::rule_make::{RuleMake};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
    pub user_id: String,
}

enum LobbyState {
    Loading,
    Loaded(Vec<MemberJSON>),
}

#[function_component(Lobby)]
pub fn lobby(props: &Props) -> Html {
    let state: UseStateHandle<LobbyState>  = use_state(|| (LobbyState::Loading));
    {
        let state = state.clone();
        let room_id = props.room_id.clone();
        use_effect_with_deps(
            |room_id| {
                sync_members(
                    room_id.as_str(),
                    move |members| {
                        state.set(LobbyState::Loaded(members))
                    },
                    || {},
                )
            },
            room_id,
        );
    }

    match &*state {
        LobbyState::Loading => loading(),
        LobbyState::Loaded(members) => {
            let is_host = members.iter()
                .find(|member| member.id.as_str() == props.user_id.as_str())
                .map(|member| member.is_host)
                .unwrap_or(false);
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
                    {{
                        let room_id = props.room_id.clone();
                        is_host.then(|| {
                            html! {
                                <SimpleCenteringSection>
                                    <Heading2WithDescription title={"ルールを決めましょう"} description={"役職とその人数を決めましょう"}/>
                                    <RuleMake room_id={room_id}/>
                                </SimpleCenteringSection>
                            }
                        }).unwrap_or_default()
                    }}
                </>
            }
        },
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