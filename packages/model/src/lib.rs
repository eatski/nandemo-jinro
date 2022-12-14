use firestore::FireStoreResource;
use historical::{HistoricalSignature, HistricalItem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct MemberInput {
    pub name: String,
    pub is_host: bool,
}

impl FireStoreResource for MemberInput {
    type ParamForPath = String;

    fn path(param: &Self::ParamForPath) -> String {
        format!("{}/rooms/{}/members", NAME_SPACE, param)
    }
}

pub type UserToRole = HashMap<String, String>;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Roll {
    pub seq_num: usize,
    pub user_to_role: UserToRole,
}

impl FireStoreResource for Roll {
    fn path(room_id: &String) -> String {
        format!("{}/rooms/{}/rolls", NAME_SPACE, room_id)
    }
    type ParamForPath = String;
}

#[derive(Clone, PartialEq, Eq,Debug)]
pub struct Room {
    pub rule: Option<Rule>,
    pub can_join: bool,
}

impl Default for Room {
    fn default() -> Self {
        Self {
            rule: None,
            can_join: true,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq,Debug)]

pub struct RoomEditAction {
    pub signature: HistoricalSignature,
    pub body: RoomEditBody,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq,Debug)]
pub enum RoomEditBody {
    SetCanJoin(bool),
    SetRule(Rule),
}

impl HistricalItem for RoomEditAction {
    type Collected = Room;

    fn signature(&self) -> HistoricalSignature {
        self.signature.clone()
    }

    fn apply(self,acc: &mut Self::Collected) {
        match self.body {
            RoomEditBody::SetCanJoin(can_join) => acc.can_join = can_join,
            RoomEditBody::SetRule(rule) => acc.rule = Some(rule),
        }
    }
}

impl FireStoreResource for RoomEditAction {
    fn path(room_id: &String) -> String {
        format!("{}/rooms/{}/edit", NAME_SPACE, room_id)
    }
    type ParamForPath = String;
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq,Debug)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub number: usize,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq,Debug)]
pub struct Rule {
    pub roles: Vec<Role>,
}

const NAME_SPACE: &str = "rollrole/v1";
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct MemberJSON {
    pub name: String,
    pub id: String,
    pub is_host: bool,
}

impl FireStoreResource for MemberJSON {
    fn path(room_id: &String) -> String {
        format!("{}/rooms/{}/members", NAME_SPACE, room_id)
    }
    type ParamForPath = String;
}
