use model::{MemberInput, MemberJSON, Room};
use firestore::add_document;
use presentational::{loading,Heading2,InputAndButton};
use yew::{function_component, Properties,Callback, html};

use crate::{components::title::title, hooks::firestore::{use_document_sync, use_collection, DataFetchState}};


#[derive(Properties, PartialEq)]
pub struct GuestEntranceProps {
    pub room_id: String,
    pub on_join: Callback<String>,
}

#[function_component(GuestEntrance)]
pub fn guest_entrance(props: &GuestEntranceProps) -> Html {
    
    let room_id = props.room_id.clone();
    let on_join = props.on_join.clone();
    let add_member = Callback::from(move |name| {
        let room_id_cloned = room_id.clone();
        let user_id = add_document(
            &room_id,
            &MemberInput {
                name,
                is_host: false,
            },
            |_| {},
            || {}
        );
        crate::storage::save_user_id(room_id_cloned.as_str(),user_id.as_str());
        on_join.emit(user_id);
    });
    let host = use_collection::<MemberJSON>(&props.room_id)
        .map(|memebers| {
            memebers.into_iter().find(|m| m.is_host).unwrap()
        });

    let room_state = use_document_sync::<Room>(&(), &props.room_id);
    let state = room_state.merge(host);

    match state {
        DataFetchState::Loading => loading(),
        DataFetchState::Loaded((room,host)) => {
            if room.can_join {
                html! {
                    <>
                        {title()}
                        <section class="mx-auto w-full max-w-2xl">
                            <Heading2>{ format!("「{}」の部屋に入る",host.name)}</Heading2>
                            <InputAndButton label="参加" placeholder="あなたの名前" onsubmit={add_member} />  
                        </section>
                    </>
                }
            }  else {
                html! {
                    <div>
                        {title()}
                        <p class="mx-auto w-full max-w-2xl text-center text-black">
                            {"この部屋は参加を締め切られました。"}
                        </p>
                    </div>
                }
            }
        },
    }
}
