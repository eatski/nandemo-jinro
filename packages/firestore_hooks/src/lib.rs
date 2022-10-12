use firestore::FireStoreResource;
use yew::{use_effect_with_deps, use_state};

#[derive(Clone, PartialEq, Eq)]
pub enum DataFetchState<R: Clone> {
    Loading,
    Loaded(R),
    Error
}

impl<T: Clone> DataFetchState<T> {
    pub fn merge<T2: Clone>(self, target: DataFetchState<T2>) -> DataFetchState<(T, T2)> {
        match (self, target) {
            (DataFetchState::Loading, DataFetchState::Loading) => DataFetchState::Loading,
            (DataFetchState::Loading, DataFetchState::Loaded(_)) => DataFetchState::Loading,
            (DataFetchState::Loaded(_), DataFetchState::Loading) => DataFetchState::Loading,
            (DataFetchState::Loaded(a), DataFetchState::Loaded(b)) => {
                DataFetchState::Loaded((a, b))
            }
            (_,DataFetchState::Error) => DataFetchState::Error,
            (DataFetchState::Error, _) => DataFetchState::Error,
        }
    }
    pub fn map<T2: Clone>(self, func: impl Fn(T) -> T2) -> DataFetchState<T2> {
        match self {
            DataFetchState::Loaded(a) => DataFetchState::Loaded(func(a)),
            DataFetchState::Loading=> DataFetchState::Loading,
            DataFetchState::Error => DataFetchState::Error,
        }
    }
}

pub fn use_collection<T>(param: &T::ParamForPath) -> DataFetchState<Vec<T>>
where
    T: 'static + FireStoreResource + Clone,
    T::ParamForPath: Clone + PartialEq,
{
    let state = use_state(|| DataFetchState::Loading);
    let state_on_complete = state.clone();
    let state_on_error = state.clone();
    use_effect_with_deps(
        |param| {
            firestore::get_collection(
                param,
                move |members| state_on_complete.set(DataFetchState::Loaded(members)),
                move || {
                    state_on_error.set(DataFetchState::Error);
                },
            );
            || {}
        },
        param.clone(),
    );
    (*state).clone()
}

pub fn use_collection_sync<T>(param: &T::ParamForPath) -> DataFetchState<Vec<T>>
where
    T: 'static + FireStoreResource + Clone,
    T::ParamForPath: Clone + PartialEq,
{
    let state = use_state(|| DataFetchState::Loading);
    let state_cloned = state.clone();
    let param = param.clone();
    use_effect_with_deps(
        |param| {
            firestore::sync_collection(
                param,
                move |collection| state.set(DataFetchState::Loaded(collection)),
                || {},
            )
        },
        param,
    );
    (*state_cloned).clone()
}

pub fn use_document_sync<T>(param: &T::ParamForPath, document_id: &str) -> DataFetchState<T>
where
    T: 'static + FireStoreResource + Clone,
    T::ParamForPath: Clone + PartialEq,
{
    let state = use_state(|| DataFetchState::Loading);
    let state_cloned = state.clone();
    let param = param.clone();
    let document_id = document_id.to_string();
    use_effect_with_deps(
        move |param| {
            firestore::sync_document(
                param,
                document_id.as_str(),
                move |document| state.set(DataFetchState::Loaded(document)),
                || {},
            )
        },
        param.clone(),
    );
    (*state_cloned).clone()
}

pub fn use_document<T>(param: &T::ParamForPath, document_id: &str) -> DataFetchState<T>
where
    T: 'static + FireStoreResource + Clone,
    T::ParamForPath: Clone + PartialEq,
{
    let state = use_state(|| DataFetchState::Loading);
    let state_cloned = state.clone();
    let param = param.clone();
    let document_id = document_id.to_string();
    use_effect_with_deps(
        move |param| {
            firestore::get_document(
                param,
                document_id.as_str(),
                move |document| state.set(DataFetchState::Loaded(document)),
                || {},
            );
            || {}
        },
        param.clone(),
    );
    (*state_cloned).clone()
}
