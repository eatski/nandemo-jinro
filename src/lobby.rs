use presentational::{InputAndButton, loading, BoxListContainer, title,SimpleCenteringSection,Heading2WithDescription,Heading2, item_box};
use yew::{function_component, html, use_effect_with_deps, use_state, UseStateHandle, Callback, Properties};

use crate::{storage::{is_host, get_user_id}};
use firestore::{sync_members, MemberJSON, MemberInput, add_members};

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
                            if is_host(cloned_room_id.as_str()) {
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
                        </SimpleCenteringSection>
                    }
                    
                },
                UserStatus::NotJoined => {
                    html! { <GuestEntrance {room_id}/> }
                },
            }
        },
    }
}

#[derive(Properties, PartialEq)]
struct GuestEntranceProps {
    room_id: String,
}

#[function_component(GuestEntrance)]
fn guest_entrance(props: &GuestEntranceProps) -> Html {
    enum State {
        Loading,
        Loaded {
            host_name: String,
        },
        Error,
    }
    let state: UseStateHandle<State> = use_state(|| State::Loading);
    {
        let state = state.clone();
        let state_on_error = state.clone();
        let room_id = props.room_id.clone();
        use_effect_with_deps(
            |room_id| {
                firestore::get_members(
                    room_id.as_str(),
                    move |members| {
                        let host_name = members
                            .iter()
                            .find(|member| member.is_host)
                            .map(|member| member.name.clone());
                        match host_name {
                            Some(host_name) => state.set(State::Loaded { host_name }),
                            None => state.set(State::Error),
                        };
                    },
                    move || {
                        state_on_error.set(State::Error);
                    },
                );
                || {}
            },
            room_id,
        );
    }
    let room_id = props.room_id.clone();
    let add_member = Callback::from(move |name| {
        let room_id_cloned = room_id.clone();
        let user_id = add_members(
            room_id.as_str(),
            &MemberInput {
                name,
                is_host: false,
            },
            move || {},
            || {}
        );
        crate::storage::save_user_id(room_id_cloned.as_str(),user_id.as_str());
    });

    match &*state {
        State::Loading => loading(),
        State::Loaded { host_name } => html! {
            <div>
                {title()}
                <SimpleCenteringSection>
                    <Heading2>{ format!("「{}」の部屋",host_name)}</Heading2>
                    <InputAndButton label="参加" placeholder="あなたの名前" onsubmit={add_member} />
                </SimpleCenteringSection>
            </div>
        },
        State::Error => todo!(),
    }
}