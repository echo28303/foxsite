use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <nav>
                <a href="/">{"Home"}</a>
                <a href="/about">{"About"}</a>
                <a href="/wallet">{"Wallet"}</a>
                <a href="/mining">{"Mining"}</a>
                <a href="/features">{"Features"}</a>
                <a href="/community">{"Community"}</a>
            </nav>
        </header>
    }
}
