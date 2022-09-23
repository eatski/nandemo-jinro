use yew::{function_component, html, Callback, use_state};
use presentational::{InputText,InputSmallNumber,AddButton,ListItemRow,ListContainer,SimpleCenteringDiv};

#[derive(Clone)]
struct Item {
    name: String,
    count: u32,
}

#[function_component(RuleMake)]
pub fn rule_make() -> Html {
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
    html! {
        <>
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
        </>
    }
}