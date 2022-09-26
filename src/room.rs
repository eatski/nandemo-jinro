use presentational::loading;
use yew::{function_component, html, Properties, use_state_eq, Callback};
use crate::entrance::{GuestEntrance};

use crate::state_hooks::{use_member, MemberState};
use crate::{storage::{get_user_id}};
use crate::lobby::Lobby;
use crate::rule_make::RuleMake;

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
    user_id: String
}

#[function_component(HasUserId)]
fn view_when_has_userid(props: &HasUserIdProps) -> Html {
    let member = use_member(props.room_id.as_str(),props.user_id.as_str());
    html! {
        <>
            <Lobby room_id={props.room_id.clone()} user_id={props.user_id.clone()}/>
            {
                match member {
                    MemberState::Loading => html!(),
                    MemberState::Loaded(member) => {
                        if member.is_host {
                            html! {
                                <RuleMake room_id={props.room_id.clone()}/>
                            }
                            
                        } else {
                            html! {}
                        }
                    }
                }
            }
        </>
    }
}
