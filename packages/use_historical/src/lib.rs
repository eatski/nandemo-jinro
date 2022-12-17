use std::{fmt::Debug};
use firestore_hooks::{use_collection_sync, DataFetchResult};
use historical::{HistoricalSignature, next_signature, calculate_latest};
use yew::{Callback, Children, Properties, hook};

#[hook]
pub fn use_historical<T: historical::HistricalItem + firestore::FireStoreResource + Clone + 'static,B>(param: T::ParamForPath,merge: impl Fn(HistoricalSignature,B) -> T + 'static) -> DataFetchResult<YewHistorical<T,B>> where T::ParamForPath: Clone + PartialEq {
    let collection = use_collection_sync::<T>(&param);
    collection.map(|collection| {
        let next_signature = next_signature(&collection, None);
        let latest = calculate_latest(collection);
        let push_with_callback = Callback::from(move |(body,callback): (B,Box<dyn FnOnce()>)| {
            let next_signature = next_signature.clone();
            firestore::add_document(&param, &merge(next_signature,body), |_| {
                callback();
            }, || {} );
        });
        YewHistorical { latest, push_with_callback }
    })
}

#[hook]
pub fn use_historical_read<T: historical::HistricalItem + firestore::FireStoreResource + Clone + 'static>(param: T::ParamForPath) -> DataFetchResult<YewHistoricalRead<T::Collected>> where T::ParamForPath: Clone + PartialEq,T::Collected : Debug {
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
    pub push_with_callback: Callback<(B,Box<dyn FnOnce()>)>,
}


#[derive(PartialEq,Clone,Properties)]
pub struct ChildrenProps {
    pub children: Children,
}