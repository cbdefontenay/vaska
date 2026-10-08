use crate::components::{SystemBarColor, SystemBars};
use dioxus::prelude::*;

#[component]
pub fn SettingsScreen() -> Element {
    rsx! {
        SystemBars { color: SystemBarColor::Primary }
        div { class: "min-h-screen w-full bg-surface pt-safe", "Hi there!" }
    }
}
