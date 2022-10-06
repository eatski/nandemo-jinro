use firestore_hooks::use_collection_sync;
use model::{Rule, Role, SetRule, MemberJSON};
use yew::{function_component, html, Callback, use_state, Properties};
use atoms::{InputText,InputSmallNumber,Heading2,HeadingDescription,ButtonLarge};
use layouting::{FixToBottom};

#[derive(Clone)]
struct Item {
    name: String,
    count: usize,
}

#[derive(Properties,PartialEq)]
pub struct Props {
    pub room_id: String,
}

#[function_component(RuleMake)]
pub fn rule_make(props: &Props) -> Html {
    let state = use_state(|| vec![
        Item {
            name: "市民".to_string(),
            count: 3,
        },
        Item {
            name: "人狼".to_string(),
            count: 1,
        },
    ]);
    let captured_state = (*state).clone();
    let room_id = props.room_id.clone();
    let publish_rule = Callback::from(move |_| {
        firestore::set_document(
    &(),
    room_id.as_str(),
            &SetRule {
                rule:  Rule {
                    roles: captured_state
                        .iter()
                        .enumerate()
                        .map(|(index,item)| Role {name: item.name.clone(),number: item.count, id: index.to_string()}).collect()
                },
            },
            || {},
            || {},
        );
    });
    let captured_state = (*state).clone();
    let members = use_collection_sync::<MemberJSON>(&props.room_id);
    html! {
        <section class="mx-auto w-full max-w-2xl py-2">
            <Heading2>{"ルールを決めましょう"}</Heading2>
            {
                match members {
                    firestore_hooks::DataFetchState::Loading => Default::default(),
                    firestore_hooks::DataFetchState::Loaded(members) => html! {
                        <>
                            <HeadingDescription>{"役職とその人数を決めましょう"}</HeadingDescription>
                            <HeadingDescription>{"現在 "}<b>{members.len()}</b>{"人のプレイヤーが参加中"}</HeadingDescription>
                            <ul class="flex flex-col gap-2 mt-8">
                                {for (*state).iter().enumerate().map(|(index,item)| {
                                    let on_number_input = {
                                        let mut captured_state = captured_state.clone();
                                        let state = state.clone();  
                                        Callback::once(move |count| {
                                            captured_state[index].count = count;
                                            state.set(captured_state)
                                        })
                                    };
                                    let on_text_input = {
                                        let mut captured_state = captured_state.clone();
                                        let state = state.clone();  
                                        Callback::once(move |name| {
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
                            <div class="flex justify-center mt-4">
                                <div class="h-6 w-6 relative">
                                    <button onclick={Callback::from(move |_| {
                                        let mut captured_state = captured_state.clone();
                                        captured_state.push(Item {
                                            name: "".to_string(),
                                            count: 1,
                                        });
                                        state.set(captured_state)
                                    })} class="
                                        absolute -translate-y-1/2 -translate-x-1/2 top-1/2 left-1/2
                                        w-5 h-5 rounded-full
                                        bg-feature transition-colors hover:bg-feature-light
                                    " 
                                    >
                                        <span role="img" aria-label="追加" style="top:44%;left: 52%;" class="absolute -translate-y-1/2 -translate-x-1/2 left-1/2 text-white">{"+"}</span>
                                    </button>
                                </div>
                            </div>
                            <FixToBottom>
                                <ButtonLarge onclick={publish_rule}>
                                    {"ルールを確定"}
                                </ButtonLarge>
                            </FixToBottom>
                        </>
                    },
                }
            }
           
        </section>
    }
}