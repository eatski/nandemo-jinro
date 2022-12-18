use yew::{hook};

#[hook]
pub fn use_stored_input(key: &str) -> (Option<String>,yew::Callback<String>){
    let get_local_storage = || {
        let window = web_sys::window()?;
        window.local_storage().ok()?
    };
    let input_state = yew::use_state(|| {
        let local_storage = get_local_storage()?;
        local_storage.get_item(key).ok()?
    });
    let key = key.to_owned();
    yew::use_effect_with_deps(move |state| {
        if let Some(state) = &**state {
            let local_storage = get_local_storage().unwrap();
            local_storage.set_item(key.as_str(),&state).unwrap();
        }
        || {}
    }, input_state.clone());
    let state = &*input_state;
    (state.clone(),yew::Callback::from (move |input| {
        input_state.set(Some(input));
    }))
}