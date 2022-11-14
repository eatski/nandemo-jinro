use atoms::{loading, unexpected_error};
use model::{MemberJSON, Roll};
use yew::{function_component, html, use_state_eq, Callback, Properties};

use landing::entrance::GuestEntrance;

use firestore_hooks::{use_collection_sync, use_document, DataFetchState};
use lobby::Lobby;
use rule_make::RuleMake;
use user_id_storage::get_user_id;

use roll::roll::RollContainer;
use roll::rolled::Rolled;

use use_historical::{use_historical_read};

#[derive(Properties, PartialEq)]
pub struct RoomProps {
    pub room_id: String,
}

#[function_component(Room)]
pub fn room(props: &RoomProps) -> Html {
    // NOTE: ルームIDが変わる場合のことを考慮できていない
    let user_id_state = use_state_eq(|| get_user_id(props.room_id.as_str()));
    let user_id_setter = user_id_state.setter();
    let set_user_id = Callback::once(move |user_id| {
        user_id_setter.set(Some(user_id));
    });
    if let Some(user_id) = &*user_id_state {
        html! {
            <HasUserId room_id={props.room_id.clone()} user_id={user_id.clone()}/>
        }
    } else {
        html! {
            <GuestEntrance room_id={props.room_id.clone()} on_join={set_user_id} />
        }
    }
    
}

#[derive(Properties, PartialEq)]
struct HasUserIdProps {
    room_id: String,
    user_id: String,
}

#[function_component(HasUserId)]
fn view_when_has_userid(props: &HasUserIdProps) -> Html {
    let member = use_document::<MemberJSON>(&props.room_id, props.user_id.as_str());
    let room = use_historical_read::<model::RoomEditAction>(props.room_id.clone());
    let roles = use_collection_sync::<Roll>(&props.room_id);
    let merged = room.merge(member).merge(roles);
    match merged {
        DataFetchState::Loading => loading(),
        DataFetchState::Loaded(((room, member), rolls)) => {
            let rolled = rolls.len() > 0;
            if member.is_host {
                if room.latest.can_join {
                    html! {
                        <Lobby room_id={props.room_id.clone()} user_id={props.user_id.clone()}/>
                    }
                } else if room.latest.rule.is_none() {
                    html! {
                        <RuleMake room_id={props.room_id.clone()} />
                    }
                } else {
                    if rolled {
                        html! {
                            <Rolled room_id={props.room_id.clone()} user_id={props.user_id.clone()}/>
                        }
                    } else {
                        html! {
                            <RollContainer room_id={props.room_id.clone()} />
                        }
                    }
                }
            } else {
                if rolled {
                    html! {
                        <Rolled room_id={props.room_id.clone()} user_id={props.user_id.clone()} />
                    }
                } else {
                    html! {
                        <Lobby room_id={props.room_id.clone()} user_id={props.user_id.clone()}/>
                    }
                }
            }
        },
        DataFetchState::Error => unexpected_error()
    }
}
