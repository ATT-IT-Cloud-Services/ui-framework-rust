use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SupportedLocale {
    #[default]
    En,
    De,
    Fr,
    Es,
    Ar,
}

impl SupportedLocale {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::En => "en",
            Self::De => "de",
            Self::Fr => "fr",
            Self::Es => "es",
            Self::Ar => "ar",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "de" => Self::De,
            "fr" => Self::Fr,
            "es" => Self::Es,
            "ar" => Self::Ar,
            _ => Self::En,
        }
    }

    pub fn native_name(&self) -> &'static str {
        match self {
            Self::En => "English",
            Self::De => "Deutsch",
            Self::Fr => "Français",
            Self::Es => "Español",
            Self::Ar => "العربية",
        }
    }

    pub fn flag(&self) -> &'static str {
        match self {
            Self::En => "🇺🇸",
            Self::De => "🇩🇪",
            Self::Fr => "🇫🇷",
            Self::Es => "🇪🇸",
            Self::Ar => "🇸🇦",
        }
    }

    pub fn direction(&self) -> TextDirection {
        match self {
            Self::Ar => TextDirection::Rtl,
            _ => TextDirection::Ltr,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextDirection {
    #[default]
    Ltr,
    Rtl,
}

impl TextDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}
