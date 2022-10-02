use firestore::MemberJSON;
use yew::{use_state, use_effect_with_deps};


#[derive(Clone)]
pub enum DataFetchState<R: Clone> {
    Loading,
    Loaded(R),
}

impl <T: Clone>DataFetchState<T> {
    pub fn merge<T2: Clone>(self,target: DataFetchState<T2>) -> DataFetchState<(T,T2)> {
        match (self,target) {
            (DataFetchState::Loading,DataFetchState::Loading) => DataFetchState::Loading,
            (DataFetchState::Loading,DataFetchState::Loaded(_)) => DataFetchState::Loading,
            (DataFetchState::Loaded(_),DataFetchState::Loading) => DataFetchState::Loading,
            (DataFetchState::Loaded(a),DataFetchState::Loaded(b)) => DataFetchState::Loaded((a,b)),
        }
    }
}

pub type MemberState = DataFetchState<MemberJSON>;

pub fn use_member(room_id: &str,user_id: &str) -> MemberState {
    let state = use_state(|| MemberState::Loading);
    let state_cloned = state.clone();
    let user_id = user_id.to_string();
    let room_id = room_id.to_string();
    use_effect_with_deps(
        |(room_id,user_id)| {
            firestore::get_member(
                room_id,
                user_id,
                move |member| {
                    state.set(MemberState::Loaded(member))
                },
                || {},
            );
            || {}
        },
        (room_id,user_id),
    );
    (*state_cloned).clone()
}

pub type RoomState = DataFetchState<firestore::Room>;

pub fn use_room_sync(room_id: &str) -> RoomState {
    let state = use_state(|| RoomState::Loading);
    let state_cloned = state.clone();
    let room_id = room_id.to_string();
    use_effect_with_deps(
        |room_id| {
            firestore::sync_room(
                room_id,
                move |room| {
                    state.set(RoomState::Loaded(room))
                },
                || {},
            )
        },
        room_id,
    );
    (*state_cloned).clone()
}

pub fn use_members(room_id: &str) -> DataFetchState<Vec<MemberJSON>> {
    let state = use_state(|| DataFetchState::Loading);
    let state_cloned = state.clone();
    let room_id = room_id.to_string();
    use_effect_with_deps(
        |room_id| {
            firestore::get_members(
                room_id,
                move |members| {
                    state.set(DataFetchState::Loaded(members))
                },
                || {},
            );
            || {}
        },
        room_id,
    );
    (*state_cloned).clone()
}

pub fn use_rolls(room_id: &str) -> DataFetchState<Vec<firestore::Roll>> {
    let state = use_state(|| DataFetchState::Loading);
    let state_cloned = state.clone();
    let room_id = room_id.to_string();
    use_effect_with_deps(
        |room_id| {
            firestore::sync_rolls(
                room_id,
                move |rolls| {
                    state.set(DataFetchState::Loaded(rolls))
                },
                || {},
            )
        },
        room_id,
    );
    (*state_cloned).clone()
}