use super::types::{FontFamilyPreset, MonoFontPreset};

pub struct FontPresetDef {
    pub name: &'static str,
    pub value: &'static str,
}

pub fn get_sans_font_definition(preset: &FontFamilyPreset) -> FontPresetDef {
    match preset {
        FontFamilyPreset::Inter => FontPresetDef {
            name: "Inter",
            value: "Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
        },
        FontFamilyPreset::Geist => FontPresetDef {
            name: "Geist Sans",
            value: "'Geist', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
        },
        FontFamilyPreset::Roboto => FontPresetDef {
            name: "Roboto",
            value: "Roboto, -apple-system, BlinkMacSystemFont, sans-serif",
        },
        FontFamilyPreset::Jakarta => FontPresetDef {
            name: "Plus Jakarta Sans",
            value: "'Plus Jakarta Sans', sans-serif",
        },
        FontFamilyPreset::IbmPlex => FontPresetDef {
            name: "IBM Plex Sans",
            value: "'IBM Plex Sans', -apple-system, sans-serif",
        },
        FontFamilyPreset::System => FontPresetDef {
            name: "Native System UI",
            value: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
        },
        FontFamilyPreset::Custom(_) => FontPresetDef {
            name: "Custom Font",
            value: "inherit",
        },
    }
}

pub fn get_mono_font_definition(preset: &MonoFontPreset) -> FontPresetDef {
    match preset {
        MonoFontPreset::Jetbrains => FontPresetDef {
            name: "JetBrains Mono",
            value: "'JetBrains Mono', ui-monospace, SFMono-Regular, monospace",
        },
        MonoFontPreset::GeistMono => FontPresetDef {
            name: "Geist Mono",
            value: "'Geist Mono', ui-monospace, SFMono-Regular, monospace",
        },
        MonoFontPreset::FiraCode => FontPresetDef {
            name: "Fira Code",
            value: "'Fira Code', ui-monospace, monospace",
        },
        MonoFontPreset::IbmPlexMono => FontPresetDef {
            name: "IBM Plex Mono",
            value: "'IBM Plex Mono', monospace",
        },
        MonoFontPreset::SystemMono => FontPresetDef {
            name: "System Monospace",
            value: "ui-monospace, SFMono-Regular, monospace",
        },
        MonoFontPreset::Custom(_) => FontPresetDef {
            name: "Custom Monospace",
            value: "monospace",
        },
    }
}
