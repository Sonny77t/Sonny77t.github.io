use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section class="my-section">
            <div class="my-wrapper">
                <div class="my-content">
                    <h1>{"Sobre mí"}</h1>
                    <p>{"Página de Sonny Pink en construcción."}</p>
                </div>
            </div>
        </section>
    }
}
