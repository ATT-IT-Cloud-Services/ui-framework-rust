use std::collections::HashMap;
use dioxus::prelude::*;
use super::types::{SupportedLocale, TextDirection};

const EN_JSON: &str = include_str!("messages/en.json");
const DE_JSON: &str = include_str!("messages/de.json");
const FR_JSON: &str = include_str!("messages/fr.json");
const ES_JSON: &str = include_str!("messages/es.json");
const AR_JSON: &str = include_str!("messages/ar.json");

#[derive(Clone)]
pub struct I18nContext {
    pub locale: Signal<SupportedLocale>,
    catalogs: HashMap<SupportedLocale, HashMap<String, String>>,
}

impl I18nContext {
    pub fn new(initial_locale: SupportedLocale) -> Self {
        let mut catalogs = HashMap::new();
        catalogs.insert(SupportedLocale::En, serde_json::from_str(EN_JSON).unwrap_or_default());
        catalogs.insert(SupportedLocale::De, serde_json::from_str(DE_JSON).unwrap_or_default());
        catalogs.insert(SupportedLocale::Fr, serde_json::from_str(FR_JSON).unwrap_or_default());
        catalogs.insert(SupportedLocale::Es, serde_json::from_str(ES_JSON).unwrap_or_default());
        catalogs.insert(SupportedLocale::Ar, serde_json::from_str(AR_JSON).unwrap_or_default());

        Self {
            locale: Signal::new(initial_locale),
            catalogs,
        }
    }

    pub fn t(&self, key: &str) -> String {
        let current = (self.locale)();
        if let Some(dict) = self.catalogs.get(&current) {
            if let Some(val) = dict.get(key) {
                return val.clone();
            }
        }
        // Fallback to English
        if let Some(dict) = self.catalogs.get(&SupportedLocale::En) {
            if let Some(val) = dict.get(key) {
                return val.clone();
            }
        }
        key.to_string()
    }

    pub fn direction(&self) -> TextDirection {
        (self.locale)().direction()
    }
}

pub fn use_i18n() -> I18nContext {
    use_context::<I18nContext>()
}

pub fn use_t() -> impl Fn(&str) -> String {
    let i18n = use_i18n();
    move |key: &str| i18n.t(key)
}

#[component]
pub fn I18nProvider(
    #[props(default)] initial: SupportedLocale,
    children: Element,
) -> Element {
    let ctx = use_hook(|| I18nContext::new(initial));
    use_context_provider(|| ctx);

    rsx! {
        {children}
    }
}
