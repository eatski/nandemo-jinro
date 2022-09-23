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
    html! {
        <>
            <ListContainer>
                {for (*state).iter().enumerate().map(|(index,item)| {
                    let on_number_input = {
                        let state = state.clone();  
                        Callback::from(move |count| {
                            let items = &*state;
                            state.set(items.iter().enumerate().map(|(i,item)| {
                                if i == index {
                                    Item {
                                        name: item.name.clone(),
                                        count,
                                    }
                                } else {
                                    item.clone()
                                }
                            }).collect())
                        })
                    };
                    let on_text_input = {
                        let state = state.clone();  
                        Callback::from(move |name: String| {
                            let items = &*state;
                            state.set(items.iter().enumerate().map(|(i,item)| {
                                if i == index {
                                    Item {
                                        name:name.clone(),
                                        count: item.count,
                                    }
                                } else {
                                    item.clone()
                                }
                            }).collect())
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
                <AddButton onclick={Callback::from(|_| {})}  />
            </SimpleCenteringDiv>
        </>
    }
}