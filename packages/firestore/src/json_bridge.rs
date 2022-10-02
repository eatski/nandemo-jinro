use std::{cell::RefCell, rc::Rc};

use serde::{de::DeserializeOwned, Serialize};
use web_sys::console;

use crate::js_bridge::{sync_collection_json, sync_document_json};

pub fn sync_collection<T>(path: &str, mut on_change: impl FnMut(T) + 'static, on_error: impl FnMut() + 'static) -> impl FnOnce() where T: DeserializeOwned {
    let on_error = Rc::new(RefCell::new(Box::new(on_error) as Box<dyn FnMut()>));
    let on_parse_error = on_error.clone();
    let callback = move |json:String| {
        let result : Result<T,_> =  serde_json::from_str(json.as_str());
        match result {
            Ok(t) => on_change(t),
            Err(e) => {
                console::log_1(&e.to_string().into());
                on_parse_error.borrow_mut()();
            },
        } 
    };
    let on_error = move || on_error.borrow_mut()();
    sync_collection_json(path, callback, on_error)
}


pub fn sync_document<T>(path: &str, mut on_change: impl FnMut(T) + 'static, on_error: impl FnMut() + 'static) -> impl FnOnce() where T: DeserializeOwned {
    let on_error = Rc::new(RefCell::new(Box::new(on_error) as Box<dyn FnMut()>));
    let on_parse_error = on_error.clone();
    let callback = move |json:String| {
        let result : Result<T,_> =  serde_json::from_str(json.as_str());
        match result {
            Ok(t) => on_change(t),
            Err(e) => {
                console::log_1(&e.to_string().into());
                on_parse_error.borrow_mut()();
            },
        } 
    };
    let on_error = move || on_error.borrow_mut()();
    sync_document_json(path, callback, on_error)
}


pub fn get_document<T>(path: &str, on_complete: impl FnOnce(T) + 'static, on_error: impl FnMut() + 'static) where T: DeserializeOwned {
    let on_error = Rc::new(RefCell::new(Box::new(on_error) as Box<dyn FnMut()>));
    let on_parse_error = on_error.clone();
    let callback = move |json:&str| {
        let result : Result<T,_> =  serde_json::from_str(json);
        match result {
            Ok(t) => on_complete(t),
            Err(e) => {
                console::log_1(&e.to_string().into());
                on_parse_error.borrow_mut()();
            },
        } 
    };
    let on_error = move || on_error.borrow_mut()();
    crate::js_bridge::get_document_json(path, callback, on_error)
}

pub fn get_collection<T>(path: &str, on_complete: impl FnOnce(Vec<T>) + 'static, on_error: impl FnMut() + 'static) where T: DeserializeOwned {
    let on_error = Rc::new(RefCell::new(Box::new(on_error) as Box<dyn FnMut()>));
    let on_parse_error = on_error.clone();
    let callback = move |json:&str| {
        let result : Result<Vec<T>,_> =  serde_json::from_str(json);
        match result {
            Ok(t) => on_complete(t),
            Err(e) => {
                console::log_1(&e.to_string().into());
                on_parse_error.borrow_mut()();
            },
        } 
    };
    let on_error = move || on_error.borrow_mut()();
    crate::js_bridge::get_collection_json(path, callback, on_error)
}

pub fn add_document<T>(path: &str,document: &T,on_complete: impl FnOnce(&str) + 'static,on_error: impl FnOnce() + 'static) -> String where T : Serialize {
    let json = serde_json::to_string(document).unwrap();
    crate::js_bridge::add_document(path, json.as_str(),on_complete,on_error)
}

pub fn set_document_field<T>(path: &str,field: &str, value: &T,on_complete: impl FnOnce() + 'static,on_error: impl FnOnce() + 'static) where T : Serialize {
    let json = serde_json::to_string(value).unwrap();
    crate::js_bridge::set_field(path, field, json.as_str(),on_complete,on_error);
}