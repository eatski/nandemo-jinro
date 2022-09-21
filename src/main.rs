
use presentational::{header,footer};
use yew::prelude::*;
use landing::Landing;

mod members;
mod firestore;
mod landing;

struct Root();

impl Component for Root {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Root {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <div>
                {header()}
                <Landing />
                {footer()}
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Root>();
}