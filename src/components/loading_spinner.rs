use dioxus::prelude::*;

/// Spinner de chargement animé, style Material Design 3.
#[component]
pub fn LoadingSpinner() -> Element {
    rsx! {
        div {
            class: "animate-spin h-5 w-5 border-2 border-on-surface-variant border-t-primary rounded-full",
            role: "status",
            "aria-label": "Chargement",
        }
    }
}
