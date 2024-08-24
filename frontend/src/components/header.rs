use crate::router::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let is_menu_open = use_state(|| false);

    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| is_menu_open.set(!*is_menu_open))
    };

    html! {
        <header class="header">
            <nav class="navbar">
            <div class="navbar-brand">
                <a href="#">{ "Foxtrot Blockchain" }</a>
                <button class="burger-menu" onclick={toggle_menu.clone()}>
                    <span class="burger-line"></span>
                    <span class="burger-line"></span>
                    <span class="burger-line"></span>
                </button>
            </div>
            <div id={if *is_menu_open { "open" } else { "navbar-links" }}>
                <ul>
                    <li><Link<AppRoute> to={AppRoute::Home}>{ "Home" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::About}>{ "About" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::Wallet}>{ "Wallet" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::Mining}>{ "Mining" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::Features}>{ "Features" }</Link<AppRoute>></li>
                    <li><Link<AppRoute> to={AppRoute::Community}>{ "Community" }</Link<AppRoute>></li>
                </ul>
            </div>
        </nav>
        </header>
    }
}
