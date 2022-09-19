
use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use serde::{Serialize, Deserialize};
use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsValue};
use web_sys::console;

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(js_name = "addMembers",js_namespace = ["window","_wasm_js_bridge"])]
    fn addMembersInner(room_id: &str);
    #[wasm_bindgen(js_name = "syncMembers",js_namespace = ["window","_wasm_js_bridge"])]
    fn syncMembersInner(room_id: &str,callback: JsValue,on_error: JsValue) -> Function;
}


pub fn add_members(room_id: &str) {
    addMembersInner(room_id);
}

#[derive(Serialize, Deserialize)]
pub struct MemberJSON {
    pub name: String,
    pub id: String,
}

fn json_to_members(json:&str) -> Result<Vec<MemberJSON>,String> {
    serde_json::from_str(json).map_err(|e| e.to_string())
}

pub fn sync_members<'a,CB: FnMut(Vec<MemberJSON>)  + 'static,OE: FnMut() + 'static>(room_id: &str,mut callback: CB, on_error: OE) -> Box<dyn FnOnce()> {
    let on_error = Rc::new(RefCell::new(Box::new(on_error) as Box<dyn FnMut()>));
    let on_parse_error = on_error.clone();
    let json_callback : Box<dyn FnMut(String)>= Box::new(
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
    let on_error = Closure::wrap(Box::new(move || on_error.borrow_mut()()) as Box<dyn FnMut()>);
    let callback = Closure::wrap(json_callback).into_js_value();
    let cleanup = syncMembersInner(room_id,callback,on_error.into_js_value());
    Box::new(move || {
        cleanup.call0(&JsValue::NULL).unwrap();
    })
}