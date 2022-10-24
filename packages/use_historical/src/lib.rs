use historical::{HistoricalSignature,HistricalItem};
use wasm_bindgen::{prelude::Closure, JsValue};
use yew::{use_effect, use_state};

pub fn use_historical<T: historical::HistricalModel>(items: Vec<T::Item>,on_push: impl Fn(HistoricalSignature)) -> MaybeOutOfSync<YewHistorical<T,impl Fn()>> {
    let signature_state = use_state(Option::<SerdeHistoricalSignature>::default);
    {
        let signature_state = signature_state.clone();
        use_effect(|| {
            let window = web_sys::window().unwrap();
            let history = window.history().unwrap();
            let callback : Box<dyn FnMut()> = Box::new(move || {
                signature_state.set(
                    serde_json::from_str(
                        history.clone()
                        .state().unwrap_or_default()
                        .as_string()
                        .unwrap()
                        .as_str()
                    ).ok()
                );
            });
            let callback = Closure::wrap( callback).into_js_value().into();
            window.add_event_listener_with_callback("popstate", &callback).unwrap();
            move || {
                window.remove_event_listener_with_callback("popstate", &callback).unwrap();
            }
        });
    }
    let current_signature = (*signature_state).clone();
    let current_index = current_signature.as_ref().map(|s| s.index);
    let next_signature = T::next_signature(&items, current_index);

    let is_out_of_sync = current_signature
        .map(|current_signature| items.iter().all(|item| item.signature() != current_signature.clone().into()))
        .unwrap_or(false);

    if is_out_of_sync {
        MaybeOutOfSync::OutOfSync
    } else {
        MaybeOutOfSync::Ok(YewHistorical {
            current: T::calculate(items, current_index),
            push: move || {
                let window = web_sys::window().unwrap();
                let history = window.history().unwrap();
                let next_signature = next_signature.clone();
                let signature = SerdeHistoricalSignature::from(next_signature.clone());
                history.push_state(&JsValue::from_str(serde_json::to_string(&signature).unwrap().as_str()), "").unwrap();
                on_push(next_signature);
                signature_state.set(Some(signature));
            }, 
        })
    }

   
}

pub struct YewHistorical<T : historical::HistricalModel, F: Fn()> {
    pub current: T,
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
