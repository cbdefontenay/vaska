use dioxus::prelude::*;

const DEFAULT_CLASS: &str =
    "mt-4 w-full h-14 rounded-2xl bg-primary text-on-primary text-base font-semibold active:opacity-80 disabled:opacity-50";

/// Bouton d'action générique stylé avec Tailwindcss.
#[component]
pub fn ActionButton(
    label: String,
    class: Option<String>,
    disabled: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: class.unwrap_or_else(|| DEFAULT_CLASS.to_string()),
            r#type: "button",
            disabled,
            onclick,
            "{label}"
        }
    }
}
