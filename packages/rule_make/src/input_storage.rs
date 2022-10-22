use serde::{Serialize, Deserialize};
use yew::{use_state, UseStateHandle, use_effect_with_deps};


#[derive(Clone,Serialize,Deserialize,PartialEq,Debug)]
pub struct Item {
    pub name: String,
    pub count: usize,
}

pub type Input = Vec<Item>;

fn create_key(room_id: &str) -> String {
    format!("{}:rule_make_input", room_id)
}

fn save_input_storage(room_id: &str, input: &Input) {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();
    local_storage
        .set_item(create_key(room_id).as_str(), &serde_json::to_string(input).unwrap())
        .unwrap();
}

fn get_input_storage(room_id: &str) -> Option<Input> {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();
    local_storage
        .get_item(create_key(room_id).as_str())
        .unwrap()
        .and_then(|json| serde_json::from_str(json.as_str()).ok())
}

pub fn use_input(room_id: &str,default: Input)->UseStateHandle<Input>{
    let saved_input = get_input_storage(room_id);
    let input_state = use_state(|| saved_input.unwrap_or(default));

    use_effect_with_deps(|(room_id,state)| {
        save_input_storage(room_id, &**state);
        || {}
    }, (room_id.to_owned(),input_state.clone()));
    input_state
}