
use std::{collections::HashMap};
use future::{sync_collection, sync_document, get_collection, get_document};
use serde::{Serialize, Deserialize};

use bridge::{add_document, set_field};
mod bridge;
pub mod future;


const NAME_SPACE: &str = "rollrole/v1";

#[derive(Serialize, Deserialize)]
pub struct MemberInput {
    pub name: String,
    pub is_host: bool,
}

pub fn add_members(room_id: &str,member: &MemberInput, on_complete: impl FnOnce() + 'static, on_error: impl FnOnce() + 'static) -> String {
    let path: &str = &format!("{}/rooms/{}/members",NAME_SPACE,room_id);
    let json: &str = &serde_json::to_string(member).expect("Failed to serialize member");
    add_document(path,json,|_| on_complete(),on_error)
}

#[derive(Serialize, Deserialize,Clone)]
pub struct MemberJSON {
    pub name: String,
    pub id: String,
    pub is_host: bool,
}

fn json_to_member(json:&str) -> Result<MemberJSON,String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

pub fn sync_members(room_id: &str,callback:impl FnMut(Vec<MemberJSON>)  + 'static , on_error: impl FnMut() + 'static) -> impl FnOnce() {
    sync_collection(
        &format!("{}/rooms/{}/members",NAME_SPACE,room_id),
        callback,
        on_error
    )
}

pub fn get_members(room_id: &str,on_complete: impl FnOnce(Vec<MemberJSON>) + 'static, on_error: impl FnMut() + 'static) {
    get_collection(
        &format!("{}/rooms/{}/members",NAME_SPACE,room_id),
        on_complete,
        on_error
    )
}

pub fn get_member(room_id: &str,member_id: &str,on_complete: impl FnOnce(MemberJSON) + 'static, on_error: impl FnMut() + 'static) {
    get_document(
        &format!("{}/rooms/{}/members/{}",NAME_SPACE,room_id,member_id),
        on_complete,
        on_error
    );
}

pub fn add_room(room: &Room,on_complete: impl FnOnce(&str) + 'static) -> String {
    let path: &str = &format!("{}/rooms",NAME_SPACE);
    let json: &str = &serde_json::to_string(room).expect("Failed to serialize room");
    add_document(path,json,on_complete,|| {})
}

pub fn set_rule(room_id: &str,rule: &Rule, on_complete: impl FnOnce() + 'static, on_error: impl FnOnce() + 'static) {
    let path: &str = &format!("{}/rooms/{}",NAME_SPACE,room_id);
    let json = serde_json::to_string(rule).expect("Failed to serialize rule");
    set_field(path,"rule",json.as_str(),on_complete,on_error);
}

pub fn set_can_join_false(room_id: &str,on_complete: impl FnOnce() + 'static, on_error: impl FnOnce() + 'static) {
    let path: &str = &format!("{}/rooms/{}",NAME_SPACE,room_id);
    set_field(path,"can_join","false",on_complete,on_error);
}

pub fn sync_room(room_id: &str,callback: impl FnMut(Room) + 'static, on_error: impl FnMut() + 'static) -> impl FnOnce() {
    sync_document(
        &format!("{}/rooms/{}",NAME_SPACE,room_id),
        callback,
        on_error
    )
}

pub fn add_roll(room_id: &str,roll: Roll,on_complete: impl FnOnce() + 'static) -> String {
    let path: &str = &format!("{}/rooms/{}/rolls",NAME_SPACE,room_id);
    let json: &str = &serde_json::to_string(&roll).expect("Failed to serialize rolls");
    add_document(path,json,|_| on_complete(),|| {})
}

pub fn sync_rolls(room_id: &str,callback: impl FnMut(Vec<Roll>) + 'static, on_error: impl FnMut() + 'static) -> impl FnOnce() {
    sync_collection(
        &format!("{}/rooms/{}/rolls",NAME_SPACE,room_id),
        callback,
        on_error
    )
}

pub fn get_rolls(room_id: &str,on_complete: impl FnOnce(Vec<Roll>) + 'static, on_error: impl FnMut() + 'static) {
    get_collection(
        &format!("{}/rooms/{}/rolls",NAME_SPACE,room_id),
        on_complete,
        on_error
    )
}

pub type UserToRole = HashMap<String,String>;

#[derive(Serialize, Deserialize,Clone)]
pub struct Roll {
    pub seq_num: usize,
    pub user_to_role: UserToRole,
}


#[derive(Serialize, Deserialize,Clone)]
pub struct Room {
    pub rule: Option<Rule>,
    pub can_join: bool,
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