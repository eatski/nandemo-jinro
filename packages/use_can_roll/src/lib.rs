use firestore_hooks::{DataFetchState, use_collection_sync};
use model::MemberJSON;
use yew::hook;
use use_historical::use_historical_read;


pub enum ValidationError {
    NotEnoughMembers,
    NotEnoughRoles,
    NoRules,
    RoomOpen
}

#[hook]
pub fn use_can_roll_validation(room_id: &str) -> DataFetchState<Vec<ValidationError>>{
    let members = use_collection_sync::<MemberJSON>(&room_id.to_owned());
    let room = use_historical_read::<model::RoomEditAction>(room_id.to_owned());
    room.merge(members).map(|(room,member)| {
        let room = room.latest;
        let mut errors = Vec::with_capacity(4);
        if member.len() < 2 {
            errors.push(ValidationError::NotEnoughMembers);
        }
        match room.rule {
            None => errors.push(ValidationError::NoRules),
            Some(rule) => {
                if rule.roles.iter().map(|role| role.number).sum::<usize>() < member.len() {
                    errors.push(ValidationError::NotEnoughRoles);
                }
            }
        }
        if room.can_join {
            errors.push(ValidationError::RoomOpen);
        }
        errors
    })
}