use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::home_page::HomePage;
use crate::components::about_page::AboutPage;
use crate::components::wallet_page::WalletPage;
use crate::components::mining_page::MiningPage;
use crate::components::features_page::FeaturesPage;
use crate::components::community_page::CommunityPage;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/wallet")]
    Wallet,
    #[at("/mining")]
    Mining,
    #[at("/features")]
    Features,
    #[at("/community")]
    Community,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <HomePage /> },
        AppRoute::About => html! { <AboutPage /> },
        AppRoute::Wallet => html! { <WalletPage /> },
        AppRoute::Mining => html! { <MiningPage /> },
        AppRoute::Features => html! { <FeaturesPage /> },
        AppRoute::Community => html! { <CommunityPage /> },
    }
}
