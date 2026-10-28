mod components;
mod database;
mod helper_functions;
mod i18n;
mod icons;
mod state;
mod views;

use crate::components::{BottomBar, SystemBarColor, SystemBars};
use crate::i18n::LangContextProvider;
use crate::views::{HomeScreen, SettingsScreen, UrlSaverScreen};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(BottomBar)]
    #[route("/")]
    HomeScreen {},
    #[route("/url-saver")]
    UrlSaverScreen {},
    #[route("/parametres")]
    SettingsScreen {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!(
    "/assets/tailwind.css",
    AssetOptions::css().with_static_head(true)
);

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, viewport-fit=cover",
        }
        document::Link { rel: "icon", href: FAVICON }
        SystemBars { color: SystemBarColor::Black }
        LangContextProvider {
            Router::<Route> {}
        }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
    }
}
