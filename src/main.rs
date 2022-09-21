
use presentational::{header,footer};
use router::Route;
use yew::prelude::*;
use landing::Landing;
use yew_router::prelude::*;


mod members;
mod firestore;
mod landing;
mod router;
mod storage;

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
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
                {footer()}
            </div>
        }
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Landing /> },
        Route::Room { id } => html! {<p>{format!("You are looking at Post {}", id)}</p>},
        Route::NotFound => todo!(),
    }
}

fn main() {
    yew::start_app::<Root>();
}