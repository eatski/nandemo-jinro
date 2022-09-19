
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
           <Lobby />
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
