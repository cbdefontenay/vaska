use crate::components::{AppHeader, LoadingSpinner, SavedUrlCard};
use crate::database;
use crate::i18n::use_lang;
use crate::icons::ArchiveIcon;
use dioxus::prelude::*;

#[component]
pub fn UrlSaverScreen() -> Element {
    let lang = use_lang();
    let mut urls = use_resource(|| async move { database::list_urls().await.unwrap_or_default() });

    rsx! {
        div { class: "min-h-screen w-full bg-surface px-5 pt-safe-12 pb-24",
            // Header
            AppHeader {
                title: lang.t("urls_saved"),
                description: lang.t("urls_saved_description"),
            }

            match urls() {
                None => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-4",
                        LoadingSpinner {}
                        p { class: "text-sm text-on-surface-variant", {lang.t("loading")} }
                    }
                },

                Some(saved_urls) if saved_urls.is_empty() => rsx! {
                    div { class: "flex flex-col items-center justify-center py-20 gap-4",
                        // Illustration vide
                        div { class: "w-20 h-20 rounded-3xl bg-surface-container-high flex items-center justify-center",
                            ArchiveIcon { class: "w-10 h-10 text-on-surface-variant opacity-50" }
                        }
                        div { class: "text-center",
                            p { class: "text-base font-medium text-on-surface mb-1", {lang.t("no_saved_urls")} }
                            p { class: "text-sm text-on-surface-variant",
                                {lang.t("no_saved_urls_description")}
                            }
                        }
                    }
                },

                Some(saved_urls) => rsx! {
                    div { class: "flex flex-col gap-3",
                        for saved_url in saved_urls {
                            SavedUrlCard {
                                key: "{saved_url.id}",
                                id: saved_url.id,
                                url: saved_url.url.clone(),

                                on_delete: move |id| {
                                    spawn(async move {
                                        if database::delete_url(id).await.is_ok() {
                                            urls.restart();
                                        }
                                    });
                                },
                            }
                        }
                    }
                },
            }
        }
    }
}
