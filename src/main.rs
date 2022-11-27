use components::layout::Layout;
use components::room::Room;
use landing::landing::Landing;
use router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <Layout>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </Layout>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Landing /> },
        Route::Room { id } => html! {
            <Room room_id={id.to_string()}/>
        },
        Route::NotFound => html! { <p>{"ページが見つかりませんでした"}</p> },
    }
}

fn main() {
    yew::Renderer::<Root>::new().render();
}
