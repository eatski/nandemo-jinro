
use std::{collections::HashMap};
use future::{FireStoreResource};
use serde::{Serialize, Deserialize};

mod js_bridge;
pub mod json_bridge;
pub mod future;

#[derive(Serialize, Deserialize)]
pub struct MemberInput {
    pub name: String,
    pub is_host: bool,
}

impl FireStoreResource for MemberInput {
    type ParamForPath = String;

    fn path(param: &Self::ParamForPath) -> String {
        format!("{}/rooms/{}/members",NAME_SPACE,param)
    }
}

pub type UserToRole = HashMap<String,String>;

#[derive(Serialize, Deserialize,Clone)]
pub struct Roll {
    pub seq_num: usize,
    pub user_to_role: UserToRole,
}

impl FireStoreResource for Roll {
    fn path(room_id: &String) -> String {
        format!("{}/rooms/{}/rolls",NAME_SPACE,room_id)
    }
    type ParamForPath = String;
}

#[derive(Serialize, Deserialize,Clone)]
pub struct Room {
    pub rule: Option<Rule>,
    pub can_join: bool,
}

impl FireStoreResource for Room {
    fn path(_: &()) -> String {
        format!("{}/rooms",NAME_SPACE)
    }
    type ParamForPath = ();
}

#[derive(Serialize, Deserialize,Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub number: usize
}
#[derive(Serialize, Deserialize,Clone)]
pub struct Rule {
    pub roles: Vec<Role>,
}

const NAME_SPACE: &str = "rollrole/v1";
#[derive(Serialize, Deserialize,Clone)]
pub struct MemberJSON {
    pub name: String,
    pub id: String,
    pub is_host: bool,
}

impl FireStoreResource for MemberJSON {
    fn path(room_id: &String) -> String {
        format!("{}/rooms/{}/members",NAME_SPACE,room_id)
    }
    type ParamForPath = String;
}

#[derive(Serialize, Deserialize,Clone)]
pub struct SetRule {
    pub rule: Rule,
}

impl FireStoreResource for SetRule {
    fn path(_: &()) -> String {
        format!("{}/rooms",NAME_SPACE)
    }
    type ParamForPath = ();
}

#[derive(Serialize, Deserialize,Clone)]
pub struct SetCanJoin {
    pub can_join: bool,
}

impl FireStoreResource for SetCanJoin {
    fn path(_: &()) -> String {
        format!("{}/rooms",NAME_SPACE)
    }
    type ParamForPath = ();
}