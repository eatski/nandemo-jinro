use firestore::FireStoreResource;
use yew::{use_effect_with_deps, use_state, hook};

#[derive(Clone,PartialEq)]
pub enum NotFetched {
    Loading,
    Error
}

pub type DataFetchResult<T> = Result<T, NotFetched>;

#[hook]
pub fn use_collection<T>(param: &T::ParamForPath) -> DataFetchResult<Vec<T>>
where
    T: 'static + FireStoreResource + Clone,
    T::ParamForPath: Clone + PartialEq,
{
    let state = use_state(|| Err(NotFetched::Loading));
    let state_on_complete = state.clone();
    let state_on_error = state.clone();
    use_effect_with_deps(
        |param| {
            firestore::get_collection(
                param,
                move |members| state_on_complete.set(Ok(members)),
                move || {
                    state_on_error.set(Err(NotFetched::Error));
                },
            );
            || {}
        },
        param.clone(),
    );
    (*state).clone()
}

#[hook]
pub fn use_collection_sync<T>(param: &T::ParamForPath) -> DataFetchResult<Vec<T>>
where
    T: 'static + FireStoreResource + Clone,
    T::ParamForPath: Clone + PartialEq,
{
    let state = use_state(|| Err(NotFetched::Loading));
    let state_cloned = state.clone();
    let state_on_error = state.clone();
    let param = param.clone();
    use_effect_with_deps(
        |param| {
            firestore::sync_collection(
                param,
                move |collection| state.set(Ok(collection)),
                move || {
                    state_on_error.set(Err(NotFetched::Error));
                },
            )
        },
        param,
    );
    (*state_cloned).clone()
}

#[hook]
pub fn use_document_sync<T>(param: &T::ParamForPath, document_id: &str) -> DataFetchResult<T>
where
    T: 'static + FireStoreResource + Clone,
    T::ParamForPath: Clone + PartialEq,
{
    let state = use_state(|| Err(NotFetched::Loading));
    let state_cloned = state.clone();
    let state_on_error = state.clone();
    let param = param.clone();
    let document_id = document_id.to_string();
    use_effect_with_deps(
        move |param| {
            firestore::sync_document(
                param,
                document_id.as_str(),
                move |document| state.set(Ok(document)),
                move || {
                    state_on_error.set(Err(NotFetched::Error));
                },
            )
        },
        param.clone(),
    );
    (*state_cloned).clone()
}

#[hook]
pub fn use_document<T>(param: &T::ParamForPath, document_id: &str) -> DataFetchResult<T>
where
    T: 'static + FireStoreResource + Clone,
    T::ParamForPath: Clone + PartialEq,
{
    let state = use_state(|| Err(NotFetched::Loading));
    let state_cloned = state.clone();
    let state_on_error = state.clone();
    let param = param.clone();
    let document_id = document_id.to_string();
    use_effect_with_deps(
        move |param| {
            firestore::get_document(
                param,
                document_id.as_str(),
                move |document| state.set(Ok(document)),
                move || {
                    state_on_error.set(Err(NotFetched::Error));
                },
            );
            || {}
        },
        param.clone(),
    );
    (*state_cloned).clone()
}
