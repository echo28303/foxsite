use yew::prelude::*;

#[function_component(WalletPage)]
pub fn wallet_page() -> Html {
    html! {
        <div>
            <h1>{"Your Wallet"}</h1>
            <p>{"Check your balance and manage your transactions here."}</p>
        </div>
    }
}
