use std::collections::HashSet;

use atoms::{ButtonLarge, Heading2, HeadingDescription, InputSmallNumber, InputText, unexpected_error};
use firestore_hooks::{use_collection_sync, DataFetchState};
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
    let state = use_input(props.room_id.as_str(),vec![
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
    let captured_state = (*state).clone();
    let members = use_collection_sync::<MemberJSON>(&props.room_id);
    html! {
        <section class="mx-auto w-full max-w-2xl py-2">
                {
                    match members.merge(room) {
                        firestore_hooks::DataFetchState::Loading => Default::default(),
                        firestore_hooks::DataFetchState::Loaded((members,YewHistorical {push, ..})) => html! {
                            <>
                                <BodyItems>
                                    <Heading2>{"ルールを決めましょう"}</Heading2>
                                    <HeadingDescription>{"役職とその人数を決めましょう"}</HeadingDescription>
                                    <HeadingDescription>{"現在 "}<b>{members.len()}</b>{"人のプレイヤーが参加中"}</HeadingDescription>
                                    <ul class="flex flex-col gap-2 mt-8">
                                        {for (*state).iter().enumerate().map(|(index,item)| {
                                            let on_number_input = {
                                                let captured_state = captured_state.clone();
                                                let state = state.clone();
                                                Callback::from(move |count| {
                                                    let mut captured_state = captured_state.clone();
                                                    let state = state.clone();
                                                    captured_state[index].count = count;
                                                    state.set(captured_state)
                                                })
                                            };
                                            let on_text_input = {
                                                let captured_state = captured_state.clone();
                                                let state = state.clone();
                                                Callback::from(move |name| {
                                                    let mut captured_state = captured_state.clone();
                                                    let state = state.clone();
                                                    captured_state[index].name = name;
                                                    state.set(captured_state)
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
                                                let state = state.clone();
                                                let captured_state = captured_state.clone();
                                                Callback::from(move |_| {
                                                    let mut captured_state = captured_state.clone();
                                                    captured_state.push(Item {
                                                        name: "".to_string(),
                                                        count: 1,
                                                    });
                                                    state.set(captured_state)
                                                })
                                            }
                                            class="text-word hover:text-word-2nd"
                                            aria-label="役職を追加"
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
                                                            let state = state.clone();
                                                            let captured_state = captured_state.clone();
                                                            (!disabled).then(|| {
                                                                Callback::from(move |_| {
                                                                    let mut captured_state = captured_state.clone();
                                                                    captured_state.pop();
                                                                    state.set(captured_state)
                                                                })
                                                            }) 
                                                        }
                                                        disabled={disabled}
                                                        class="text-word hover:text-word-2nd disabled:text-word-disable"
                                                        aria-label="役職を削除"
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
                                    let not_enough_roles = (*state).iter().map(|e| e.count).sum::<usize>() < members.len();
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
                                        let push = push.clone();
                                        Callback::from(move |_| {
                                            push.emit(RoomEditBody::SetRule(rule.clone()));
                                            if let Some(on_complete) = &on_complete {
                                                on_complete.emit(());
                                            };
                                        })
                                    };
                                    html! {
                                        <BottomOperaton>
                                            <ButtonLarge disabled={empty || duplicated_name || not_enough_roles} onclick={onclick}>
                                                {"ルールを確定"}
                                            </ButtonLarge>
                                        </BottomOperaton>
                                    }
                                }}
                            </>
                        },
                        DataFetchState::Error => unexpected_error()
                    }
                }
        </section>
    }
}
