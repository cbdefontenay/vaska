use crate::components::ActionButton;
use dioxus::prelude::*;

/// Copie l'URL nettoyée dans le presse-papiers.
#[component]
pub fn CopyUrlButton(value: String, class: Option<String>) -> Element {
    rsx! {
        ActionButton {
            label: "Copier l'URL nettoyée".to_string(),
            class,
            disabled: false,
            onclick: move |_| copy_to_clipboard(value.clone()),
        }
    }
}

#[cfg(target_os = "android")]
pub fn copy_to_clipboard(value: String) {
    use dioxus::mobile::wry::prelude::{dispatch, JObject};

    dispatch(move |env, activity, _webview| {
        let Ok(service_name) = env.new_string("clipboard") else {
            return;
        };
        let service_name = JObject::from(service_name);
        let Ok(clipboard) = env
            .call_method(
                activity,
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[(&service_name).into()],
            )
            .and_then(|service| service.l())
        else {
            return;
        };

        let Ok(label) = env.new_string("Vaska") else {
            return;
        };
        let label = JObject::from(label);
        let Ok(text) = env.new_string(value) else {
            return;
        };
        let text = JObject::from(text);
        let Ok(clip) = env
            .call_static_method(
                "android/content/ClipData",
                "newPlainText",
                "(Ljava/lang/CharSequence;Ljava/lang/CharSequence;)Landroid/content/ClipData;",
                &[(&label).into(), (&text).into()],
            )
            .and_then(|clip| clip.l())
        else {
            return;
        };

        let _ = env.call_method(
            &clipboard,
            "setPrimaryClip",
            "(Landroid/content/ClipData;)V",
            &[(&clip).into()],
        );
    });
}

#[cfg(not(target_os = "android"))]
pub fn copy_to_clipboard(_value: String) {}
