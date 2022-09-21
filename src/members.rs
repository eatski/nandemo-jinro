use presentational::{button, loading};
use yew::{function_component, html, use_effect_with_deps, use_state, UseStateHandle, Callback, Properties};

use crate::{firestore::{sync_members, MemberJSON, MemberInput}, storage::{is_host, get_user_id}};


enum LobbyState {
    Loading,
    Loaded(Vec<MemberJSON>,LobbyType),
}

struct Member {
    name: String,
    id: String,
}

enum LobbyType {
    Host,
    Guest,
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
            move |_| {
                let room_id_cloned = room_id.clone();
                let cleanup = sync_members(
                    room_id.as_str(),
                    move |members| {
                        let lobby_type = if is_host(room_id_cloned.as_str()) {
                            LobbyType::Host
                        } else {
                            if let Some(_) = get_user_id(room_id_cloned.as_str()) {
                                LobbyType::Guest
                            } else {
                                LobbyType::NotJoined
                            }
                        };
                        state.set(LobbyState::Loaded(members,lobby_type))
                    },
                    || {},
                );
                || {
                    cleanup();
                }
            },
            (),
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
                            LobbyType::Host => html! { <button>{ "Start" }</button> },
                            LobbyType::Guest => html! {<button onclick={add_member}>{"Add Member"}</button> },
                            LobbyType::NotJoined => html! { <button>{"Join"}</button> },
                        }
                    }
                </div>
            }
        },
    }

    
}