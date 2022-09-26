
use std::{cell::RefCell, rc::Rc};
use serde::{Serialize, Deserialize};

use bridge::{add_document, get_document_json, set_field, sync_collection_json, get_collection_json, sync_document_json};
use web_sys::console;
mod bridge;


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

fn json_to_members(json:&str) -> Result<Vec<MemberJSON>,String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

fn json_to_member(json:&str) -> Result<MemberJSON,String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

pub fn sync_members(room_id: &str,mut callback:impl FnMut(Vec<MemberJSON>)  + 'static , on_error: impl FnMut() + 'static) -> impl FnOnce() {
    let on_error = Rc::new(RefCell::new(Box::new(on_error) as Box<dyn FnMut()>));
    let on_parse_error = on_error.clone();
    let callback = move |json:String| {
        match json_to_members(json.as_str()) {
            Ok(members) => callback(members),
            Err(e) => {
                console::log_1(&e.into());
                on_parse_error.borrow_mut()();
            },
        } 
    };
    let on_error = move || on_error.borrow_mut()();
    sync_collection_json(
        &format!("{}/rooms/{}/members",NAME_SPACE,room_id),
        callback,
        on_error
    )
}

pub fn get_members(room_id: &str,on_complete: impl FnOnce(Vec<MemberJSON>) + 'static, on_error: impl FnMut() + 'static) {
    let on_error = Rc::new(RefCell::new(Box::new(on_error) as Box<dyn FnMut()>));
    let on_parse_error = on_error.clone();
    let callback = move |json:&str| {
        match json_to_members(json) {
            Ok(members) => on_complete(members),
            Err(e) => {
                console::log_1(&e.into());
                on_parse_error.borrow_mut()();
            },
        } 
    };
    let on_error = move || on_error.borrow_mut()();
    get_collection_json(
        &format!("{}/rooms/{}/members",NAME_SPACE,room_id),
        callback,
        on_error
    )
}

pub fn get_member(room_id: &str,member_id: &str,on_complete: impl FnOnce(MemberJSON) + 'static, on_error: impl FnMut() + 'static) {
    let on_error = Rc::new(RefCell::new(Box::new(on_error) as Box<dyn FnMut()>));
    let on_parse_error = on_error.clone();
    let callback = move |json:&str| {
        match json_to_member(json) {
            Ok(member) => {
                on_complete(member)
            },
            Err(e) => {
                console::log_1(&e.into());
                on_parse_error.borrow_mut()();
            },
        } 
    };
    let on_error = move || on_error.borrow_mut()();
    get_document_json(
        &format!("{}/rooms/{}/members/{}",NAME_SPACE,room_id,member_id),
        callback,
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

pub fn sync_room(room_id: &str,mut callback: impl FnMut(Room) + 'static, on_error: impl FnMut() + 'static) -> impl FnOnce() {
    let on_error = Rc::new(RefCell::new(Box::new(on_error) as Box<dyn FnMut()>));
    let on_parse_error = on_error.clone();
    let callback = move |json:String| {
        match serde_json::from_str(json.as_str()) {
            Ok(room) => callback(room),
            Err(e) => {
                console::log_1(&e.to_string().into());
                on_parse_error.borrow_mut()();
            },
        } 
    };
    let on_error = move || on_error.borrow_mut()();
    sync_document_json(
        &format!("{}/rooms/{}",NAME_SPACE,room_id),
        callback,
        on_error
    )
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
    pub number: u32
}

#[derive(Serialize, Deserialize,Clone)]
pub struct Rule {
    pub roles: Vec<Role>,
}