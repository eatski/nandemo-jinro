use presentational::loading;
use yew::{html, Properties, function_component};

use crate::state_hooks::{use_rolls, use_room_sync};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
    pub user_id: String,
}

#[function_component(Rolled)]
pub fn rolled(props: &Props) -> Html {
    let rolls = use_rolls(props.room_id.as_str());
    let room = use_room_sync(props.room_id.as_str());
    let state = rolls.merge(room);
    match state {
    crate::state_hooks::DataFetchState::Loading => loading(),
    crate::state_hooks::DataFetchState::Loaded((mut rolls,room)) => {
        rolls.sort_by_key(|roll| roll.seq_num);
        let last_rolled = rolls
            .last();
        match last_rolled {
            Some(last_rolled) => {
                let role = last_rolled.user_to_role.get(props.user_id.as_str()).unwrap();
                let role_name = room.rule.as_ref().unwrap().roles.iter().find(|role_input| role_input.id == *role).unwrap().name.clone();
                html! {
                    <>
                        <h2>{"Rolled"}</h2>
                        <p>{role_name}</p>
                    </>
                }
            }
            
            None => html! {

            },
        }
    },
    }
}