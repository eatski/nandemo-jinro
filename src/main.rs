
use presentational::{header,footer};
use router::Route;
use yew::prelude::*;
use landing::Landing;
use yew_router::prelude::*;
use room::Room;

mod room;
mod landing;
mod router;
mod storage;
mod entrance;
mod rule_make;
mod lobby;
mod hook;
mod roll;
mod rolled;
mod function;

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
            <Room room_id={id.to_string()}/>
        },
        Route::NotFound => todo!(),
    }
}

fn main() {
    yew::start_app::<Root>();
}