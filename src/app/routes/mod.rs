use yew::prelude::*;
use yew_router::prelude::*;

pub mod home;
pub mod about;
pub mod proyectos;

pub use home::Home;
pub use about::About;
pub use proyectos::Proyectos;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/proyectos")]
    Proyectos,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Proyectos => html! { <Proyectos /> },
        Route::NotFound => html! { <Redirect<Route> to={Route::Home} /> },
    }
}
