use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::{prelude::Closure, JsValue};
use yew::{use_state, use_effect};

fn get_history_state<T: Serialize + DeserializeOwned>() -> Option<T> {
    let window = web_sys::window().unwrap();
    let history = window.history().unwrap();
    let state = history.state().unwrap();
    if state.is_null() {
        None
    } else {
        let json = state.as_string().unwrap();
        Some(serde_json::from_str(json.as_str()).unwrap())
    }
}

pub fn use_history_state<T : DeserializeOwned + Serialize + Eq + Clone + 'static>() -> (Option<T> , impl Fn(T)) {
    let state = use_state(get_history_state);
    {
        let state = state.clone();
        use_effect(|| {
            let window = web_sys::window().unwrap();
            let callback : Box<dyn FnMut()> = Box::new(move || {
                state.set(get_history_state());
            });
            let callback = Closure::wrap( callback).into_js_value().into();
            window.add_event_listener_with_callback("popstate", &callback).unwrap();
            move || {
                window.remove_event_listener_with_callback("popstate", &callback).unwrap();
            }
        });
    }
    (
        (&*state).clone(),
        move |next| {
            let window = web_sys::window().unwrap();
            let history = window.history().unwrap();
            let next = Some(next);
            history.push_state(&JsValue::from_str(serde_json::to_string(&next).unwrap().as_str()), "").unwrap();
            state.set(next);
        }
    )
}