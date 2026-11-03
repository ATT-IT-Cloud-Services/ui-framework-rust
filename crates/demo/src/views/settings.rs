use dioxus::prelude::*;
use ui_framework::layouts::settings_layout::{SettingsLayout, SettingsTabItem};
use ui_framework::settings::provider::use_layout_settings;
use ui_framework::settings::types::{Density, FontFamilyPreset};
use ui_framework::ui::switch::Switch;

#[component]
pub fn SettingsView() -> Element {
    let mut settings = use_layout_settings();
    let mut active_tab = use_signal(|| "appearance".to_string());

    let tabs = vec![
        SettingsTabItem { id: "appearance".to_string(), label: "Appearance & Theme".to_string(), badge: None },
        SettingsTabItem { id: "typography".to_string(), label: "Typography & Fonts".to_string(), badge: None },
        SettingsTabItem { id: "profile".to_string(), label: "User Profile".to_string(), badge: None },
        SettingsTabItem { id: "notifications".to_string(), label: "Alert Notifications".to_string(), badge: None },
        SettingsTabItem { id: "security".to_string(), label: "Security & Roles".to_string(), badge: None },
    ];

    rsx! {
        SettingsLayout {
            title: "Application & Workspace Settings".to_string(),
            subtitle: Some("Customize density, typography, theme preferences, and security permissions.".to_string()),
            tabs: tabs,
            active_tab_id: active_tab(),
            on_select_tab: move |id| active_tab.set(id),

            if active_tab() == "appearance" {
                div { class: "space-y-4 text-xs",
                    h3 { class: "text-sm font-semibold pb-2 border-b border-border", "Appearance & Ergonomics" }
                    div { class: "grid grid-cols-2 gap-4",
                        div { class: "space-y-1.5",
                            span { class: "text-2xs font-semibold text-muted-foreground uppercase", "Layout Density" }
                            div { class: "flex gap-2",
                                button {
                                    r#type: "button",
                                    class: if settings.read().density == Density::Comfortable { "px-3 py-1.5 rounded bg-primary text-primary-foreground font-semibold" } else { "px-3 py-1.5 rounded border border-border" },
                                    onclick: move |_| settings.write().density = Density::Comfortable,
                                    "Comfortable"
                                }
                                button {
                                    r#type: "button",
                                    class: if settings.read().density == Density::Compact { "px-3 py-1.5 rounded bg-primary text-primary-foreground font-semibold" } else { "px-3 py-1.5 rounded border border-border" },
                                    onclick: move |_| settings.write().density = Density::Compact,
                                    "Compact"
                                }
                            }
                        }
                    }

                    div { class: "flex items-center justify-between pt-2",
                        div {
                            span { class: "font-medium text-foreground block", "Sticky Header (Navbar)" }
                            span { class: "text-2xs text-muted-foreground", "Keep top navigation bar fixed during scroll" }
                        }
                        Switch {
                            checked: settings.read().header_sticky,
                            onchange: move |c| settings.write().header_sticky = c,
                        }
                    }

                    div { class: "flex items-center justify-between",
                        div {
                            span { class: "font-medium text-foreground block", "Sticky Sidebar (Navigation)" }
                            span { class: "text-2xs text-muted-foreground", "Pin sidebar to viewport height" }
                        }
                        Switch {
                            checked: settings.read().sidebar_sticky,
                            onchange: move |c| settings.write().sidebar_sticky = c,
                        }
                    }
                }
            } else if active_tab() == "typography" {
                div { class: "space-y-4 text-xs",
                    h3 { class: "text-sm font-semibold pb-2 border-b border-border", "Typography & Font Engine" }
                    p { class: "text-2xs text-muted-foreground", "Set custom font families for interface body, headings, and data telemetry monospace." }

                    div { class: "space-y-2",
                        span { class: "text-2xs font-semibold text-muted-foreground uppercase", "Primary Body Font (Sans)" }
                        div { class: "grid grid-cols-3 gap-2",
                            for (font, label) in [
                                (FontFamilyPreset::Inter, "Inter"),
                                (FontFamilyPreset::Geist, "Geist"),
                                (FontFamilyPreset::Roboto, "Roboto"),
                                (FontFamilyPreset::Jakarta, "Jakarta"),
                                (FontFamilyPreset::IbmPlex, "IBM Plex"),
                                (FontFamilyPreset::System, "System UI"),
                            ] {
                                {
                                    let is_active = settings.read().typography.font_sans == font;
                                    rsx! {
                                        button {
                                            r#type: "button",
                                            class: if is_active {
                                                "py-1.5 px-2 rounded border border-primary bg-primary text-primary-foreground font-semibold text-center cursor-pointer"
                                            } else {
                                                "py-1.5 px-2 rounded border border-border hover:bg-muted text-foreground text-center cursor-pointer"
                                            },
                                            onclick: move |_| settings.write().typography.font_sans = font.clone(),
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "space-y-3 text-xs text-muted-foreground",
                    h3 { class: "text-sm font-semibold text-foreground pb-2 border-b border-border", "Workspace Properties" }
                    p { "Role privileges, telemetry subscriptions, and security rules are active." }
                }
            }
        }
    }
}
