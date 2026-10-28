use chrono::{Datelike, Utc};
use crate::components::AppHeader;
use crate::i18n::{use_lang, AppLanguage};
use crate::icons::{InfoIcon, FilterIcon, CodeIcon, LockIcon, CheckmarkCircleIcon};
use dioxus::prelude::*;

#[component]
pub fn SettingsScreen() -> Element {
    let current_year = Utc::now().year();
    let lang = use_lang();
    let current_lang = (lang.language)();

    rsx! {
        div { class: "min-h-screen w-full bg-surface px-5 pt-safe-12 pb-24",
            // Header
            AppHeader {
                title: lang.t("settings"),
                description: lang.t("settings_description"),
            }

            // Section: Langue
            div { class: "mb-6",
                p { class: "text-xs font-semibold text-on-surface-variant uppercase tracking-wider mb-3 px-1",
                    {lang.t("language").to_uppercase()}
                }
                div { class: "rounded-2xl bg-surface-container-high overflow-hidden border border-outline/20",
                    // French option
                    div {
                        class: format!(
                            "flex items-center justify-between px-4 py-4 cursor-pointer {}",
                            if current_lang == AppLanguage::French { "bg-primary/10" } else { "" }
                        ),
                        onclick: move |_| {
                            lang.set_language(AppLanguage::French);
                        },
                        div { class: "flex items-center gap-3",
                            span { class: "text-lg", "🇫🇷" }
                            div {
                                p { class: "text-sm font-medium text-on-surface",
                                    {lang.t("french")}
                                }
                            }
                        }
                        if current_lang == AppLanguage::French {
                            div { class: "w-5 h-5 rounded-full bg-primary flex items-center justify-center",
                                span { class: "text-on-primary text-xs", "✓" }
                            }
                        }
                    }
                    div { class: "mx-4 h-px bg-outline/20" }
                    // German option
                    div {
                        class: format!(
                            "flex items-center justify-between px-4 py-4 cursor-pointer {}",
                            if current_lang == AppLanguage::German { "bg-primary/10" } else { "" }
                        ),
                        onclick: move |_| {
                            lang.set_language(AppLanguage::German);
                        },
                        div { class: "flex items-center gap-3",
                            span { class: "text-lg", "🇩🇪" }
                            div {
                                p { class: "text-sm font-medium text-on-surface",
                                    {lang.t("german")}
                                }
                            }
                        }
                        if current_lang == AppLanguage::German {
                            div { class: "w-5 h-5 rounded-full bg-primary flex items-center justify-center",
                                span { class: "text-on-primary text-xs", "✓" }
                            }
                        }
                    }
                }
            }

            // Section: À propos
            div { class: "mb-6",
                p { class: "text-xs font-semibold text-on-surface-variant uppercase tracking-wider mb-3 px-1",
                    {lang.t("about").to_uppercase()}
                }
                div { class: "rounded-2xl bg-surface-container-high overflow-hidden border border-outline/20",
                    // Version
                    div { class: "flex items-center justify-between px-4 py-4",
                        div { class: "flex items-center gap-3",
                            div { class: "w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center",
                                InfoIcon { class: "w-5 h-5 text-primary" }
                            }
                            div {
                                p { class: "text-sm font-medium text-on-surface",
                                    {lang.t("version")}
                                }
                                p { class: "text-xs text-on-surface-variant", "0.1.0" }
                            }
                        }
                    }
                    div { class: "mx-4 h-px bg-outline/20" }
                    // Description
                    div { class: "flex items-center justify-between px-4 py-4",
                        div { class: "flex items-center gap-3",
                            div { class: "w-10 h-10 rounded-xl bg-secondary/10 flex items-center justify-center",
                                FilterIcon { class: "w-5 h-5 text-secondary" }
                            }
                            div {
                                p { class: "text-sm font-medium text-on-surface",
                                    {lang.t("feature")}
                                }
                                p { class: "text-xs text-on-surface-variant",
                                    {lang.t("feature_description")}
                                }
                            }
                        }
                    }
                    div { class: "mx-4 h-px bg-outline/20" }
                    // Technologie
                    div { class: "flex items-center justify-between px-4 py-4",
                        div { class: "flex items-center gap-3",
                            div { class: "w-10 h-10 rounded-xl bg-tertiary/10 flex items-center justify-center",
                                CodeIcon { class: "w-5 h-5 text-tertiary" }
                            }
                            div {
                                p { class: "text-sm font-medium text-on-surface",
                                    {lang.t("technology")}
                                }
                                p { class: "text-xs text-on-surface-variant", {lang.t("tech_description")} }
                            }
                        }
                    }
                }
            }

            // Section: Suivi détecté
            div { class: "mb-6",
                p { class: "text-xs font-semibold text-on-surface-variant uppercase tracking-wider mb-3 px-1",
                    {lang.t("tracking_detected_section").to_uppercase()}
                }
                div { class: "rounded-2xl bg-surface-container-high overflow-hidden border border-outline/20",
                    // Plateformes supportées
                    div { class: "px-4 py-4",
                        p { class: "text-sm text-on-surface mb-3", {lang.t("supported_platforms")} }
                        div { class: "flex flex-wrap gap-2",
                            span { class: "px-3 py-1.5 rounded-lg bg-primary/10 text-xs font-medium text-primary",
                                {lang.t("platform_tiktok")}
                            }
                            span { class: "px-3 py-1.5 rounded-lg bg-secondary/10 text-xs font-medium text-secondary",
                                {lang.t("platform_linkedin")}
                            }
                            span { class: "px-3 py-1.5 rounded-lg bg-tertiary/10 text-xs font-medium text-tertiary",
                                {lang.t("platform_google_maps")}
                            }
                            span { class: "px-3 py-1.5 rounded-lg bg-primary/10 text-xs font-medium text-primary",
                                {lang.t("platform_deezer")}
                            }
                            span { class: "px-3 py-1.5 rounded-lg bg-secondary/10 text-xs font-medium text-secondary",
                                {lang.t("platform_amazon")}
                            }
                            span { class: "px-3 py-1.5 rounded-lg bg-tertiary/10 text-xs font-medium text-tertiary",
                                {lang.t("platform_facebook")}
                            }
                            span { class: "px-3 py-1.5 rounded-lg bg-primary/10 text-xs font-medium text-error",
                                {lang.t("platform_ads")}
                            }
                        }
                    }
                }
            }

            // Section: Données
            div { class: "mb-6",
                p { class: "text-xs font-semibold text-on-surface-variant uppercase tracking-wider mb-3 px-1",
                    {lang.t("data").to_uppercase()}
                }
                div { class: "rounded-2xl bg-surface-container-high overflow-hidden border border-outline/20",
                    div { class: "flex items-center gap-3 px-4 py-4",
                        div { class: "w-10 h-10 rounded-xl bg-surface-variant flex items-center justify-center",
                            LockIcon { class: "w-5 h-5 text-on-surface-variant" }
                        }
                        div {
                            p { class: "text-sm font-medium text-on-surface", {lang.t("local_storage")} }
                            p { class: "text-xs text-on-surface-variant",
                                {lang.t("local_storage_description")}
                            }
                        }
                    }
                }
            }

            // Footer
            div { class: "mt-12 text-center",
                div { class: "flex items-center justify-center gap-2 mb-2",
                    CheckmarkCircleIcon { class: "w-6 h-6 text-primary" }
                    p { class: "text-sm font-medium text-on-surface", "Vaska" }
                }
                p { class: "text-xs text-on-surface-variant", {lang.t("privacy_slogan")} }
                p { class: "text-xs text-on-surface-variant mt-1 opacity-50",
                    {format!("©{} {}", current_year, lang.t("rights_reserved"))}
                }
            }
        }
    }
}
