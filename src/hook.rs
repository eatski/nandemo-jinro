use firestore::{MemberJSON, future::FireStoreResource};
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

pub fn use_collection<T>(param: &T::ParamForPath) -> DataFetchState<Vec<T>>  where T: 'static + FireStoreResource + Clone ,T::ParamForPath: Clone + PartialEq {
    let state = use_state(|| DataFetchState::Loading);
    let state_cloned = state.clone();
    use_effect_with_deps(
        |param| {
            firestore::future::get_collection(
                param,
                move |members| {
                    state.set(DataFetchState::Loaded(members))
                },
                || {},
            );
            || {}
        },
        param.clone(),
    );
    (*state_cloned).clone()
}

pub fn use_collection_sync<T>(param: &T::ParamForPath) -> DataFetchState<Vec<T>> where T: 'static + FireStoreResource + Clone ,T::ParamForPath: Clone + PartialEq {
    let state = use_state(|| DataFetchState::Loading);
    let state_cloned = state.clone();
    let param = param.clone();
    use_effect_with_deps(
        |param| {
            firestore::future::sync_collection(
                param,
                move |collection| {
                    state.set(DataFetchState::Loaded(collection))
                },
                || {},
            )
        },
        param,
    );
    (*state_cloned).clone()
}

pub fn use_document_sync<T>(param: &T::ParamForPath,document_id: &str) -> DataFetchState<T> where T: 'static + FireStoreResource + Clone ,T::ParamForPath: Clone + PartialEq {
    let state = use_state(|| DataFetchState::Loading);
    let state_cloned = state.clone();
    let param = param.clone();
    let document_id = document_id.to_string();
    use_effect_with_deps(
        move |param| {
            firestore::future::sync_document(
                param,
                document_id.as_str(),
                move |document| {
                    state.set(DataFetchState::Loaded(document))
                },
                || {},
            )
        },
        param.clone(),
    );
    (*state_cloned).clone()
}