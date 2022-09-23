use yew::{function_component, html, Callback};
use presentational::{InputText,InputSmallNumber,AddButton,ListItemRow,ListContainer};

#[function_component(RuleMake)]
pub fn rule_make() -> Html {
    html! {
        <ListContainer>
            <ListItemRow>
                <InputText oninput={Callback::from(|_| {})} placeholder={"役職名"} />
                <InputSmallNumber oninput={Callback::from(|_| {})}  />
            </ListItemRow> 
            <ListItemRow>
                <InputText oninput={Callback::from(|_| {})} placeholder={"役職名"} />
                <InputSmallNumber oninput={Callback::from(|_| {})}  />
                <AddButton onclick={Callback::from(|_| {})}  />
            </ListItemRow>  
            <ListItemRow>
                <InputText oninput={Callback::from(|_| {})} placeholder={"役職名"} />
                <InputSmallNumber oninput={Callback::from(|_| {})}  />
            </ListItemRow>           
        </ListContainer>
    }
}