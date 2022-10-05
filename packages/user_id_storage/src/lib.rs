fn get_user_id_from_localstorage(room_id: &str) -> Option<String> {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();
    local_storage.get_item(create_key(room_id).as_str()).unwrap()
}

pub fn get_user_id(room_id: &str) -> Option<String> {
    get_user_id_from_localstorage(room_id)
}


fn create_key(room_id: &str) -> String {
    format!("{}:user_id", room_id)
}


fn save_user_id_to_localstorage(room_id: &str,user_id: &str) {
    let window = web_sys::window().unwrap();
    let local_storage = window.local_storage().unwrap().unwrap();
    local_storage.set_item(create_key(room_id).as_str(),user_id).unwrap();
}

pub fn save_user_id(room_id: &str,user_id: &str) {
    save_user_id_to_localstorage(room_id,user_id);
}

