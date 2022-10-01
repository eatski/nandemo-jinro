use std::{iter::repeat};

use firestore::{UserToRole, add_roll,Roll, get_rolls};
use presentational::loading;
use yew::{function_component, html, Callback, Properties};
use rand::{seq::SliceRandom};

use crate::{state_hooks::{use_room_sync, use_members, DataFetchState}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
}

#[function_component(RollButton)]
pub fn roll(props: &Props) -> Html {
    let room = use_room_sync(props.room_id.as_str());
    let members = use_members(props.room_id.as_str());
    let state = room.merge(members);

    match state {
    DataFetchState::Loading => loading(),
    DataFetchState::Loaded((room,members)) => {
        let room_id = props.room_id.clone();
        let onclick = Callback::from(move |_| {
            let room = room.clone();
            let mut rng = rand::thread_rng();
            if let Some(rule) = room.rule {
                let mut roles: Vec<_> = rule.roles.into_iter()
                    .flat_map(|role_input| repeat(role_input.id).take(role_input.number)).collect();
                roles.shuffle(&mut rng);
                let user_to_role: UserToRole = members
                    .iter()
                    .map(|member| (member.id.clone(), roles.pop().expect("Not enough roles")))
                    .collect();
                let room_id_cloned = room_id.clone();
                get_rolls(room_id.as_str(), move |rolls| {
                    let seq_num = rolls.len();
                    let roll = Roll { user_to_role,seq_num };
                     add_roll(room_id_cloned.as_str(), roll, || {});
                },|| {});
            }
        });
        html! {
            <button onclick={onclick}>
                {"Roll"}
            </button>
        }
    },
}
}