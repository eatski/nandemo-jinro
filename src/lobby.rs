use presentational::{loading, BoxListContainer,SimpleCenteringSection,Heading2WithDescription, SimpleCenteringDiv,item_box, button};
use yew::{function_component, html, use_effect_with_deps, use_state, UseStateHandle, Callback, Properties};
use crate::entrance::{GuestEntrance};

use crate::{storage::{get_user_id}};
use crate::rule_make::{RuleMake};
use firestore::{sync_members, MemberJSON};

enum LobbyState {
    Loading,
    Loaded(Vec<MemberJSON>,UserStatus),
}
enum MemberType {
    Host,
    Guest,
}

enum UserStatus {
    Joined(MemberType,String),
    NotJoined,
}


#[derive(Properties, PartialEq)]
pub struct LobbyProps {
    pub room_id: String,
}

#[function_component(Lobby)]
pub fn lobby(props: &LobbyProps) -> Html {
    let state: UseStateHandle<LobbyState>  = use_state(|| (LobbyState::Loading));
    {
        let state = state.clone();
        let room_id = props.room_id.clone();
        use_effect_with_deps(
            |room_id| {
                let cloned_room_id = room_id.clone();
                sync_members(
                    room_id.as_str(),
                    move |members| {
                        let user_id = get_user_id(cloned_room_id.as_str());
                       
                        let user_status = if let Some(user_id) = user_id {
                            let is_host = members.iter()
                                .find(|member| member.id == user_id)
                                .map(|member| member.is_host)
                                .unwrap_or(false);
                            if is_host {
                                UserStatus::Joined(MemberType::Host,user_id)
                            } else {
                                UserStatus::Joined(MemberType::Guest,user_id)
                            }
                        } else {
                            UserStatus::NotJoined
                        };
                        state.set(LobbyState::Loaded(members,user_status))
                    },
                    || {},
                )
            },
            room_id,
        );
    }
    let room_id = props.room_id.clone();

    match &*state {
        LobbyState::Loading => loading(),
        LobbyState::Loaded(members,user_status) => {
            match user_status {
                UserStatus::Joined(member_type, user_id) => {
                    html! {
                        <>
                        <SimpleCenteringSection>
                            {
                                match member_type {
                                    MemberType::Host =>  html! {
                                        <Heading2WithDescription title={"メンバーを集めましょう"} description={"このページのURLを一緒に遊ぶメンバーに共有しましょう"}/>
                                    },
                                    MemberType::Guest => html! {
                                        <Heading2WithDescription title={"部屋に参加しました"} description={"ホストがゲームを始めるのを待ちましょう"}/>
                                    },
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
                            <SimpleCenteringDiv>
                                {
                                    match member_type {
                                        MemberType::Host => button("メンバー確定", Callback::from(|_| {}) ),
                                        MemberType::Guest => html! {},
                                    }
                                }
                            </SimpleCenteringDiv>
                        </SimpleCenteringSection>
                        {
                            match member_type {
                                MemberType::Host => html! {
                                    <SimpleCenteringSection>
                                        <Heading2WithDescription title={"ルールを決めましょう"} description={"役職とその人数を決めましょう"}/>
                                        <RuleMake />
                                    </SimpleCenteringSection>
                                },
                                MemberType::Guest => html! {},
                            }
                        }
                        </>
                    }
                    
                },
                UserStatus::NotJoined => {
                    html! { <GuestEntrance {room_id}/> }
                },
            }
        },
    }
}
