use redirect::Policy;
use reqwest::{header, redirect, Certificate, Client, Url};
use scraper::{Html, Selector};
use std::error::Error;

pub fn is_facebook_short_url(url: &Url) -> bool {
    matches!(url.host_str(), Some("facebook.com" | "www.facebook.com"))
}

pub fn is_amazon_marketplace_short_url(url: &Url) -> bool {
    matches!(
        url.host_str(),
        Some("amzn.eu" | "www.amzn.eu" | "amzn.com" | "www.amzn.com")
    )
}

pub fn is_deezer_short_url(url: &Url) -> bool {
    matches!(
        url.host_str(),
        Some("link.deezer.com" | "www.link.deezer.com")
    )
}

pub fn is_linkedin_short_url(url: &Url) -> bool {
    matches!(url.host_str(), Some("lnkd.in" | "www.lnkd.in"))
}

pub fn is_google_maps_short_url(url: &Url) -> bool {
    matches!(url.host_str(), Some("maps.app.goo.gl" | "goo.gl"))
}

fn extract_linkedin_destination(html: &str) -> Option<Url> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href]").ok()?;

    for element in document.select(&selector) {
        let Some(href) = element.value().attr("href") else {
            continue;
        };

        let Ok(url) = Url::parse(href) else {
            continue;
        };

        if !matches!(url.scheme(), "http" | "https") {
            continue;
        }

        let Some(host) = url.host_str() else {
            continue;
        };

        // Ignore LinkedIn's own links
        if host == "linkedin.com"
            || host.ends_with(".linkedin.com")
            || host == "lnkd.in"
            || host.ends_with(".lnkd.in")
        {
            continue;
        }

        return Some(url);
    }

    None
}

pub fn is_tiktok_short_url(url: &Url) -> bool {
    let Some(host) = url.host_str() else {
        return false;
    };
    // Shortlinks directs
    if host == "vm.tiktok.com" || host == "vt.tiktok.com" {
        return true;
    }
    // Format https://www.tiktok.com/t/XXXX
    if host == "tiktok.com" || host == "www.tiktok.com" {
        return url.path().starts_with("/t/");
    }
    false
}

pub async fn resolve_short_url(mut url: Url) -> dioxus::Result<Url, Box<dyn Error>> {
    // Sur Android, le verifier de plateformes (rustls-platform-verifier) exige une
    // initialisation JNI jamais faite ici et panique au premier handshake TLS.
    // On fournit donc les racines webpki directement au client.
    let client = Client::builder()
        .redirect(Policy::none())
        .referer(false)
        .timeout(std::time::Duration::from_secs(10))
        .tls_certs_only(
            webpki_root_certs::TLS_SERVER_ROOT_CERTS
                .iter()
                .map(|cert| Certificate::from_der(cert.as_ref()).expect("racine webpki invalide")),
        )
        .build()?;

    for _ in 0..10 {
        let response = client
            .get(url.clone())
            .header(
                header::USER_AGENT,
                "Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 \
                 Chrome/140.0 Mobile Safari/537.36",
            )
            .send()
            .await?;

        if response.status().is_success() {
            let response_url = response.url().clone();

            if is_linkedin_short_url(&response_url) {
                let html = response.text().await?;

                if let Some(destination) = extract_linkedin_destination(&html) {
                    return Ok(destination);
                }

                return Err("Impossible de trouver l'URL finale dans la page LinkedIn.".into());
            }

            return Ok(response_url);
        }

        if !response.status().is_redirection() {
            return Err(format!("Réponse HTTP inattendue : {}", response.status()).into());
        }

        let location = response
            .headers()
            .get(header::LOCATION)
            .ok_or("Redirection sans header Location")?
            .to_str()?;

        url = url.join(location)?;
    }

    Err("Trop de redirections".into())
}

pub fn clean_tracking_parameters(mut url: Url) -> (Url, Option<String>, bool) {
    let mut detected_origin: Option<String> = None;
    let host = url.host_str().unwrap_or("").to_string();
    let parameters: Vec<(String, String)> = url
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    let mut remaining_parameters = Vec::new();
    for (key, value) in parameters {
        if let Some(origin) = tracker_origin(&key, &host) {
            if detected_origin.is_none() {
                detected_origin = Some(origin.to_string());
            }
            continue;
        }
        remaining_parameters.push((key, value));
    }
    let has_no_trackers = detected_origin.is_none();
    url.set_query(None);
    if !remaining_parameters.is_empty() {
        let mut query = url.query_pairs_mut();
        for (key, value) in remaining_parameters {
            query.append_pair(&key, &value);
        }
    }
    url.set_fragment(None);
    (url, detected_origin, has_no_trackers)
}

pub fn tracker_origin(parameter: &str, host: &str) -> Option<&'static str> {
    match parameter {
        // YouTube / Spotify
        "si" | "playnext" => match host {
            "open.spotify.com" => Some("Spotify"),
            "youtube.com" | "www.youtube.com" | "m.youtube.com" | "music.youtube.com"
            | "youtu.be" => Some("YouTube"),
            _ => Some("Inconnu"),
        },
        // TikTok
        "_t" | "_r" | "ttclid" => Some("TikTok"),
        "li_fat_id" => Some("LinkedIn"),
        // Meta
        "fbclid" => Some("Facebook"),
        "igshid" | "stkn" | "obrf" | "is" => Some("Instagram"),
        // Google
        "gclid" | "gbraid" | "wbraid" | "dclid" => Some("Google"),
        // Microsoft
        "msclkid" => Some("Microsoft"),
        // Twitter / X
        "twclid" => Some("Twitter/X"),
        // Google Maps
        "entry" | "g_ep" => Some("Google Maps"),
        // Yandex
        "yclid" => Some("Yandex"),
        // Mailchimp
        "mc_eid" => Some("Mailchimp"),
        // OpenStat
        "_openstat" => Some("Yandex/VK"),
        "_encoding" | "pd_rd_w" | "content-id" | "pd_rd_r" | "ref_" | "social_share" => {
            Some("Amazon")
        }
        "marketplaceId" | "musicTerritory" | "ref" => match host {
            "music.amazon.com" => Some("Amazon Music"),
            "amazon.com" => Some("Amazon"),
            _ => Some("Inconnu"),
        },
        // UTM
        "utm_source"
        | "utm_medium"
        | "utm_campaign"
        | "utm_term"
        | "utm_content"
        | "utm_id"
        | "utm_source_platform"
        | "utm_creative_format"
        | "utm_marketing_tactic" => match host {
            "kleinanzeigen.de" | "www.kleinanzeigen.de" => Some("Kleinanzeigen"),
            _ => Some("UTM/Marketing"),
        },
        _ => None,
    }
}
