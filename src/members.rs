use presentational::{button, loading};
use yew::{function_component, html, use_effect_with_deps, use_state, UseStateHandle, Callback, Properties};

use crate::{firestore::{sync_members, MemberJSON, MemberInput}, storage::{is_host, get_user_id}};


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
    Joined(MemberType),
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
                        let user_status = if user_id.is_some() {
                            if is_host(cloned_room_id.as_str()) {
                                UserStatus::Joined(MemberType::Host)
                            } else {
                                UserStatus::Joined(MemberType::Guest)
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
    let add_member = Callback::from(move |_| {
        crate::firestore::add_members(room_id.as_str(),&MemberInput {
            name:"testes".to_string(),
        },|_| {
            
        });
    });

    match &*state {
        LobbyState::Loading => loading(),
        LobbyState::Loaded(state,lobby_type) => {
            html! { 
                <div>
                    <h1>{"Lobby"}</h1>
                    <ul>
                        { for state.iter().map(|member| html! { <li key={member.id.to_string()}>{&member.name}</li> }) }
                    </ul>
                    {
                        match lobby_type {
                            UserStatus::Joined(MemberType::Host) => html! { <button>{ "Start" }</button> },
                            UserStatus::Joined(MemberType::Guest) => html! { "待っててね" },
                            UserStatus::NotJoined => html! { <button onclick={add_member}>{"Join"}</button> },
                        }
                    }
                </div>
            }
        },
    }

    
}