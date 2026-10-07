use crate::state::WARNING_MESSAGE;
use redirect::Policy;
use reqwest::{header, redirect, Certificate, Client, Url};
use std::error::Error;

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

pub async fn resolve_tiktok_url(mut url: Url) -> dioxus::Result<Url, Box<dyn Error>> {
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

        if !response.status().is_redirection() {
            return Ok(response.url().clone());
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

pub fn clean_tracking_parameters(mut url: Url) -> (Url, Option<String>) {
    let mut detected_origin: Option<String> = None;
    let parameters: Vec<(String, String)> = url
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    let mut remaining_parameters = Vec::new();
    for (key, value) in parameters {
        if let Some(origin) = tracker_origin(&key) {
            if detected_origin.is_none() {
                detected_origin = Some(origin.to_string());
            }
            continue;
        }
        remaining_parameters.push((key, value));
    }
    if detected_origin.is_none() {
        *WARNING_MESSAGE.write() =
            "L'URL ne semble pas contenir de tracker dans ses paramètres.".to_string();
    } else {
        WARNING_MESSAGE.write().clear();
    }
    url.set_query(None);
    if !remaining_parameters.is_empty() {
        let mut query = url.query_pairs_mut();
        for (key, value) in remaining_parameters {
            query.append_pair(&key, &value);
        }
    }
    url.set_fragment(None);
    (url, detected_origin)
}

pub fn tracker_origin(parameter: &str) -> Option<&'static str> {
    match parameter {
        // YouTube
        "?si" => Some("YouTube"),
        // TikTok
        "_t" | "_r" | "ttclid" => Some("TikTok"),
        // Meta
        "fbclid" => Some("Facebook"),
        "igshid" | "stkn" => Some("Instagram"),
        // Google
        "gclid" | "gbraid" | "wbraid" | "dclid" => Some("Google"),
        // Microsoft
        "msclkid" => Some("Microsoft"),
        // Twitter / X
        "twclid" => Some("Twitter/X"),
        // Yandex
        "yclid" => Some("Yandex"),
        // Mailchimp
        "mc_eid" => Some("Mailchimp"),
        // OpenStat
        "_openstat" => Some("Yandex/VK"),
        // UTM
        "utm_source"
        | "utm_medium"
        | "utm_campaign"
        | "utm_term"
        | "utm_content"
        | "utm_id"
        | "utm_source_platform"
        | "utm_creative_format"
        | "utm_marketing_tactic" => Some("UTM"),
        _ => None,
    }
}
