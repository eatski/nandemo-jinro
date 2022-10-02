use std::{iter::repeat};
use model::{Roll, Rule, MemberJSON, UserToRole,Room};
use rand::seq::SliceRandom;
use yew::{use_effect_with_deps, use_state};

use super::firestore::{DataFetchState, use_collection, use_document};

fn create_next_roll(rule: &Rule,members: &Vec<MemberJSON>,rolls: &Vec<Roll>) -> Roll {
    let mut roles: Vec<_> = rule.roles.iter().cloned()
                    .flat_map(|role_input| repeat(role_input.id).take(role_input.number)).collect();
    let mut rng = rand::thread_rng();
    roles.shuffle(&mut rng);
    let user_to_role: UserToRole = members
        .iter()
        .map(|member| (member.id.clone(), roles.pop().expect("Not enough roles")))
        .collect();
    let seq_num = rolls.len();
    Roll { user_to_role,seq_num }
}

#[derive(Eq,PartialEq)]
enum ButtonState {
    Loading,
    NotClicked,
    Clicked,
}
pub fn use_roll(room_id: &str) -> Option<impl Fn()> {
    let clicked = use_state(|| ButtonState::NotClicked);
    let room  = use_document::<Room>(&(),room_id);
    let members = use_collection::<MemberJSON>(&room_id.to_string());
    let rolls = use_collection::<Roll>(&room_id.to_string());
    {
        let clicked = clicked.clone();
        let room_id = room_id.to_string();
        use_effect_with_deps(move |(room,members,rolls,clicked)| {
            let merged = room.clone().merge(members.clone()).merge(rolls.clone());
            match merged {
                DataFetchState::Loaded(((room,members),rolls)) => {
                    if matches!(*clicked.clone(), ButtonState::Clicked)  {
                        let next_roll = create_next_roll(&room.rule.unwrap(),&members,&rolls);
                        clicked.set(ButtonState::Loading);
                        let clicked = clicked.clone();
                        firestore::add_document(
                            &room_id,
                            &next_roll,
                            move |_| {
                                clicked.set(ButtonState::NotClicked);
                            },
                            || {}
                        );
                    }
                }
                _ => {}
            }
            || {
    
            }
        }, (room,members,rolls,clicked));
    }
    if matches!(*clicked, ButtonState::NotClicked) {
        Some(move || {
            clicked.set(ButtonState::Clicked);
        })
    } else {
        None
    }
    
}