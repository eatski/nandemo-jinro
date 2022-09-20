use yew::{function_component, html, use_effect_with_deps, use_state, UseStateHandle, Callback};

use crate::firestore::{sync_members, MemberJSON};


enum LobbyState {
    Loading,
    Loaded(Vec<MemberJSON>),
}

#[function_component(Lobby)]
pub fn lobby() -> Html {
    let state: UseStateHandle<LobbyState>  = use_state(|| LobbyState::Loading);
    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                let cleanup = sync_members(
                    "test",
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
    let add_member = Callback::from(|_| {
        crate::firestore::add_members("test");
    });

    match &*state {
        LobbyState::Loading => {
            html! {
                <div class="animate-spin h-10 w-10 border-4 border-quiet-500 rounded-full border-t-transparent"></div>
            }
        },
        LobbyState::Loaded(state) => {
            html! { 
                <div>
                    <h1>{"Lobby"}</h1>
                    <ul>
                        { for state.iter().map(|member| html! { <li key={member.id.to_string()}>{&member.name}</li> }) }
                    </ul>
                    <button class={"bg-feature hover:bg-feature-light text-white py-2 px-4 rounded-md"} onclick={add_member}>{"参加する"}</button>
                </div>
            }
        },
    }

    
}