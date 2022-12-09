use serde::{Serialize, Deserialize};
use web_sys::window;
use yew::{use_state, Callback, hook, use_effect_with_deps};


/**
 * A hook that allows you to use a state that is stored in the browser history.
 */
#[hook]
pub fn use_history_state<T: Clone + PartialEq + Serialize + for<'a> Deserialize<'a> + 'static>() -> (Option<T>,Callback<T>) {
   let history = web_sys::window().unwrap().history().unwrap();
   
   let state = use_state(||  {
        let state = history.state().ok()?;
        let state: T = serde_wasm_bindgen::from_value(state).ok()?;
        Some(state)
   });
   {
    let history = history.clone();
    use_effect_with_deps(|state| {
        let state = state.clone();
        let window = window().unwrap();
        let callback = wasm_bindgen::prelude::Closure::wrap(Box::new(move |_: web_sys::Event| {
            let next_state = history.state().unwrap();
            let next_state = serde_wasm_bindgen::from_value(next_state).ok();
            state.set(next_state);
        }) as Box<dyn FnMut(_)>).into_js_value();
        window.add_event_listener_with_callback("popstate", &callback.clone().into()).unwrap();
        move || {
            window.remove_event_listener_with_callback("popstate", &callback.into()).unwrap();
        }
    }, state.clone());
   }
  
   let callback = {
        let state = state.clone();
        Callback::from(move |next_val| {
            let next_val_js = serde_wasm_bindgen::to_value(&next_val).unwrap();
            history.push_state(&next_val_js,"").unwrap();
            state.set(Some(next_val));
        })
    };
   let state = &*state;
    (state.clone(),callback)
}

