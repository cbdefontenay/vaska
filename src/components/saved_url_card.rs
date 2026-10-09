use crate::components::copy_to_clipboard;
use dioxus::prelude::*;

/// Carte affichant une URL enregistrée, avec copie et suppression.
#[component]
pub fn SavedUrlCard(id: i64, url: String, on_delete: EventHandler<i64>) -> Element {
    let url_copy = url.clone();

    rsx! {
        div { class: "w-full rounded-2xl border border-primary p-4",
            p { class: "select-all text-sm text-secondary break-all leading-relaxed", "{url}" }

            div { class: "mt-3 flex gap-3",
                button {
                    class: "h-10 flex-1 rounded-2xl bg-primary text-on-primary text-xs font-semibold active:opacity-80",
                    r#type: "button",

                    onclick: move |_| copy_to_clipboard(url_copy.clone()),
                    "Copier"
                }

                button {
                    class: "h-10 flex-1 rounded-2xl border border-red-500 bg-transparent text-red-500 text-xs font-semibold active:opacity-80",
                    r#type: "button",

                    onclick: move |_| on_delete.call(id),
                    "Supprimer"
                }
            }
        }
    }
}
