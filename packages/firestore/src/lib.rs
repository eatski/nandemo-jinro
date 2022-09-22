
use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use serde::{Serialize, Deserialize};
use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsValue};
use web_sys::console;

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(js_name = "addMembers",js_namespace = ["window","_wasm_js_bridge"])]
    fn addMembersInner(room_id: &str, name: &str, on_complete: &JsValue, on_error: &JsValue) -> String;
    #[wasm_bindgen(js_name = "addRoom",js_namespace = ["window","_wasm_js_bridge"])]
    fn addRoomInner(on_complete: &JsValue);
    #[wasm_bindgen(js_name = "syncCollection",js_namespace = ["window","_wasm_js_bridge"])]
    fn syncCollectionInner(path: &str,callback: JsValue,on_error: JsValue) -> Function;
}

fn sync_collection_json(path: &str,callback:impl FnMut(String) + 'static , on_error: impl FnMut() + 'static) -> impl FnOnce() {
    let callback : Box<dyn FnMut(String)> = Box::new(callback);
    let on_error = Closure::wrap(Box::new(on_error) as Box<dyn FnMut()>);
    let callback = Closure::wrap(callback).into_js_value();
    let cleanup = syncCollectionInner(path,callback,on_error.into_js_value());
    move || {
        cleanup.call0(&JsValue::NULL).unwrap();
    }
}

const NAME_SPACE: &str = "rollrole/v1";

pub struct MemberInput {
    pub name: String,
}

pub fn add_members(room_id: &str,member: &MemberInput, on_complete: impl FnOnce() + 'static, on_error: impl FnOnce() + 'static) -> String {
    addMembersInner(room_id,&member.name,&Closure::once_into_js(on_complete),&Closure::once_into_js(on_error))
}

#[derive(Serialize, Deserialize)]
pub struct MemberJSON {
    pub name: String,
    pub id: String,
}

fn json_to_members(json:&str) -> Result<Vec<MemberJSON>,String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

pub fn sync_members(room_id: &str,mut callback:impl FnMut(Vec<MemberJSON>)  + 'static , on_error: impl FnMut() + 'static) -> impl FnOnce() {
    let on_error = Rc::new(RefCell::new(Box::new(on_error) as Box<dyn FnMut()>));
    let on_parse_error = on_error.clone();
    let callback : Box<dyn FnMut(String)>= Box::new(
        move |json:String| {
            match json_to_members(json.as_str()) {
                Ok(members) => callback(members),
                Err(e) => {
                    console::log_1(&e.into());
                    on_parse_error.borrow_mut()();
                },
            } 
        }
    );
    let on_error = move || on_error.borrow_mut()();
    sync_collection_json(
        &format!("{}/rooms/{}/members",NAME_SPACE,room_id),
        callback,
        on_error
    )
}

pub fn add_room<CB: FnOnce(&str) + 'static>(on_complete: CB) {
    addRoomInner(&Closure::once_into_js (move |val: JsValue| {
        on_complete(val.as_string().unwrap().as_str());
    }));
}