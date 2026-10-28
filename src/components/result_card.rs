use crate::components::{copy_to_clipboard, SaveUrlButton};
use crate::i18n::use_lang;
use dioxus::prelude::*;
use webbrowser::open;
use crate::icons::{ConfirmedIcon, CopyIcon, OpenBrowserIcon, WarningIcon};

#[component]
pub fn ResultCard(cleaned_url: String, tracker_origin: Option<String>, show_warning: bool ) -> Element {
    let lang = use_lang();

    if show_warning {
        return rsx! {};
    }

    let url_open = cleaned_url.clone();
    let url_copy = cleaned_url.clone();
    let url_save = cleaned_url.clone();

    rsx! {
        div { class: "mt-8 w-full rounded-3xl bg-surface-container-high p-5 shadow-lg border border-outline/20",
            // Status badge
            div { class: "flex items-center gap-2 mb-3",
                div { class: "w-6 h-6 rounded-full bg-primary/20 flex items-center justify-center",
                    ConfirmedIcon { class: "w-4 h-4 text-primary" }
                }
                span { class: "text-sm font-semibold text-primary", {lang.t("cleaned_url")} }
            }

            // URL display
            div { class: "rounded-2xl bg-surface-container p-4 mb-4",
                p { class: "select-all text-sm text-on-surface break-all leading-relaxed",
                    "{cleaned_url}"
                }
            }

            // Action buttons - Bouton Ouvrir
            button {
                class: "w-full h-14 rounded-2xl bg-primary text-on-primary text-base font-semibold flex items-center justify-center gap-2 active:opacity-80 shadow-md mb-3",
                r#type: "button",
                onclick: move |_| {
                    let url = url_open.clone();
                    spawn(async move {
                        let _ = open(&url);
                    });
                },
                OpenBrowserIcon {}
                {lang.t("open_browser")}
            }

            // Action buttons - Copier et Enregistrer
            div { class: "flex gap-3 w-full",
                button {
                    class: "flex-1 h-12 rounded-xl bg-secondary text-on-secondary text-sm font-semibold flex items-center justify-center gap-2 active:opacity-80 shadow-sm",
                    r#type: "button",
                    onclick: move |_| copy_to_clipboard(url_copy.clone()),
                    CopyIcon { class: "w-4 h-4" }
                    {lang.t("copy")}
                }
                SaveUrlButton {
                    value: url_save,
                    class: Some(
                        "flex-1 h-12 rounded-xl bg-tertiary text-on-tertiary text-sm font-semibold flex items-center justify-center gap-2 active:opacity-80 shadow-sm"
                            .to_string(),
                    ),
                }
            }

            // Tracking origin info
            if let Some(ref origin) = tracker_origin {
                div { class: "mt-4 flex items-center gap-2 px-3 py-2 rounded-xl bg-secondary-container/50",
                    WarningIcon { class: "w-4 h-4 text-on-secondary-container" }

                    p { class: "text-xs text-on-secondary-container", {lang.t_with("tracking_detected", origin)} }
                }
            }
        }
    }
}
