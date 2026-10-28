use crate::components::{AppHeader, LoadingSpinner, ResultCard, UrlInputField};
use crate::helper_functions::{
    clean_tracking_parameters, is_amazon_marketplace_short_url, is_deezer_short_url,
    is_facebook_short_url, is_google_maps_short_url, is_linkedin_short_url, is_tiktok_short_url,
    resolve_short_url,
};
use crate::i18n::use_lang;
use crate::icons::{FilterIcon, ErrorXIcon, AlertTriangleIcon};
use crate::state::WARNING_MESSAGE;

use dioxus::prelude::*;
use reqwest::Url;

#[component]
pub fn HomeScreen() -> Element {
    let lang = use_lang();
    let url = use_signal(String::new);
    let mut cleaned_url = use_signal(String::new);
    let mut tracker_origin_name = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let mut error_message = use_signal(String::new);

    rsx! {
        div { class: "min-h-screen w-full bg-surface px-5 pt-safe-12 pb-24",
            // Header avec effets visuels
            AppHeader {
                title: lang.t("home_title"),
                description: lang.t("home_description"),
            }

            // Champ de saisie URL amélioré
            UrlInputField { value: url, error_message }

            // Bouton de nettoyage avec icône
            button {
                class: "mt-6 w-full h-14 rounded-2xl bg-primary text-on-primary text-base font-semibold flex items-center justify-center gap-2 active:opacity-80 disabled:opacity-50 shadow-lg shadow-primary/20",
                disabled: is_loading(),

                onclick: move |_| {
                    let input_url = url();
                    let lang = lang.clone();

                    async move {
                        error_message.set(String::new());
                        cleaned_url.set(String::new());
                        tracker_origin_name.set(String::new());
                        WARNING_MESSAGE.write().clear();

                        if input_url.trim().is_empty() {
                            error_message.set(lang.t("please_enter_url"));
                            return;
                        }

                        let input = input_url.trim();

                        let normalized_url =
                            if input.starts_with("https://")
                            || input.starts_with("http://")
                        {
                            input.to_string()
                        } else {
                            format!("https://{}", input)
                        };

                        let parsed_url =
                            match Url::parse(&normalized_url) {
                            Ok(url) => url,
                            Err(_) => {
                                error_message.set(lang.t("invalid_url"));
                                return;
                            }
                        };

                        is_loading.set(true);

                        let is_tiktok = is_tiktok_short_url(&parsed_url);
                        let is_linkedin = is_linkedin_short_url(&parsed_url);
                        let is_google_maps = is_google_maps_short_url(&parsed_url);
                        let is_deezer = is_deezer_short_url(&parsed_url);
                        let is_amazon_marketplace = is_amazon_marketplace_short_url(&parsed_url);
                        let is_facebook = is_facebook_short_url(&parsed_url);

                        let final_url = if is_tiktok || is_linkedin || is_google_maps || is_deezer
                            || is_amazon_marketplace || is_facebook
                        {
                            match resolve_short_url(parsed_url.clone()).await {
                                Ok(url) => url,
                                Err(error) => {
                                    error_message
                                        .set(lang.t_with("cannot_resolve_url", &error.to_string()));
                                    is_loading.set(false);
                                    return;
                                }
                            }
                        } else {
                            parsed_url
                        };
                        let (cleaned, tracker_origin, has_no_trackers) = clean_tracking_parameters(final_url);
                        let tracker_origin = if is_linkedin {
                            Some("LinkedIn".to_string())
                        } else if is_tiktok {
                            Some("TikTok".to_string())
                        } else {
                            tracker_origin
                        };
                        // Set warning message based on whether trackers were found
                        if has_no_trackers && !is_linkedin && !is_tiktok {
                            *WARNING_MESSAGE.write() = lang.t("no_trackers");
                        } else {
                            WARNING_MESSAGE.write().clear();
                        }
                        cleaned_url.set(cleaned.to_string());
                        if let Some(origin) = tracker_origin {
                            tracker_origin_name.set(origin);
                        }
                        is_loading.set(false);
                    }
                },

                // Bouton contenu avec icône
                if is_loading() {
                    div { class: "flex items-center gap-2",
                        LoadingSpinner {}
                        {lang.t("cleaning")}
                    }
                } else {
                    div { class: "flex items-center gap-2",
                        FilterIcon { class: "w-5 h-5" }
                        {lang.t("clean_url")}
                    }
                }
            }

            // Message d'erreur amélioré
            if !error_message().is_empty() {
                div { class: "mt-6 w-full rounded-2xl bg-error-container/50 p-4 flex items-start gap-3",
                    div { class: "mt-0.5 w-5 h-5 rounded-full bg-error/20 flex-shrink-0 flex items-center justify-center",
                        ErrorXIcon { class: "w-3 h-3 text-error" }
                    }
                    p { class: "text-sm text-error", "{error_message}" }
                }
            }

            // Message d'avertissement amélioré
            if !WARNING_MESSAGE().is_empty() {
                div { class: "mt-6 w-full rounded-2xl bg-tertiary-container/50 p-4 flex items-start gap-3",
                    div { class: "mt-0.5 w-5 h-5 rounded-full bg-tertiary/20 flex-shrink-0 flex items-center justify-center",
                        AlertTriangleIcon { class: "w-3 h-3 text-tertiary" }
                    }
                    p { class: "text-sm text-on-tertiary-container", "{WARNING_MESSAGE}" }
                }
            }

            // Résultat URL nettoyée
            if !cleaned_url().is_empty() {
                ResultCard {
                    cleaned_url: cleaned_url(),
                    tracker_origin:
                    if tracker_origin_name().is_empty() {
                        None
                    } else {
                        Some(tracker_origin_name())
                    },
                    show_warning: !WARNING_MESSAGE().is_empty(),
                }
            }
        }
    }
}
