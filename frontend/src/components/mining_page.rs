use yew::prelude::*;

#[function_component(MiningPage)]
pub fn mining_page() -> Html {
    html! {
        <div>
            <h1>{"Start Mining"}</h1>
            <p>{"Join the network and start mining Foxtrot coins."}</p>
        </div>
    }
}
