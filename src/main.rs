
use presentational::{header, title, CardListContainer, card_case};
use yew::prelude::*;

mod members;
mod firestore;

use members::Lobby;

struct Model();

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div>
                {header()}
                {title()}
                <CardListContainer>
                    {card_case()}
                    {card_case()}
                </CardListContainer>
                <Lobby />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
