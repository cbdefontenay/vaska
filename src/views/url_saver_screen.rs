use crate::components::SavedUrlCard;
use crate::database;
use dioxus::prelude::*;

#[component]
pub fn UrlSaverScreen() -> Element {
    let mut urls = use_resource(|| async move { database::list_urls().await.unwrap_or_default() });

    rsx! {
        div { class: "min-h-screen w-full bg-surface px-5 pt-safe-12 pb-24",

            // Header
            div { class: "mb-8",
                h1 { class: "text-3xl font-bold text-secondary leading-tight", "URLs enregistrées" }

                p { class: "mt-2 text-sm text-secondary opacity-70",
                    "Retrouvez ici les URLs nettoyées que vous avez enregistrées."
                }
            }

            match urls() {
                None => rsx! {
                    p { class: "text-sm text-secondary opacity-70", "Chargement..." }
                },

                Some(saved_urls) if saved_urls.is_empty() => rsx! {
                    p { class: "text-sm text-secondary opacity-70",
                        "Aucune URL enregistrée pour le moment."
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
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
