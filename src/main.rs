use router::Route;
use yew::prelude::*;
use components::landing::Landing;
use components::layout::Layout;
use yew_router::prelude::*;
use components::room::Room;

mod components;
mod router;
mod storage;
mod entrance;
mod rule_make;
mod lobby;
mod roll;
mod rolled;
mod hooks;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <Layout>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </Layout>
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