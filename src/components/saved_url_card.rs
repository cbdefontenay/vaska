use crate::components::copy_to_clipboard;
use crate::i18n::use_lang;
use crate::icons::{LinkIcon, CopyIcon, TrashIcon};
use dioxus::prelude::*;

/// Carte affichant une URL enregistrée, avec copie et suppression.
#[component]
pub fn SavedUrlCard(id: i64, url: String, on_delete: EventHandler<i64>) -> Element {
    let lang = use_lang();
    let url_copy = url.clone();

    rsx! {
        div { class: "w-full rounded-2xl bg-surface-container-high p-4 shadow-md border border-outline/20",
            // URL avec icône
            div { class: "flex items-start gap-3",
                div { class: "mt-0.5 w-8 h-8 rounded-lg bg-primary/10 flex-shrink-0 flex items-center justify-center",
                    LinkIcon { class: "w-4 h-4 text-primary" }
                }
                p { class: "select-all text-sm text-on-surface break-all leading-relaxed",
                    "{url}"
                }
            }

            // Boutons d'action
            div { class: "mt-4 flex gap-3",
                button {
                    class: "h-11 flex-1 rounded-xl bg-primary text-on-primary text-sm font-semibold flex items-center justify-center gap-2 active:opacity-80",
                    r#type: "button",

                    onclick: move |_| copy_to_clipboard(url_copy.clone()),
                    CopyIcon { class: "w-4 h-4" }
                    {lang.t("copy")}
                }

                button {
                    class: "h-11 flex-1 rounded-xl border border-error/30 bg-error-container/30 text-error text-sm font-semibold flex items-center justify-center gap-2 active:opacity-80",
                    r#type: "button",

                    onclick: move |_| on_delete.call(id),
                    TrashIcon { class: "w-4 h-4" }
                    {lang.t("delete")}
                }
            }
        }
    }
}
