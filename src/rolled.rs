use model::{Roll, Room, MemberJSON};
use presentational::loading;
use yew::{html, Properties, function_component};

use crate::{hooks::firestore::{use_collection_sync, use_document_sync, use_document}};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
    pub user_id: String,
}

#[function_component(Rolled)]
pub fn rolled(props: &Props) -> Html {
    let rolls = use_collection_sync::<Roll>(&props.room_id);
    let room = use_document_sync::<Room>(&(),props.room_id.as_str());
    let member = use_document::<MemberJSON>(&props.room_id, props.user_id.as_str());
    let state = rolls.merge(room).merge(member);
    match state {
    crate::hooks::firestore::DataFetchState::Loading => loading(),
    crate::hooks::firestore::DataFetchState::Loaded(((mut rolls,room),member)) => {
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
                        {
                            member.is_host.then(|| {
                                // TODO: Add a button to go to the next round
                                html! {
                                    <button>{"Next"}</button>
                                }
                            }).unwrap_or_default()
                        }
                    </>
                }
            }
            
            None => html! {

            },
        }
    },
    }
}