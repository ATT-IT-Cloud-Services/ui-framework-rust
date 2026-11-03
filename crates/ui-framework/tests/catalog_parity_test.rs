use std::collections::HashMap;

const EN_JSON: &str = include_str!("../src/i18n/messages/en.json");
const DE_JSON: &str = include_str!("../src/i18n/messages/de.json");
const FR_JSON: &str = include_str!("../src/i18n/messages/fr.json");
const ES_JSON: &str = include_str!("../src/i18n/messages/es.json");
const AR_JSON: &str = include_str!("../src/i18n/messages/ar.json");

#[test]
fn test_catalogs_key_parity() {
    let en: HashMap<String, String> = serde_json::from_str(EN_JSON).expect("valid EN json");
    let de: HashMap<String, String> = serde_json::from_str(DE_JSON).expect("valid DE json");
    let fr: HashMap<String, String> = serde_json::from_str(FR_JSON).expect("valid FR json");
    let es: HashMap<String, String> = serde_json::from_str(ES_JSON).expect("valid ES json");
    let ar: HashMap<String, String> = serde_json::from_str(AR_JSON).expect("valid AR json");

    let mut en_keys: Vec<_> = en.keys().cloned().collect();
    en_keys.sort();

    let mut de_keys: Vec<_> = de.keys().cloned().collect();
    de_keys.sort();

    let mut fr_keys: Vec<_> = fr.keys().cloned().collect();
    fr_keys.sort();

    let mut es_keys: Vec<_> = es.keys().cloned().collect();
    es_keys.sort();

    let mut ar_keys: Vec<_> = ar.keys().cloned().collect();
    ar_keys.sort();

    assert_eq!(de_keys, en_keys, "German catalog must match English keys");
    assert_eq!(fr_keys, en_keys, "French catalog must match English keys");
    assert_eq!(es_keys, en_keys, "Spanish catalog must match English keys");
    assert_eq!(ar_keys, en_keys, "Arabic catalog must match English keys");
}
