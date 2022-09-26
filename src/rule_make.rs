use firestore::{Rule, Role};
use yew::{function_component, html, Callback, use_state, Properties};
use presentational::{InputText,InputSmallNumber,AddButton,ListItemRow,ListContainer,SimpleCenteringDiv,Heading2WithDescription,SimpleCenteringSection, button};

#[derive(Clone)]
struct Item {
    name: String,
    count: u32,
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
        firestore::set_rule(
            room_id.as_str(),
            &Rule {
                roles: captured_state
                    .iter()
                    .enumerate()
                    .map(|(index,item)| Role {name: item.name.clone(),number: item.count, id: index.to_string()}).collect(),
            },
            || {},
            || {},
        );
    });
    let captured_state = (*state).clone();
    html! {
        <SimpleCenteringSection>
            <Heading2WithDescription title={"ルールを決めましょう"} description={"役職とその人数を決めましょう"}/>
            <ListContainer>
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
                        <ListItemRow>
                            <InputText
                                value={item.name.clone()}
                                placeholder="役職"
                                oninput={on_text_input}
                            />
                            <InputSmallNumber
                                value={item.count}
                                oninput={on_number_input}
                            />
                        </ListItemRow>
                    }
                }
            )}
            </ListContainer>
            <SimpleCenteringDiv>
                <AddButton onclick={Callback::from(move |_| {
                    let mut captured_state = captured_state.clone();
                    captured_state.push(Item {
                        name: "".to_string(),
                        count: 1,
                    });
                    state.set(captured_state)
                })}  />
            </SimpleCenteringDiv>
            <SimpleCenteringDiv>
                {button("ルールを確定",publish_rule)}
            </SimpleCenteringDiv>
        </SimpleCenteringSection>
    }
}