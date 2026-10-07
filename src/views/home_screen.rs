use crate::components::{ClearFieldButton, SystemBarColor, SystemBars};
use crate::helper_functions::{clean_tracking_parameters, is_tiktok_short_url, resolve_tiktok_url};
use crate::state::WARNING_MESSAGE;
use dioxus::prelude::*;
use reqwest::Url;

#[component]
pub fn HomeScreen() -> Element {
    let mut url = use_signal(String::new);
    let mut cleaned_url = use_signal(String::new);
    let mut tracker_origin_name = use_signal(String::new);
    let mut is_loading = use_signal(|| false);
    let mut error_message = use_signal(String::new);

    rsx! {
        SystemBars { color: SystemBarColor::Scrim }
        div { class: "min-h-screen w-full bg-surface px-5 pt-safe-12 pb-24",
            div { class: "mb-8",
                h1 { class: "text-3xl font-bold text-secondary leading-tight", "Nettoyeur d'URL" }
                p { class: "mt-2 text-sm text-secondary opacity-70",
                    "Supprime les paramètres de tracking de vos liens."
                }
            }
            div { class: "w-full",
                p { class: "mb-2 text-sm font-medium text-secondary", "Votre URL" }
                div {
                    class: "relative w-full",

                    input {
                        class: "w-full h-14 rounded-2xl border border-primary bg-transparent px-4 pr-12 text-base text-secondary outline-none",
                        placeholder: "Collez votre URL ici",
                        r#type: "url",
                        value: "{url}",

                        oninput: move |evt| {
                            url.set(evt.value());
                            error_message.set(String::new());
                            WARNING_MESSAGE.write().clear();
                        },
                    }

                    if !url().is_empty() {
                        ClearFieldButton {
                            value: url,
                        }
                    }
                }

                button {
                    class: "mt-4 w-full h-14 rounded-2xl bg-tertiary text-scrim text-base font-semibold active:opacity-80 disabled:opacity-50",
                    disabled: is_loading(),
                    onclick: move |_| {
                        let input_url = url();
                        async move {
                            error_message.set(String::new());
                            cleaned_url.set(String::new());
                            tracker_origin_name.set(String::new());
                            WARNING_MESSAGE.write().clear();
                            if input_url.trim().is_empty() {
                                error_message.set("Veuillez saisir une URL.".to_string());
                                return;
                            }
                            let input = input_url.trim();
                            let normalized_url = if input.starts_with("https://")
                                || input.starts_with("http://")
                            {
                                input.to_string()
                            } else {
                                format!("https://{}", input)
                            };
                            let parsed_url = match Url::parse(&normalized_url) {
                                Ok(url) => url,
                                Err(_) => {
                                    error_message.set("L'URL n'est pas valide.".to_string());
                                    return;
                                }
                            };
                            is_loading.set(true);
                            let final_url = if is_tiktok_short_url(&parsed_url) {
                                match resolve_tiktok_url(parsed_url.clone()).await {
                                    Ok(url) => url,
                                    Err(error) => {
                                        error_message
                                            .set(
                                                format!("Impossible de résoudre l'URL TikTok : {error}"),
                                            );
                                        is_loading.set(false);
                                        return;
                                    }
                                }
                            } else {
                                parsed_url
                            };
                            let (cleaned, tracker_origin) = clean_tracking_parameters(final_url);
                            cleaned_url.set(cleaned.to_string());
                            if let Some(origin) = tracker_origin {
                                tracker_origin_name.set(origin);
                            }
                            is_loading.set(false);
                        }
                    },
                    if is_loading() {
                        "Nettoyage..."
                    } else {
                        "Nettoyer l'URL"
                    }
                }
            }
            if !error_message().is_empty() {
                div { class: "mt-6 w-full rounded-2xl border border-red-500 p-4",
                    p { class: "text-sm text-red-500", "{error_message}" }
                }
            }
            if !WARNING_MESSAGE().is_empty() {
                div { class: "mt-6 w-full rounded-2xl border border-orange-500 p-4",
                    p { class: "text-sm text-orange-500", "{WARNING_MESSAGE}" }
                }
            }
            if !cleaned_url().is_empty() {
                if WARNING_MESSAGE().is_empty() {
                    div { class: "flex flex-col mt-8 w-full rounded-2xl border border-primary p-5",
                        div { class: "items-center justify-between",
                            span { class: "text-xs font-medium text-green-500", "✓ Nettoyé" }
                            p { class: "select-all text-sm text-secondary break-all leading-relaxed",
                                "{cleaned_url}"
                            }
                        }
                    }
                    if !tracker_origin_name().is_empty() {
                        p { class: "mt-3 text-xs text-secondary opacity-70",
                            "Tracking détecté : {tracker_origin_name}"
                        }
                    }
                }
            }
        }
    }
}
