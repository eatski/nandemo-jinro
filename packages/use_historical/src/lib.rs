use std::{fmt::Debug};
use firestore_hooks::{use_collection_sync, DataFetchState};
use historical::{HistoricalSignature, next_signature, calculate_latest};
use yew::{Callback, Children, Properties, hook};

#[hook]
pub fn use_historical<T: historical::HistricalItem + firestore::FireStoreResource + Clone + 'static,B>(param: T::ParamForPath,merge: impl Fn(HistoricalSignature,B) -> T + 'static) -> DataFetchState<YewHistorical<T,B>> where T::ParamForPath: Clone + PartialEq {
    let collection = use_collection_sync::<T>(&param);
    match collection {
        firestore_hooks::DataFetchState::Loading => {
            DataFetchState::Loading
        },
        firestore_hooks::DataFetchState::Loaded(items) => {
            let next_signature = next_signature(&items, None);
            DataFetchState::Loaded(YewHistorical {
                latest: calculate_latest(items),
                push: Callback::from(move |body| {
                    let next_signature = next_signature.clone();
                    firestore::add_document(&param, &merge(next_signature,body), |_| {}, || {} );
                }), 
            })
        }
        firestore_hooks::DataFetchState::Error => {
            DataFetchState::Error
        },
    }
}

#[hook]
pub fn use_historical_read<T: historical::HistricalItem + firestore::FireStoreResource + Clone + 'static>(param: T::ParamForPath) -> DataFetchState<YewHistoricalRead<T::Collected>> where T::ParamForPath: Clone + PartialEq,T::Collected : Debug {
    let collection = use_collection_sync::<T>(&param);
    collection.map(move |items| {
        let read = YewHistoricalRead { latest: calculate_latest(items) };
        read
    })
}


#[derive(Debug)]
pub struct YewHistoricalRead<T> {
    pub latest: T
}

pub struct YewHistorical<T : historical::HistricalItem, B> {
    pub latest: T::Collected,
    pub push: Callback<B>,
}

#[derive(PartialEq,Clone,Properties)]
pub struct ChildrenProps {
    pub children: Children,
}