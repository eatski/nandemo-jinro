

use js_sys::Function;
use wasm_bindgen::{prelude::{wasm_bindgen, Closure}, JsValue};

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(js_name = "syncCollection",js_namespace = ["window","_wasm_js_bridge"])]
    fn syncCollectionInner(path: &str,callback: JsValue,on_error: JsValue) -> Function;
    #[wasm_bindgen(js_name = "addDocument",js_namespace = ["window","_wasm_js_bridge"])]
    fn addDocument(path: &str, data: &str, on_complete: &JsValue, on_error: &JsValue) -> String;
    #[wasm_bindgen(js_name = "getCollection",js_namespace = ["window","_wasm_js_bridge"])]
    fn getCollection(path: &str, on_complete: &JsValue, on_error: &JsValue);
    #[wasm_bindgen(js_name = "setField",js_namespace = ["window","_wasm_js_bridge"])]
    fn setField(path: &str, field: &str, value: &str, on_complete: &JsValue, on_error: &JsValue);
    #[wasm_bindgen(js_name = "syncDocument",js_namespace = ["window","_wasm_js_bridge"])]
    fn syncDocument(path: &str, on_complete: &JsValue, on_error: &JsValue) -> Function;
    #[wasm_bindgen(js_name = "getDocument",js_namespace = ["window","_wasm_js_bridge"])]
    fn getDocument(path: &str, on_complete: &JsValue, on_error: &JsValue);
    #[wasm_bindgen(js_name = "setDocument",js_namespace = ["window","_wasm_js_bridge"])]
    fn setDocument(path: &str, data: &str, on_complete: &JsValue, on_error: &JsValue);
}

pub fn sync_collection_json(path: &str,callback:impl FnMut(String) + 'static , on_error: impl FnMut() + 'static) -> impl FnOnce() {
    let callback : Box<dyn FnMut(String)> = Box::new(callback);
    let on_error = Closure::wrap(Box::new(on_error) as Box<dyn FnMut()>);
    let callback = Closure::wrap(callback).into_js_value();
    let cleanup = syncCollectionInner(path,callback,on_error.into_js_value());
    move || {
        cleanup.call0(&JsValue::NULL).unwrap();
    }
}

pub fn add_document(path: &str, json: &str, on_complete: impl FnOnce(&str) + 'static, on_error: impl FnOnce() + 'static) -> String {
    let on_complete : JsValue = Closure::once_into_js(|val: JsValue| {
        on_complete(&val.as_string().unwrap());
    });
    let on_error : JsValue = Closure::once_into_js(on_error);
    addDocument(path,json,&on_complete,&on_error)
}

pub fn get_collection_json(path: &str,on_complete: impl FnOnce(&str) + 'static, on_error: impl FnOnce() + 'static)  {
    let on_complete : JsValue = Closure::once_into_js(|val: JsValue| {
        on_complete(&val.as_string().unwrap());
    });
    let on_error : JsValue = Closure::once_into_js(on_error);
    getCollection(path,&on_complete,&on_error)
}

pub fn set_field(path: &str, field: &str, value: &str, on_complete: impl FnOnce() + 'static, on_error: impl FnOnce() + 'static)  {
    let on_complete : JsValue = Closure::once_into_js(on_complete);
    let on_error : JsValue = Closure::once_into_js(on_error);
    setField(path,field,value,&on_complete,&on_error)
}

pub fn sync_document_json(path: &str, on_complete: impl FnMut(String) + 'static, on_error: impl FnOnce() + 'static) -> impl FnOnce() {
    let on_complete : Box<dyn FnMut(String)> = Box::new(on_complete);
    let on_error : JsValue = Closure::once_into_js(on_error);
    let on_complete = Closure::wrap(on_complete).into_js_value();
    let cleanup = syncDocument(path,&on_complete,&on_error);
    move || {
        cleanup.call0(&JsValue::NULL).unwrap();
    }
}

pub fn get_document_json(path: &str, on_complete: impl FnOnce(&str) + 'static, on_error: impl FnOnce() + 'static)  {
    let on_complete : JsValue = Closure::once_into_js(|val: JsValue| {
        on_complete(&val.as_string().unwrap());
    });
    let on_error : JsValue = Closure::once_into_js(on_error);
    getDocument(path,&on_complete,&on_error)
}

pub fn set_document_json(path: &str, json: &str, on_complete: impl FnOnce() + 'static, on_error: impl FnOnce() + 'static)  {
    let on_complete : JsValue = Closure::once_into_js(on_complete);
    let on_error : JsValue = Closure::once_into_js(on_error);
    setDocument(path,json,&on_complete,&on_error)
}