use std::iter::repeat;

use firestore::{Roll, Rule, MemberJSON, UserToRole};
use rand::seq::SliceRandom;

pub fn create_next_roll(rule: &Rule,members: &Vec<MemberJSON>,rolls: &Vec<Roll>) -> Roll {
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