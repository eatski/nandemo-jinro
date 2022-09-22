
use presentational::{header,footer};
use router::Route;
use yew::prelude::*;
use landing::Landing;
use yew_router::prelude::*;
use lobby::Lobby;

mod lobby;
mod landing;
mod router;
mod storage;

#[function_component(Root)]
fn root() -> Html {
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

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Landing /> },
        Route::Room { id } => html! {
            <Lobby room_id={id.to_string()}/>
        },
        Route::NotFound => todo!(),
    }
}

fn main() {
    yew::start_app::<Root>();
}