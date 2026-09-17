use yew::prelude::*;
use yew_router::prelude::*;

mod routes;
use routes::{switch, Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="my-container">
                <Switch<Route> render={switch}/>
            </div>
        </BrowserRouter>
    }
}
