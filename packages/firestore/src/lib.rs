
use std::{collections::HashMap};
use future::{FireStoreResource};
use json_bridge::{sync_collection, sync_document, get_collection, get_document, set_document_field, add_document};
use serde::{Serialize, Deserialize};

mod js_bridge;
pub mod json_bridge;
pub mod future;

#[derive(Serialize, Deserialize)]
pub struct MemberInput {
    pub name: String,
    pub is_host: bool,
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

pub fn add_members(room_id: &str,member: &MemberInput, on_complete: impl FnOnce() + 'static, on_error: impl FnOnce() + 'static) -> String {
    let path: &str = &format!("{}/rooms/{}/members",NAME_SPACE,room_id);
    add_document(path,member,|_| on_complete(),on_error)
}

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

pub fn sync_members(room_id: &str,callback:impl FnMut(Vec<MemberJSON>)  + 'static , on_error: impl FnMut() + 'static) -> impl FnOnce() {
    crate::future::sync_collection(
        &room_id.to_string(),
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
    add_document(path,room,on_complete,|| {})
}

pub fn set_rule(room_id: &str,rule: &Rule, on_complete: impl FnOnce() + 'static, on_error: impl FnOnce() + 'static) {
    let path: &str = &format!("{}/rooms/{}",NAME_SPACE,room_id);
    set_document_field(path,"rule",rule,on_complete,on_error);
}

pub fn set_can_join_false(room_id: &str,on_complete: impl FnOnce() + 'static, on_error: impl FnOnce() + 'static) {
    let path: &str = &format!("{}/rooms/{}",NAME_SPACE,room_id);
    set_document_field(path,"can_join",&false,on_complete,on_error);
}

pub fn sync_room(room_id: &str,callback: impl FnMut(Room) + 'static, on_error: impl FnMut() + 'static) -> impl FnOnce() {
    sync_document(
        &format!("{}/rooms/{}",NAME_SPACE,room_id),
        callback,
        on_error
    )
}

pub fn add_roll(room_id: &str,roll: &Roll,on_complete: impl FnOnce() + 'static) -> String {
    let path: &str = &format!("{}/rooms/{}/rolls",NAME_SPACE,room_id);
    add_document(path,roll,|_| on_complete(),|| {})
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