use dioxus::prelude::*;
use crate::i18n::language_switcher::LanguageSwitcher;
use crate::settings::provider::{use_is_dark, use_layout_settings};
use crate::config::types::ThemeMode;

/// The application header: brand, actions, a search trigger, the language switcher and
/// the theme toggle.
///
/// Every visible label can be overridden, so a host application keeps its own language
/// instead of the framework's English defaults. Controls an application already renders
/// itself can be switched off. Icons are inline SVG — no emoji, no icon font.
#[component]
pub fn Header(
    #[props(default = "")] class: &'static str,
    logo_slot: Option<Element>,
    actions_slot: Option<Element>,
    user_menu_slot: Option<Element>,
    on_search: Option<EventHandler<()>>,
    /// Optional mobile navigation trigger; renders a hamburger below `lg`.
    on_menu: Option<EventHandler<()>>,
    /// Label of the search trigger.
    #[props(default = "Search".to_string())] search_label: String,
    /// Shortcut hint next to the label, e.g. `Ctrl+K`.
    #[props(default = "Ctrl+K".to_string())] search_shortcut: String,
    /// Tooltip and accessible name of the theme toggle.
    #[props(default = "Toggle theme".to_string())] theme_label: String,
    /// Accessible name of the language switcher.
    #[props(default = "Change language".to_string())] language_label: String,
    /// Accessible name of the mobile menu trigger.
    #[props(default = "Open navigation".to_string())] menu_label: String,
    /// Hide the search trigger when the application has its own.
    #[props(default = false)] hide_search: bool,
    /// Hide the language switcher.
    #[props(default = false)] hide_language: bool,
    /// Hide the theme toggle.
    #[props(default = false)] hide_theme: bool,
) -> Element {
    let mut settings = use_layout_settings();

    let is_sticky = settings.read().header_sticky;
    let is_dark = use_is_dark();

    rsx! {
        header {
            class: if is_sticky {
                "flex h-12 w-full items-center justify-between border-b border-border bg-background/95 px-3 sm:px-4 backdrop-blur-md select-none sticky top-0 z-[300] {class}"
            } else {
                "flex h-12 w-full items-center justify-between border-b border-border bg-background px-3 sm:px-4 select-none {class}"
            },

            // Brand / Logo Slot
            div { class: "flex items-center gap-3",
                if let Some(handler) = on_menu {
                    button {
                        r#type: "button",
                        class: "inline-flex lg:hidden h-8 w-8 items-center justify-center rounded-md text-muted-foreground hover:bg-muted hover:text-foreground cursor-pointer",
                        aria_label: "{menu_label}",
                        onclick: move |_| handler.call(()),
                        svg {
                            class: "w-4 h-4",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "M4 7h16M4 12h16M4 17h16" }
                        }
                    }
                }
                if let Some(logo) = logo_slot {
                    {logo}
                } else {
                    div { class: "flex items-center gap-2",
                        div { class: "flex h-6 w-6 items-center justify-center rounded-sm bg-primary text-primary-foreground font-bold text-xs shadow-2xs", "E" }
                        span { class: "font-semibold text-xs tracking-tight text-foreground", "EnterpriseUI" }
                    }
                }
            }

            // Right Actions
            div { class: "flex items-center gap-2",
                if let Some(handler) = on_search {
                    if !hide_search {
                        button {
                            r#type: "button",
                            class: "flex items-center gap-2 h-7 rounded-md border border-input bg-muted/30 px-2 text-2xs text-muted-foreground hover:bg-muted hover:text-foreground cursor-pointer",
                            onclick: move |_| handler.call(()),
                            svg {
                                class: "w-3.5 h-3.5 shrink-0",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                circle { cx: "11", cy: "11", r: "7" }
                                path { d: "m20 20-3.5-3.5" }
                            }
                            span { "{search_label}" }
                            kbd { class: "rounded border border-border bg-background px-1 text-[10px]", "{search_shortcut}" }
                        }
                    }
                }

                if let Some(actions) = actions_slot {
                    {actions}
                }

                if !hide_language {
                    LanguageSwitcher { label: language_label.clone(), show_flag: false }
                }

                // Theme Mode Switcher
                if !hide_theme {
                    button {
                        r#type: "button",
                        class: "h-7 w-7 rounded flex items-center justify-center text-muted-foreground hover:bg-muted hover:text-foreground cursor-pointer",
                        onclick: move |_| {
                            let next = if is_dark { ThemeMode::Light } else { ThemeMode::Dark };
                            settings.write().theme = next;
                        },
                        title: "{theme_label}",
                        aria_label: "{theme_label}",
                        if is_dark {
                            svg {
                                class: "w-4 h-4",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                circle { cx: "12", cy: "12", r: "4" }
                                path { d: "M12 2v2M12 20v2M2 12h2M20 12h2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M19.1 4.9l-1.4 1.4M6.3 17.7l-1.4 1.4" }
                            }
                        } else {
                            svg {
                                class: "w-4 h-4",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                path { d: "M21 12.8A8.5 8.5 0 1 1 11.2 3a6.5 6.5 0 0 0 9.8 9.8Z" }
                            }
                        }
                    }
                }

                if let Some(user_menu) = user_menu_slot {
                    {user_menu}
                }
            }
        }
    }
}
