use model::{Roll, Room, MemberJSON};
use presentational::loading;
use yew::{html, Properties, function_component, Callback};

use firestore_hooks::{use_collection_sync, use_document_sync, use_document};

use crate::use_roll::use_roll;

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
    let roll = use_roll(props.room_id.as_str());
    match state {
    firestore_hooks::DataFetchState::Loading => loading(),
    firestore_hooks::DataFetchState::Loaded(((mut rolls,room),member)) => {
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
                                match roll {
                                    Some(roll) => {
                                        let onclick = Callback::once(move |_| {
                                            roll();
                                        });
                                        html! {
                                            <button onclick={onclick}>{"Next"}</button>
                                        }
                                    },
                                    None => loading(),
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