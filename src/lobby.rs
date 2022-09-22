use presentational::{InputAndButton, loading, mark};
use yew::{function_component, html, use_effect_with_deps, use_state, UseStateHandle, Callback, Properties};

use crate::{firestore::{sync_members, MemberJSON, MemberInput, add_members}, storage::{is_host, get_user_id}};


enum LobbyState {
    Loading,
    Loaded(Vec<MemberJSON>,UserStatus),
}

struct Member {
    name: String,
    id: String,
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
    let add_member = Callback::from(move |name| {
        let room_id_cloned = room_id.clone();
        let user_id = add_members(
            room_id.as_str(),
            &MemberInput {name},
            move || {}
        );
        crate::storage::save_user_id(room_id_cloned.as_str(),user_id.as_str());
    });

    match &*state {
        LobbyState::Loading => loading(),
        LobbyState::Loaded(state,lobby_type) => {
            html! { 
                <div>
                    <h1>{"Lobby"}</h1>
                    {
                        if let UserStatus::Joined(_,user_id) = lobby_type {
                            html! {
                                <ul>
                                    {for state.iter().map(|member| 
                                        {
                                            let is_you = member.id.as_str() == user_id;
                                            html! { 
                                                <li key={member.id.to_string()}>{&member.name}{if is_you { mark("you") } else {html!{}}}</li> 
                                            }
                                        }) 
                                    }
                                </ul>
                            }
                        } else {
                            html! {}
                        }
                    }
                    {
                        match lobby_type {
                            UserStatus::Joined(MemberType::Host,_) => html! { <button>{ "Start" }</button> },
                            UserStatus::Joined(MemberType::Guest,_) => html! { "待っててね" },
                            UserStatus::NotJoined => html! { <InputAndButton label="参加" placeholder="あなたの名前" onsubmit={add_member} /> },
                        }
                    }
                </div>
            }
        },
    }

    
}