use dioxus::prelude::*;

#[component]
pub fn SettingsScreen() -> Element {
    rsx! {
        div { class: "min-h-screen w-full bg-surface pt-safe", "Hi there!" }
    }
}
