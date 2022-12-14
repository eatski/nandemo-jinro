use std::iter::repeat;

use atoms::{loading, Heading2, unexpected_error};
use layouting::{BodyItems, BottomOperaton};
use model::{MemberJSON, Roll, RoomEditAction};
use use_historical::use_historical_read;
use wasm_bindgen::prelude::Closure;
use yew::{function_component, html, Callback, Properties, use_effect, use_state, Html};

use firestore_hooks::{use_collection_sync, use_document, NotFetched};

use crate::common::RollButton;
use crate::use_roll::use_roll;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq,Properties)]
struct TimerLoadingProps {
    on_timeout: Callback<()>,
}

#[function_component(TimerLoading)]
fn timer_loading(props: &TimerLoadingProps) -> Html {
    let props = props.clone();
    let window = web_sys::window().unwrap();
    use_effect(move || {
        let id = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            &Closure::once_into_js(move || {
                props.on_timeout.emit(());
            }).into(),
            800,
        ).unwrap();
        move || {
            window.clear_timeout_with_handle(id);
        }
    });
    html! {
        <div class=" flex flex-col h-80 items-center justify-center">
            <div class="text-2xl text-word">{"役職を割り振り中..."}</div>
            <div class="flex justify-center gap-8 mt-6">
                {
                    for repeat(html! {
                        <div class="transition-colors w-3 h-3 bg-action rounded-full animate-bounce"></div>
                    }).take(5)
                }
            </div>
        </div>
        
    }
}

#[function_component(Rolled)]
pub fn rolled(props: &Props) -> Html {
    let rolls = use_collection_sync::<Roll>(&props.room_id);
    let room = use_historical_read::<RoomEditAction>(props.room_id.clone());
    let member = use_document::<MemberJSON>(&props.room_id, props.user_id.as_str());
    let state = (|| {
        let rolls = rolls?;
        let room = room?;
        let member = member?;
        Ok((rolls, room, member))
    })();
    let roll = use_roll(props.room_id.as_str());
    let counter = use_state(|| None);
    match state {
        Err(NotFetched::Loading) => loading(),
        Ok((mut rolls, room, member)) => {
            rolls.sort_by_key(|roll| roll.seq_num);
            let last_rolled = rolls.last();
            match last_rolled {
                Some(last_rolled) => {
                    let seq_num = last_rolled.seq_num;
                    if Some(seq_num) != *counter {
                        return html! {
                            <TimerLoading on_timeout={Callback::from(move |_| {
                                counter.set(Some(seq_num));
                            })} />
                        }
                    }
                    let role = last_rolled
                        .user_to_role
                        .get(props.user_id.as_str())
                        .unwrap();
                    let role_name = room
                        .latest
                        .rule
                        .as_ref()
                        .unwrap()
                        .roles
                        .iter()
                        .find(|role_input| role_input.id == *role)
                        .unwrap()
                        .name
                        .clone();
                    html! {
                        <section>
                            <BodyItems>
                                <Heading2>{"あなたの役職は"}<b>{role_name}</b>{"です"}</Heading2>
                            </BodyItems>
                            {
                                member.is_host.then(|| {
                                    match roll {
                                        Some(roll) => {
                                            html! {
                                                <BottomOperaton>
                                                    <RollButton onclick={roll}>
                                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-full h-full">
                                                            <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                                                        </svg>
                                                    </RollButton>
                                                </BottomOperaton>
                                            }
                                        },
                                        None => loading(),
                                    }

                                }).unwrap_or_default()
                            }
                        </section>
                    }
                }

                None => html! {},
            }
        },
        Err(NotFetched::Error) => unexpected_error()
    }
}
