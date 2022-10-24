use historical::{HistoricalSignature, next_signature, calculate};

mod use_history_state;

use crate::use_history_state::use_history_state;

pub fn use_historical<T: historical::HistricalItem>(items: Vec<T>,on_push: impl Fn(HistoricalSignature)) -> MaybeOutOfSync<YewHistorical<T,impl Fn()>> {
    let (signature_state, set_signature) = use_history_state::<SerdeHistoricalSignature>();
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
                let signature = SerdeHistoricalSignature::from(next_signature.clone());
                on_push(next_signature);
                set_signature(signature);
            }, 
        })
    }
}

pub struct YewHistorical<T : historical::HistricalItem, F: Fn()> {
    pub current: T::Collected,
    pub push: F
}

pub enum MaybeOutOfSync<T> {
    Ok(T),
    OutOfSync,
}


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq, Default)]
struct SerdeHistoricalSignature {
    pub index: usize,
    pub branch: usize,
}

impl From<HistoricalSignature> for SerdeHistoricalSignature {
    fn from(signature: HistoricalSignature) -> Self {
        Self {
            index: signature.index,
            branch: signature.branch,
        }
    }
}

impl Into<HistoricalSignature> for SerdeHistoricalSignature {
    fn into(self) -> HistoricalSignature {
        HistoricalSignature {
            index: self.index,
            branch: self.branch,
        }
    }
}
