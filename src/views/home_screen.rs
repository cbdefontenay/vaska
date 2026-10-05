use dioxus::prelude::*;

#[component]
pub fn HomeScreen() -> Element {
    let mut url = use_signal(|| String::new());
    let mut cleaned_url = use_signal(String::new);
    let mut tracker_origin_name = use_signal(String::new);

    let mut check_if_url_contains_trackers = move |url: &str| {
        let trackers = [
            "?si",
            "&si",
            "?utm_source",
            "&utm_source",
            "?utm_medium",
            "&utm_medium",
            "?utm_campaign",
            "&utm_campaign",
            "?utm_term",
            "&utm_term",
            "?fbclid",
            "&fbclid",
            "?igshid",
            "&igshid",
            "?gclid",
            "&gclid",
            "&stkn",
            "?stkn",
            "?gbraid",
            "&gbraid",
            "?wbraid",
            "&wbraid",
            "?dclid",
            "&dclid",
            "?msclkid",
            "&msclkid",
            "?ttclid",
            "&ttclid",
            "?twclid",
            "&twclid",
            "?yclid",
            "&yclid",
            "?mc_eid",
            "&mc_eid",
            "?_openstat",
            "&_openstat",
        ];

        for tracker in trackers {
            if let Some((clean_url, _)) = url.split_once(tracker) {
                match tracker {
                    "?si" | "&si" => {
                        tracker_origin_name.set("YouTube".to_string());
                    }
                    "?utm_source"
                    | "&utm_source"
                    | "?utm_medium"
                    | "&utm_medium"
                    | "?utm_campaign"
                    | "&utm_campaign"
                    | "?utm_term"
                    | "&utm_term" => {
                        tracker_origin_name.set("Article".to_string());
                    }
                    "?fbclid" | "&fbclid" => {
                        tracker_origin_name.set("Facebook".to_string());
                    }
                    "?igshid" | "&igshid" => {
                        tracker_origin_name.set("Instagram".to_string());
                    }
                    "?stkn" | "&stkn" => {
                        tracker_origin_name.set("Instagram".to_string());
                    }
                    "?gclid"
                    | "&gclid"
                    | "?gbraid"
                    | "&gbraid"
                    | "?wbraid"
                    | "&wbraid"
                    | "?dclid"
                    | "&dclid" => {
                        tracker_origin_name.set("Google".to_string());
                    }
                    "?msclkid" | "&msclkid" => {
                        tracker_origin_name.set("Microsoft".to_string());
                    }
                    "?ttclid" | "&ttclid" => {
                        tracker_origin_name.set("TikTok".to_string());
                    }
                    "?twclid" | "&twclid" => {
                        tracker_origin_name.set("Twitter/X".to_string());
                    }
                    "?yclid" | "&yclid" => {
                        tracker_origin_name.set("Yandex".to_string());
                    }
                    "?mc_eid" | "&mc_eid" => {
                        tracker_origin_name.set("Mailchimp".to_string());
                    }
                    "?_openstat" | "&_openstat" => {
                        tracker_origin_name.set("Yandex/VK".to_string());
                    }
                    _ => {}
                }

                return clean_url.to_string();
            }
        }

        tracker_origin_name.set(String::new());
        url.to_string()
    };

    rsx! {
        div {
            class: "min-h-screen w-full bg-surface px-5 pt-12 pb-8",

            div {
                class: "mb-8",

                h1 {
                    class: "text-3xl font-bold text-secondary leading-tight",
                    "Nettoyeur d'URL"
                }

                p {
                    class: "mt-2 text-sm text-secondary opacity-70",
                    "Supprime les paramètres de tracking de vos liens."
                }
            }

            div {
                class: "w-full",

                p {
                    class: "mb-2 text-sm font-medium text-secondary",
                    "Votre URL"
                }

                input {
                    class: "w-full h-14 rounded-2xl border border-primary bg-transparent px-4 text-base text-secondary outline-none",
                    placeholder: "Collez votre URL ici",
                    r#type: "url",
                    value: "{url}",

                    oninput: move |evt| {
                        url.set(evt.value());
                    },
                }

                button {
                    class: "mt-4 w-full h-14 rounded-2xl bg-tertiary text-scrim text-base font-semibold active:opacity-80",

                    onclick: move |_| {
                        let result = check_if_url_contains_trackers(&url());
                        cleaned_url.set(result);
                    },

                    "Nettoyer l'URL"
                }
            }

            if !cleaned_url().is_empty() {
                div {
                    class: "mt-8 w-full rounded-2xl border border-primary p-5",

                    div {
                        class: "flex items-center justify-between mb-3",

                        p {
                            class: "text-sm font-semibold text-secondary",
                            "URL nettoyée"
                        }

                        span {
                            class: "text-xs font-medium text-green-500",
                            "✓ Nettoyée"
                        }
                    }

                    p {
                        class: "select-all text-sm text-secondary break-all leading-relaxed",
                        "{cleaned_url}"
                    }

                    if !tracker_origin_name().is_empty() {
                        p {
                            class: "mt-3 text-xs text-secondary opacity-70",
                            "Tracking détecté : {tracker_origin_name}"
                        }
                    }
                }
            }
        }
    }
}