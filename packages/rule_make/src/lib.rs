use std::collections::HashSet;

use atoms::{ButtonLarge, Heading2, HeadingDescription, InputSmallNumber, InputText, unexpected_error};
use firestore_hooks::{use_collection_sync, NotFetched};
use input_storage::{Item, use_input};
use layouting::{BodyItems, BottomOperaton};
use model::{MemberJSON, Role, Rule, RoomEditAction, RoomEditBody};
use use_historical::YewHistorical;
use yew::{function_component, html, Callback, Properties, Html};

mod input_storage;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
    pub on_complete: Option<Callback<()>>,
}


#[function_component(RuleMake)]
pub fn rule_make(props: &Props) -> Html {
    let (state,set_state) = use_input(props.room_id.as_str());
    let state = state.unwrap_or(vec![
        Item {
            name: "市民".to_string(),
            count: 3,
        },
        Item {
            name: "人狼".to_string(),
            count: 1,
        },
    ]);
    let room_id = props.room_id.clone();
    let room = {
        use_historical::use_historical::<RoomEditAction,RoomEditBody>(
            room_id.clone(), 
            |signature,body| {
                RoomEditAction { signature, body }
            }
        )
    };
    let captured_state = state.clone();
    let members = use_collection_sync::<MemberJSON>(&props.room_id);
    html! {
        <section class="mx-auto w-full max-w-2xl py-2">
                {
                    match (|| {Ok((members?,room?))})() {
                        Result::Err(NotFetched::Loading) => Default::default(),
                        Result::Ok((members,YewHistorical {push_with_callback, ..})) => html! {
                            <>
                                <BodyItems>
                                    <Heading2>{"ルールを決めましょう"}</Heading2>
                                    <HeadingDescription>{"役職とその人数を決めましょう"}</HeadingDescription>
                                    <HeadingDescription>{"現在 "}<b>{members.len()}</b>{"人のプレイヤーが参加中"}</HeadingDescription>
                                    <ul class="flex flex-col gap-2 mt-8">
                                        {for (*state).iter().enumerate().map(|(index,item)| {
                                            let on_number_input = {
                                                let captured_state = captured_state.clone();
                                                set_state.clone().reform(move |count| {
                                                    let mut captured_state = captured_state.clone();
                                                    captured_state[index].count = count;
                                                    captured_state
                                                })
                                            };
                                            let on_text_input = {
                                                let captured_state = captured_state.clone();
                                                set_state.clone().reform(move |name| {
                                                    let mut captured_state = captured_state.clone();
                                                    captured_state[index].name = name;
                                                    captured_state
                                                })
                                            };
                                            html! {
                                                <li class="flex justify-center gap-3 w-full">
                                                    <InputText
                                                        value={item.name.clone()}
                                                        placeholder="役職"
                                                        oninput={on_text_input}
                                                        maxlength={12}
                                                    />
                                                    <InputSmallNumber
                                                        value={item.count}
                                                        oninput={on_number_input}
                                                    />
                                                </li>
                                            }
                                        }
                                    )}
                                    </ul>
                                    <div class="flex justify-center mt-5 gap-5">
                                        <button
                                            onclick={
                                                let captured_state = captured_state.clone();
                                                set_state.reform(move |_| {
                                                    let mut captured_state = captured_state.clone();
                                                    captured_state.push(Item {
                                                        name: "".to_string(),
                                                        count: 1,
                                                    });
                                                    captured_state
                                                })
                                            }
                                            class="text-word hover:text-word-hover"
                                            aria-label="役職を追加"
                                            title="役職を追加"
                                        >
                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                                            </svg>
                                        </button>
                                        {
                                            {
                                                let disabled = (*state).len() <= 1;
                                                html! {
                                                    <button
                                                        onclick={
                                                            let captured_state = captured_state.clone();
                                                            (!disabled).then(move || {
                                                                set_state.clone().reform(move |_| {
                                                                    let mut captured_state = captured_state.clone();
                                                                    captured_state.pop();
                                                                    captured_state
                                                                })
                                                            }) 
                                                        }
                                                        disabled={disabled}
                                                        class="text-word hover:text-word-hover disabled:text-word-disable"
                                                        aria-label="役職を削除"
                                                        title="役職を削除"
                                                    >
                                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                            <path stroke-linecap="round" stroke-linejoin="round" d="M5.5 12.5h13" />
                                                        </svg>
                                                    </button>
                                                }
                                            }
                                            
                                        }
                                    </div>
                                </BodyItems>
                                {{
                                    let empty = (*state).iter().any(|item| item.name.is_empty());
                                    let duplicated_name = {
                                        let mut names = HashSet::new();
                                        (*state).iter().any(|item| !names.insert(item.name.clone()))
                                    };
                                    let rule = Rule {
                                        roles: state
                                            .iter()
                                            .enumerate()
                                            .map(|(index, item)| Role {
                                                name: item.name.clone(),
                                                number: item.count,
                                                id: index.to_string(),
                                            })
                                            .collect(),
                                    };
                                    let onclick = {
                                        let on_complete = props.on_complete.clone();
                                        let push_with_callback = push_with_callback.clone();
                                        Callback::from(move |_| {
                                            let on_complete = on_complete.clone();
                                            push_with_callback.emit((RoomEditBody::SetRule(rule.clone()), Box::new(move || {
                                                if let Some(on_complete) = &on_complete {
                                                    on_complete.emit(());
                                                };
                                            })));
                                        })
                                    };
                                    html! {
                                        <BottomOperaton>
                                            <ButtonLarge disabled={empty || duplicated_name} onclick={onclick}>
                                                {"ルールを確定"}
                                            </ButtonLarge>
                                        </BottomOperaton>
                                    }
                                }}
                            </>
                        },
                        Result::Err(NotFetched::Error) => unexpected_error()
                    }
                }
        </section>
    }
}
