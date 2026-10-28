use crate::components::ClearFieldButton;
use crate::i18n::use_lang;
use crate::icons::LinkIcon;
use crate::state::WARNING_MESSAGE;
use dioxus::prelude::*;

/// Champ de saisie URL amélioré avec icône, style Material Design 3.
#[component]
pub fn UrlInputField(
    mut value: Signal<String>,
    mut error_message: Signal<String>,
) -> Element {
    let lang = use_lang();

    rsx! {
        div { class: "w-full",
            p { class: "mb-2 text-sm font-medium text-on-surface-variant", {lang.t("your_url")} }

            div { class: "relative w-full",
                // Link icon
                div { class: "absolute left-4 top-1/2 -translate-y-1/2 text-on-surface-variant",
                    LinkIcon { class: "w-5 h-5" }
                }

                input {
                    class: "w-full h-14 rounded-2xl border border-outline/50 bg-surface-container px-4 pl-12 pr-12 text-base text-on-surface outline-none focus:border-primary focus:bg-surface-container-high transition-colors",
                    placeholder: lang.t("url_placeholder"),
                    r#type: "url",
                    value: "{value}",

                    oninput: move |evt| {
                        value.set(evt.value());
                        error_message.set(String::new());
                        WARNING_MESSAGE.write().clear();
                    },
                }

                if !value().is_empty() {
                    ClearFieldButton { value }
                }
            }
        }
    }
}
