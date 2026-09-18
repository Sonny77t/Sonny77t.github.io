use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::routes::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <section class="my-section">
            <div class="my-wrapper">
                <div class="my-content">
                    <div class="avatar-animada"></div>
                    <h1>{"Holaa! Soy Sonny ♡"}</h1>
                    <h3><strong>{"ABOUT ME"}</strong></h3>
                    <h4>{"Me gusta practicar Roller Skating ♡ me encantan los video juegos, y me gusta el Punk y el K-pop (Mis lados opuestos jeje)"}</h4>
                    <p>{"Aqui compartire un pedacito de mi vida ❤︎⁠"}</p>
                    <nav class="my-social-navbar">
                        <a href="https://www.tiktok.com/@sonny_pinku77t?is_from_webapp=1&sender_device=pc" target="_blank" rel="noopener noreferrer" aria-label="TikTok">
                            <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M19.59 6.69a4.83 4.83 0 0 1-3.77-4.25V2h-3.45v13.67a2.89 2.89 0 0 1-5.2 1.74 2.89 2.89 0 0 1 2.31-4.64 2.93 2.93 0 0 1 .88.13V9.4a6.84 6.84 0 0 0-1-.05A6.33 6.33 0 0 0 5 20.1a6.34 6.34 0 0 0 10.86-4.43v-7a8.16 8.16 0 0 0 4.77 1.52v-3.4a4.85 4.85 0 0 1-1-.1z"/>
                            </svg>
                        </a>
                        <a href="https://www.instagram.com/sonny.pinku77/" target="_blank" rel="noopener noreferrer" aria-label="Instagram">
                            <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <rect width="20" height="20" x="2" y="2" rx="5" ry="5"/>
                                <path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"/>
                                <line x1="17.5" x2="17.51" y1="6.5" y2="6.5"/>
                            </svg>
                        </a>
                        <a href="https://youtube.com/@sonnypinku77?si=bep9kfDci04kr2tm" target="_blank" rel="noopener noreferrer" aria-label="YouTube">
                            <svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z"/>
                            </svg>
                        </a>
                    </nav>
                    <nav class="my-navbar-1">
                        <Link<Route> to={Route::Home}>
                            {"Ejemplo 1"}
                            <span>{">"}</span>
                        </Link<Route>>
                        <Link<Route> to={Route::About}>
                            {"Ejemplo 2"}
                            <span>{">"}</span>
                        </Link<Route>>
                        <Link<Route> to={Route::Proyectos}>
                            {"Ejemplo 3"}
                            <span>{">"}</span>
                        </Link<Route>>
                    </nav>
                    <hr/>
                    <h3><strong>{"LISTA EJEMPLO"}</strong></h3>
                    <nav class="my-navbar-2">
                        <a href="">
                            {"Ejemplo A"}
                            <span>{">"}</span>
                        </a>
                        <a href="">
                            {"Ejemplo B"}
                            <span>{">"}</span>
                        </a>
                        <a href="">
                            {"Ejemplo C"}
                            <span>{">"}</span>
                        </a>
                    </nav>
                    <hr/>
                    <h3><strong>{"CONTACTO"}</strong></h3>
                    <h3>{"sonny.pinku77@gmail.com"}</h3>
                </div>
            </div>
        </section>
    }
}
