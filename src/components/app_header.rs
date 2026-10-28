use dioxus::prelude::*;

/// En-tête d'écran avec titre et description, style Material Design 3.
#[component]
pub fn AppHeader(title: String, description: String) -> Element {
    rsx! {
        div { class: "mb-8 relative",
            div { class: "absolute -top-6 -left-4 w-20 h-20 rounded-3xl bg-primary/10 blur-xl" }
            div { class: "absolute -bottom-4 -right-2 w-16 h-16 rounded-2xl bg-tertiary/10 blur-lg" }

            div { class: "relative",
                h1 { class: "text-3xl font-bold text-secondary leading-tight", "{title}" }
                p { class: "mt-2 text-sm text-on-surface-variant", "{description}" }
            }
        }
    }
}
