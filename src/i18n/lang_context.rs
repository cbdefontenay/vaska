use dioxus::prelude::*;
use crate::i18n::{AppLanguage, Translations, load_translations};

/// Context that holds the current language and translations.
#[derive(Clone, Copy)]
pub struct LangContext {
    pub language: Signal<AppLanguage>,
    pub translations: Signal<Translations>,
}

impl LangContext {
    pub fn t(&self, key: &str) -> String {
        (self.translations)().get(key)
    }

    pub fn t_with(&self, key: &str, value: &str) -> String {
        (self.translations)().get_with_format(key, value)
    }

    pub fn set_language(&self, new_lang: AppLanguage) {
        new_lang.save();
        let mut language = self.language;
        let mut translations = self.translations;
        language.set(new_lang.clone());
        translations.set(load_translations(&new_lang));
    }
}

/// Provide the language context to all children.
#[component]
pub fn LangContextProvider(children: Element) -> Element {
    // Load saved language preference, or detect system language
    let initial_lang = AppLanguage::initial_language();
    let language = use_signal(|| initial_lang.clone());
    let translations = use_signal(|| load_translations(&initial_lang));

    let ctx = LangContext {
        language,
        translations,
    };

    use_context_provider(|| ctx);
    children
}

/// Get the current language context from any child component.
pub fn use_lang() -> LangContext {
    use_context()
}
