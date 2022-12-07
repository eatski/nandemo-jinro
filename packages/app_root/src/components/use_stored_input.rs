use yew::{UseStateHandle, hook};

#[hook]
pub fn use_stored_input(key: &str,default: &str) -> UseStateHandle<String>{
    let saved_input = {
        let window = web_sys::window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        local_storage
            .get_item(key)
            .unwrap()
    };
    let input_state = yew::use_state(|| saved_input.unwrap_or(default.to_owned()));
    let key = key.to_owned();
    yew::use_effect_with_deps(move |state| {
        let window = web_sys::window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        local_storage
            .set_item(key.as_str(), &**state)
            .unwrap();
        || {}
    }, input_state.clone());
    input_state
}