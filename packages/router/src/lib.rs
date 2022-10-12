use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/rooms/:id")]
    Room { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
