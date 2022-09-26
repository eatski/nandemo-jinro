use firestore::MemberJSON;
use yew::{use_state, use_effect_with_deps};

#[derive(Clone)]
pub enum MemberState {
    Loading,
    Loaded(MemberJSON)
} 

pub fn use_member(room_id: &str,user_id: &str) -> MemberState {
    let state = use_state(|| MemberState::Loading);
    let state_cloned = state.clone();
    let user_id = user_id.to_string();
    let room_id = room_id.to_string();
    use_effect_with_deps(
        |(room_id,user_id)| {
            firestore::get_member(
                room_id,
                user_id,
                move |member| {
                    state.set(MemberState::Loaded(member))
                },
                || {},
            );
            || {}
        },
        (room_id,user_id),
    );
    (*state_cloned).clone()
} 