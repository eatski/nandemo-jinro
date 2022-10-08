use model::{Roll, Room, MemberJSON};
use atoms::{loading,Heading2,};
use yew::{html, Properties, function_component, Callback};
use layouting::{BodyItems,BottomOperaton};

use firestore_hooks::{use_collection_sync, use_document_sync, use_document};

use crate::use_roll::use_roll;
use crate::common::{RollButton};

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
                    <section>
                        <BodyItems>
                            <Heading2>{"あなたの役職は"}<b>{role_name}</b>{"です"}</Heading2>
                        </BodyItems>
                        {
                            member.is_host.then(|| {
                                match roll {
                                    Some(roll) => {
                                        html! {
                                            <BottomOperaton>
                                                <RollButton onclick={Callback::once(move |_| roll())}>
                                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-full h-full">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                                                    </svg>
                                                </RollButton>
                                            </BottomOperaton>
                                        }
                                    },
                                    None => loading(),
                                }
                                
                            }).unwrap_or_default()
                        }
                        
                        

                       
                    </section>
                }
            }
            
            None => html! {

            },
        }
    },
    }
}