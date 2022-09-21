use presentational::{button, loading};
use yew::{function_component, html, use_effect_with_deps, use_state, UseStateHandle, Callback, Properties};

use crate::firestore::{sync_members, MemberJSON, MemberInput};


enum LobbyState {
    Loading,
    Loaded(Vec<MemberJSON>),
}


#[derive(Properties, PartialEq)]
pub struct LobbyProps {
    pub room_id: String,
}

#[function_component(Lobby)]
pub fn lobby(props: &LobbyProps) -> Html {
    let state: UseStateHandle<LobbyState>  = use_state(|| LobbyState::Loading);
    {
        let state = state.clone();
        let room_id = props.room_id.clone();
        use_effect_with_deps(
            move |_| {
                let cleanup = sync_members(
                    room_id.as_str(),
                    move |members| state.set(LobbyState::Loaded(members)),
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
        LobbyState::Loaded(state) => {
            html! { 
                <div>
                    <h1>{"Lobby"}</h1>
                    <ul>
                        { for state.iter().map(|member| html! { <li key={member.id.to_string()}>{&member.name}</li> }) }
                    </ul>
                    {button("参加する",add_member)}
                </div>
            }
        },
    }

    
}