use firestore_hooks::{use_collection_sync, DataFetchState};
use historical::{HistoricalSignature, next_signature, calculate, calculate_latest};

mod use_history_state;

use crate::use_history_state::use_history_state;

pub fn use_historical<T: historical::HistricalItem + firestore::FireStoreResource + Clone + 'static>(param: T::ParamForPath,on_push: impl Fn(HistoricalSignature) -> T) -> MaybeOutOfSync<YewHistorical<T,impl Fn()>> where T::ParamForPath: Clone + PartialEq {
    let collection = use_collection_sync::<T>(&param);
    let (signature_state, set_signature) = use_history_state::<HistoricalSignature>();
    match collection {
        firestore_hooks::DataFetchState::Loading => {
            MaybeOutOfSync::OutOfSync
        },
        firestore_hooks::DataFetchState::Loaded(items) => {
            let current_signature = signature_state.clone();
            let current_index = current_signature.as_ref().map(|s| s.index);
            let next_signature = next_signature(&items, current_index);
            let is_out_of_sync = current_signature
                .map(|current_signature| items.iter().all(|item| item.signature() != current_signature.clone().into()))
                .unwrap_or(false);
            if is_out_of_sync {
                MaybeOutOfSync::OutOfSync
            } else {
                MaybeOutOfSync::Ok(YewHistorical {
                    current: calculate(items, current_index),
                    push: move || {
                        let next_signature = next_signature.clone();
                        let signature = next_signature.clone();
                        set_signature(signature);
                        firestore::add_document(&param, &on_push(next_signature), |_| {}, || {} );
                    }, 
                })
            }
        }
        firestore_hooks::DataFetchState::Error => {
            MaybeOutOfSync::Error
        },
    }
}

pub fn use_historical_read<T: historical::HistricalItem + firestore::FireStoreResource + Clone + 'static>(param: T::ParamForPath) -> DataFetchState<T::Collected> where T::ParamForPath: Clone + PartialEq,T::Collected: Clone {
    let collection = use_collection_sync::<T>(&param);
    collection.map(|items| calculate_latest(items))
}


pub struct YewHistorical<T : historical::HistricalItem, F: Fn()> {
    pub current: T::Collected,
    pub push: F
}

pub enum MaybeOutOfSync<T> {
    Ok(T),
    OutOfSync,
    Error
}