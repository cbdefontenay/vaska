use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const FR_JSON: &str = include_str!("../translations/fr.json");
const DE_JSON: &str = include_str!("../translations/de.json");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppLanguage {
    French,
    German,
}

impl AppLanguage {
    pub fn code(&self) -> &'static str {
        match self {
            AppLanguage::French => "fr",
            AppLanguage::German => "de",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code.trim().to_ascii_lowercase().as_str() {
            "fr" | "french" => Some(AppLanguage::French),
            "de" | "german" => Some(AppLanguage::German),
            _ => None,
        }
    }

    /// Primary path to save user preferences in JSON.
    fn primary_preference_path() -> PathBuf {
        #[cfg(target_os = "android")]
        {
            let data_dir = "/data/data/com.cyprien.vaska/files";
            let _ = fs::create_dir_all(data_dir);
            PathBuf::from(format!("{data_dir}/vaska_prefs.json"))
        }

        #[cfg(not(target_os = "android"))]
        {
            PathBuf::from("vaska_prefs.json")
        }
    }

    /// List of paths to check when loading saved preferences.
    fn candidate_preference_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();

        #[cfg(target_os = "android")]
        {
            paths.push(PathBuf::from("/data/data/com.cyprien.vaska/files/vaska_prefs.json"));
            paths.push(PathBuf::from("/data/data/com.cyprien.vaska/files/.vaska_prefs.json"));
        }

        #[cfg(not(target_os = "android"))]
        {
            paths.push(PathBuf::from("vaska_prefs.json"));
            paths.push(PathBuf::from(".vaska_prefs.json"));
        }

        paths
    }

    /// Load the saved language from a JSON file, or return None if not saved yet.
    pub fn load_saved() -> Option<Self> {
        for path in Self::candidate_preference_paths() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Some(lang) = parse_language_from_json(&content) {
                    return Some(lang);
                }
            }
        }
        None
    }

    /// Save the language preference to a JSON file.
    pub fn save(&self) {
        let path = Self::primary_preference_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let json = format!("{{\n  \"language\": \"{}\"\n}}\n", self.code());
        let _ = fs::write(&path, json);
    }

    /// Detect system language (German if device is in German, otherwise default to French).
    pub fn detect_system_language() -> Self {
        #[cfg(target_os = "android")]
        {
            let props = [
                "persist.sys.locale",
                "ro.product.locale",
                "persist.sys.language",
                "ro.product.locale.language",
            ];
            for prop in props {
                if let Some(val) = get_android_system_property(prop) {
                    if is_german_locale(&val) {
                        return AppLanguage::German;
                    } else if is_french_locale(&val) {
                        return AppLanguage::French;
                    }
                }
            }
        }

        let env_vars = ["LANG", "LC_ALL", "LC_MESSAGES", "LANGUAGE"];
        for var in env_vars {
            if let Ok(val) = std::env::var(var) {
                if is_german_locale(&val) {
                    return AppLanguage::German;
                } else if is_french_locale(&val) {
                    return AppLanguage::French;
                }
            }
        }

        AppLanguage::French
    }

    /// Get the initial language: saved preference > system detection > French.
    pub fn initial_language() -> Self {
        Self::load_saved().unwrap_or_else(Self::detect_system_language)
    }
}

/// Helper to read an Android system property using libc __system_property_get.
#[cfg(target_os = "android")]
fn get_android_system_property(key: &str) -> Option<String> {
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_int};

    extern "C" {
        fn __system_property_get(name: *const c_char, value: *mut c_char) -> c_int;
    }

    let c_key = CString::new(key).ok()?;
    let mut val_buffer = [0 as c_char; 92]; // PROP_VALUE_MAX on Android
    let len = unsafe { __system_property_get(c_key.as_ptr(), val_buffer.as_mut_ptr()) };
    if len > 0 {
        let c_str = unsafe { CStr::from_ptr(val_buffer.as_ptr()) };
        c_str.to_str().ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
    } else {
        None
    }
}

/// Returns true if the locale string indicates German.
fn is_german_locale(locale: &str) -> bool {
    let lower = locale.to_ascii_lowercase();
    lower.starts_with("de") || lower.contains("german")
}

/// Returns true if the locale string indicates French.
fn is_french_locale(locale: &str) -> bool {
    let lower = locale.to_ascii_lowercase();
    lower.starts_with("fr") || lower.contains("french")
}

/// Pure Rust JSON parser to extract language preference.
pub fn parse_language_from_json(content: &str) -> Option<AppLanguage> {
    let content = content.trim();
    if let Some(pos) = content.find("\"language\"") {
        let after_key = &content[pos + "\"language\"".len()..];
        if let Some(colon_pos) = after_key.find(':') {
            let after_colon = after_key[colon_pos + 1..].trim();
            let val = if let Some(first_quote) = after_colon.find('"') {
                let after_first_quote = &after_colon[first_quote + 1..];
                if let Some(second_quote) = after_first_quote.find('"') {
                    &after_first_quote[..second_quote]
                } else {
                    ""
                }
            } else {
                after_colon.trim_matches(|c| c == '}' || c == ',' || c == ' ' || c == '\n' || c == '\r')
            };

            return AppLanguage::from_code(val);
        }
    }
    None
}

#[derive(Debug, Clone)]
pub struct Translations {
    pub map: HashMap<String, String>,
}

impl Translations {
    fn from_json(json: &str) -> Self {
        let mut map = HashMap::new();
        let lines: Vec<&str> = json.lines().collect();

        for line in lines {
            let trimmed = line.trim();

            if trimmed.starts_with('"') {
                if let Some(colon_pos) = trimmed.find(':') {
                    let key_part = &trimmed[1..colon_pos];
                    if let Some(end_quote) = key_part.rfind('"') {
                        let key = key_part[..end_quote].to_string();

                        let after_colon = &trimmed[colon_pos + 1..];
                        let after_colon = after_colon.trim();
                        if after_colon.starts_with('"') {
                            if let Some(end_val) = after_colon.rfind('"') {
                                let value = after_colon[1..end_val].to_string();
                                map.insert(key, value);
                            }
                        }
                    }
                }
            }
        }

        Translations { map }
    }

    pub fn get(&self, key: &str) -> String {
        self.map.get(key).cloned().unwrap_or_else(|| key.to_string())
    }

    pub fn get_with_format(&self, key: &str, value: &str) -> String {
        let base = self.get(key);
        format!("{}{}", base, value)
    }
}

pub fn load_translations(lang: &AppLanguage) -> Translations {
    match lang {
        AppLanguage::French => Translations::from_json(FR_JSON),
        AppLanguage::German => Translations::from_json(DE_JSON),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_language_from_json() {
        assert_eq!(
            parse_language_from_json(r#"{"language": "de"}"#),
            Some(AppLanguage::German)
        );
        assert_eq!(
            parse_language_from_json(r#"{"language":"de"}"#),
            Some(AppLanguage::German)
        );
        assert_eq!(
            parse_language_from_json(r#"{"language": "fr"}"#),
            Some(AppLanguage::French)
        );
        assert_eq!(
            parse_language_from_json("{\n  \"language\": \"de\"\n}"),
            Some(AppLanguage::German)
        );
        assert_eq!(parse_language_from_json(r#"{"other": "value"}"#), None);
    }

    #[test]
    fn test_is_german_locale() {
        assert!(is_german_locale("de"));
        assert!(is_german_locale("de_DE"));
        assert!(is_german_locale("de-DE"));
        assert!(is_german_locale("de_AT.UTF-8"));
        assert!(is_german_locale("DE_CH"));
        assert!(!is_german_locale("fr_FR"));
        assert!(!is_german_locale("en_US"));
    }

    #[test]
    fn test_is_french_locale() {
        assert!(is_french_locale("fr"));
        assert!(is_french_locale("fr_FR"));
        assert!(is_french_locale("fr-FR"));
        assert!(!is_french_locale("de_DE"));
    }

    #[test]
    fn test_translations_loaded() {
        let fr = load_translations(&AppLanguage::French);
        assert_eq!(fr.get("home_title"), "Nettoyeur d'URL");

        let de = load_translations(&AppLanguage::German);
        assert_eq!(de.get("home_title"), "URL-Reiniger");
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        AppLanguage::German.save();
        let loaded = AppLanguage::load_saved();
        assert_eq!(loaded, Some(AppLanguage::German));

        AppLanguage::French.save();
        let loaded = AppLanguage::load_saved();
        assert_eq!(loaded, Some(AppLanguage::French));

        // Clean up preference file created in test
        let path = AppLanguage::primary_preference_path();
        let _ = fs::remove_file(path);
    }
}
