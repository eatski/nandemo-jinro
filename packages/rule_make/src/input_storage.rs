use serde::{Serialize, Deserialize};
use use_stored_input::use_stored_model;
use yew::{hook};

#[derive(Clone,Serialize,Deserialize,PartialEq,Debug,Eq)]
pub struct Item {
    pub name: String,
    pub count: usize,
}

pub type Input = Vec<Item>;

fn create_key(room_id: &str) -> String {
    format!("{}:rule_make_input", room_id)
}
#[hook]
pub fn use_input(room_id: &str)-> (Option<Input>, yew::Callback<Input>) {
    use_stored_model(create_key(room_id).as_str())
}