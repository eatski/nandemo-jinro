use firestore::{add_roll, get_rolls};
use presentational::loading;
use yew::{function_component, html, Callback, Properties};

use crate::{hook::{use_room_sync, use_members, DataFetchState}, function::create_next_roll};

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
            let members = members.clone();
            let room_id_cloned = room_id.clone();
            if let Some(rule) = room.rule {
                get_rolls(room_id.as_str(), move |rolls| {
                    let roll = create_next_roll(&rule, &members, &rolls);
                    add_roll(room_id_cloned.as_str(), &roll, || {});
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