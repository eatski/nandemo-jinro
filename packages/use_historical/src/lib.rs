use std::{fmt::Debug};
use firestore_hooks::{use_collection_sync, DataFetchState};
use historical::{HistoricalSignature, next_signature, calculate, calculate_latest};
use use_history_state::HistoryState;
use wasm_bindgen::JsValue;
use yew::{Callback, html, ContextProvider, function_component, Children, Properties, use_context};

mod use_history_state;

use crate::use_history_state::use_history_state;

pub fn use_historical<T: historical::HistricalItem + firestore::FireStoreResource + Clone + 'static,B>(param: T::ParamForPath,merge: impl Fn(HistoricalSignature,B) -> T + 'static) -> DataFetchState<YewHistorical<T,B>> where T::ParamForPath: Clone + PartialEq {
    let collection = use_collection_sync::<T>(&param);
    let context = use_context::<HistoryState<HistoricalSignature>>().expect("use_historical: no history state");
    match collection {
        firestore_hooks::DataFetchState::Loading => {
            DataFetchState::Loading
        },
        firestore_hooks::DataFetchState::Loaded(items) => {
            let current_signature = context.state.clone();
            let current_index = current_signature.clone().map(|s| s.index);
            let is_out_of_sync = current_signature
                .map(|current_signature| items.iter().all(|item| item.signature() != current_signature.clone()))
                .unwrap_or(false);
            if is_out_of_sync {
                DataFetchState::Loading
            } else {
                let next_signature = next_signature(&items, current_index);
                DataFetchState::Loaded(YewHistorical {
                    current: calculate(items.clone(), current_index),
                    latest: calculate_latest(items),
                    push: Callback::from(move |body| {
                        let next_signature = next_signature.clone();
                        let signature = next_signature.clone();
                        context.push.emit(signature);
                        firestore::add_document(&param, &merge(next_signature,body), |_| {}, || {} );
                    }), 
                })
            }
        }
        firestore_hooks::DataFetchState::Error => {
            DataFetchState::Error
        },
    }
}

pub fn use_historical_read<T: historical::HistricalItem + firestore::FireStoreResource + Clone + 'static>(param: T::ParamForPath) -> DataFetchState<YewHistoricalRead<T::Collected>> where T::ParamForPath: Clone + PartialEq,T::Collected : Debug {
    let collection = use_collection_sync::<T>(&param);
    let state = use_context::<HistoryState<HistoricalSignature>>().expect("use_historical_read must be used inside a HistoryStateProvider");

    collection.map(move |items| {
        let read = YewHistoricalRead { current: calculate(items.clone(), state.state.map(|sig| sig.index)), latest: calculate_latest(items) };
        web_sys::console::log_1(&JsValue::from_str(&format!("use_historical_read: {:?}", read)));
        read
    })
}


#[derive(Debug)]
pub struct YewHistoricalRead<T> {
    pub current: T,
    pub latest: T
}

pub struct YewHistorical<T : historical::HistricalItem, B> {
    pub current: T::Collected,
    pub latest: T::Collected,
    pub push: Callback<B>,
}

#[derive(PartialEq,Clone,Properties)]
pub struct ChildrenProps {
    pub children: Children,
}

#[function_component(HistoricalProvider)]
pub fn historical_provider(props: &ChildrenProps) -> Html {
    let state = use_history_state::<HistoricalSignature>();
    html! {
        <ContextProvider<HistoryState<HistoricalSignature>> context={state.clone()}>{props.children.clone()}</ContextProvider<HistoryState<HistoricalSignature>>>
    }
}