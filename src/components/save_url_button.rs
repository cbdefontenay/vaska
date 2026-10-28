use crate::components::ActionButton;
use crate::database;
use crate::i18n::use_lang;
use dioxus::prelude::*;

/// Enregistre l'URL nettoyée dans la base de données locale.
#[component]
pub fn SaveUrlButton(value: String, class: Option<String>) -> Element {
    let lang = use_lang();
    let mut saved = use_signal(|| false);
    let label = if saved() {
        lang.t("saved")
    } else {
        lang.t("save_url")
    };

    rsx! {
        ActionButton {
            label,
            class,
            disabled: saved(),

            onclick: move |_| {
                let url = value.clone();
                saved.set(true);

                spawn(async move {
                    if database::insert_url(&url).await.is_err() {
                        saved.set(false);
                    }
                });
            },
        }
    }
}
