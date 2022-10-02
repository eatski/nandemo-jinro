use firestore::{future::{get_collection, add_document}, Roll, MemberJSON, Room};
use presentational::loading;
use yew::{function_component, html, Callback, Properties};

use crate::{hook::{DataFetchState, use_collection, use_document_sync}, function::create_next_roll};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
}

#[function_component(RollButton)]
pub fn roll(props: &Props) -> Html {
    let room = use_document_sync::<Room>(&(),props.room_id.as_str());
    let members = use_collection::<MemberJSON>(&props.room_id);
    let state = room.merge(members);

    match state {
    DataFetchState::Loading => loading(),
    DataFetchState::Loaded((room,members)) => {
        let room_id = props.room_id.clone();
        let onclick = Callback::from(move |_| {
            let room = room.clone();
            let members = members.clone();
            let room_id_cloned = room_id.clone();
            if let Some(rule) = room.rule {
                get_collection::<Roll>(&room_id, move |rolls| {
                    let roll = create_next_roll(&rule, &members, &rolls);
                    add_document(&room_id_cloned, &roll, |_| {},|| {});
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