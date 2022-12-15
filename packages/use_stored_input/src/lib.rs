use yew::{hook};

#[hook]
fn use_stored_string_inner<T: Clone + Eq + 'static>(key: &str, to_str: impl Fn(String) -> Option<T> + 'static, from_str: impl Fn(&T) -> String + 'static) -> (Option<T>,yew::Callback<T>){
    let get_local_storage = || {
        let window = web_sys::window()?;
        window.local_storage().ok()?
    };
    let input_state = yew::use_state(|| {
        let local_storage = get_local_storage()?;
        let stored = local_storage.get_item(key).ok()?;
        to_str(stored?)
    });
    let key = key.to_owned();
    yew::use_effect_with_deps(move |state| {
        if let Some(state) = &**state {
            let local_storage = get_local_storage().unwrap();
            local_storage.set_item(key.as_str(),from_str(state).as_str()).unwrap();
        }
        || {}
    }, input_state.clone());
    let state = &*input_state;
    (state.clone(),yew::Callback::from(move |input| input_state.set(Some(input))))
}

#[hook]
pub fn use_stored_string(key: &str) -> (Option<String>,yew::Callback<String>){
    use_stored_string_inner(key, Some, ToOwned::to_owned)
 }

#[hook]
pub fn use_stored_model<M: Clone + Eq + serde::Serialize + serde::de::DeserializeOwned + 'static >(key: &str) -> (Option<M>,yew::Callback<M>){
    let (state,set_state) = use_stored_string_inner(key,|state| serde_json::from_str(state.as_str()).ok(), |state| serde_json::to_string(state).unwrap());
    (state,set_state)
}