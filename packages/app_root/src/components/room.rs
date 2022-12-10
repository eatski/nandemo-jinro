use atoms::{loading, unexpected_error};
use model::{MemberJSON, Roll};
use serde::{Serialize, Deserialize};
use use_can_roll::{use_can_roll_validation,ValidationError};
use use_history_state::use_history_state;
use yew::{function_component, html, use_state_eq, Callback, Properties, Html};

use landing::entrance::GuestEntrance;

use firestore_hooks::{use_collection_sync, use_document, DataFetchState};
use lobby::Lobby;
use rule_make::RuleMake;
use user_id_storage::get_user_id;

use roll::roll::RollContainer;
use roll::rolled::Rolled;

use crate::components::room_host_navi::{RoomHostNavi, LinkStatus};

#[derive(Properties, PartialEq)]
pub struct RoomProps {
    pub room_id: String,
}

#[function_component(Room)]
pub fn room(props: &RoomProps) -> Html {
    // NOTE: ルームIDが変わる場合のことを考慮できていない
    let user_id_state = use_state_eq(|| get_user_id(props.room_id.as_str()));
    let user_id_setter = user_id_state.setter();
    let set_user_id = Callback::from(move |user_id| {
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

#[derive(Debug, Clone, PartialEq,Serialize, Deserialize,Eq)]
enum RoomHistoryState {
    Lobby,
    RuleMake,
    Confirm
}

#[function_component(HasUserId)]
fn view_when_has_userid(props: &HasUserIdProps) -> Html {
    let member = use_document::<MemberJSON>(&props.room_id, props.user_id.as_str());
    let validation = use_can_roll_validation(&props.room_id);
    let roles = use_collection_sync::<Roll>(&props.room_id);
    let merged = validation.merge(member).merge(roles);
    let (history_state,push) = use_history_state::<RoomHistoryState>();
    match merged {
        DataFetchState::Loading => loading(),
        DataFetchState::Loaded(((validation, member), rolls)) => {
            let rolled = rolls.len() > 0;
            if member.is_host {
                if rolled {
                    html! {
                        <Rolled room_id={props.room_id.clone()} user_id={props.user_id.clone()}/>
                    }
                } else {
                    let history_state = history_state.unwrap_or(RoomHistoryState::Lobby);
                    let content = match history_state {
                        RoomHistoryState::Lobby => {
                            html! {
                                <Lobby room_id={props.room_id.clone()} user_id={props.user_id.clone()} on_complete={push.reform(|_| RoomHistoryState::RuleMake)}/>
                            }
                        },
                        RoomHistoryState::RuleMake => {
                            let room_open = validation.iter().any(|error| matches!(error,ValidationError::RoomOpen));
                            html! {
                                <RuleMake 
                                    room_id={props.room_id.clone()} 
                                    on_complete={push.reform(move |_| if room_open { RoomHistoryState::Lobby} else { RoomHistoryState::Confirm })} 
                                />
                            }
                        },
                        RoomHistoryState::Confirm => {
                            html! {
                                <RollContainer room_id={props.room_id.clone()}  />
                            }
                        }
                    };
                    html! {
                        <>
                            <RoomHostNavi 
                                lobby={if history_state == RoomHistoryState::Lobby { LinkStatus::Current} else {LinkStatus::Clickable {
                                    onclick: push.reform(|_| RoomHistoryState::Lobby)
                                }}} 
                                make_rule={if history_state == RoomHistoryState::RuleMake { LinkStatus::Current} else {LinkStatus::Clickable {
                                    onclick: push.reform(|_| RoomHistoryState::RuleMake)
                                }}}
                                confirm={if history_state == RoomHistoryState::Confirm { LinkStatus::Current} else {
                                    if !validation.is_empty() {
                                        LinkStatus::Disabled
                                    } else {
                                        LinkStatus::Clickable {
                                            onclick: push.reform(|_| RoomHistoryState::Confirm)
                                        }
                                    }
                                }}
                            />
                            {content}
                        </>
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
