use yew::{function_component, html, use_effect_with_deps, use_state, UseStateHandle, Callback};

use crate::firestore::{sync_members, MemberJSON};

#[function_component(Lobby)]
pub fn lobby() -> Html {
    let state: UseStateHandle<Vec<MemberJSON>>  = use_state(Vec::new);
    {
        let state = state.clone();
        use_effect_with_deps(
            move |_| {
                let cleanup = sync_members(
                    "test",
                    move |members| state.set(members),
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
    html! { 
        <div>
            <h1>{"Lobby"}</h1>
            <ul>
                { for state.iter().map(|member| html! { <li key={member.id.to_string()}>{&member.name}</li> }) }
            </ul>
            <button onclick={add_member}>{"Add Member"}</button>
        </div>
    }
}