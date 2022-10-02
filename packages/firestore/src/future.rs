use std::{cell::RefCell, rc::Rc};

use serde::de::DeserializeOwned;
use web_sys::console;

use crate::js_bridge::sync_collection_json;
pub trait FireStoreResource where Self: DeserializeOwned {
    type ParamForPath;
    fn path(param: &Self::ParamForPath) -> String;
}

pub fn sync_collection<T>(param: &T::ParamForPath,mut on_change: impl FnMut(Vec<T>) + 'static, on_error: impl FnMut() + 'static) -> impl FnOnce() where T: FireStoreResource {
    let on_error = Rc::new(RefCell::new(Box::new(on_error) as Box<dyn FnMut()>));
    let on_parse_error = on_error.clone();
    let callback = move |json:String| {
        match serde_json::from_str(json.as_str()) {
            Ok(t) => on_change(t),
            Err(e) => {
                console::log_1(&e.to_string().into());
                on_parse_error.borrow_mut()();
            },
        } 
    };
    let on_error = move || on_error.borrow_mut()();
    sync_collection_json(T::path(param).as_str(), callback, on_error)
}

pub fn get_collection<T>(param: &T::ParamForPath, on_complete: impl FnOnce(Vec<T>) + 'static, on_error: impl FnMut() + 'static) where T: FireStoreResource {
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
    crate::js_bridge::get_collection_json(T::path(param).as_str(), callback, on_error)
}

pub fn sync_document<T>(param: &T::ParamForPath, document_id: &str,mut on_change: impl FnMut(T) + 'static, on_error: impl FnMut() + 'static) -> impl FnOnce() where T: FireStoreResource {
    let on_error = Rc::new(RefCell::new(Box::new(on_error) as Box<dyn FnMut()>));
    let on_parse_error = on_error.clone();
    let callback = move |json:String| {
        match serde_json::from_str(json.as_str()) {
            Ok(t) => on_change(t),
            Err(e) => {
                console::log_1(&e.to_string().into());
                on_parse_error.borrow_mut()();
            },
        } 
    };
    let on_error = move || on_error.borrow_mut()();
    crate::js_bridge::sync_document_json((T::path(param) + "/" + document_id).as_str(), callback, on_error)
}
