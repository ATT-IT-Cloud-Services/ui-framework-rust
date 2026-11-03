use dioxus::prelude::*;
use crate::i18n::language_switcher::LanguageSwitcher;
use crate::settings::provider::use_layout_settings;
use crate::config::types::ThemeMode;

#[component]
pub fn Header(
    #[props(default = "")] class: &'static str,
    logo_slot: Option<Element>,
    actions_slot: Option<Element>,
    user_menu_slot: Option<Element>,
    on_search: Option<EventHandler<()>>,
) -> Element {
    let mut settings = use_layout_settings();

    let is_sticky = settings.read().header_sticky;
    let is_dark = settings.read().theme == ThemeMode::Dark;

    rsx! {
        header {
            class: if is_sticky {
                "flex h-12 w-full items-center justify-between border-b border-border bg-background/95 px-3 sm:px-4 backdrop-blur-md select-none sticky top-0 z-[300] {class}"
            } else {
                "flex h-12 w-full items-center justify-between border-b border-border bg-background px-3 sm:px-4 select-none {class}"
            },

            // Brand / Logo Slot
            div { class: "flex items-center gap-3",
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
                    button {
                        r#type: "button",
                        class: "flex items-center gap-2 h-7 rounded-md border border-input bg-muted/30 px-2 text-2xs text-muted-foreground hover:bg-muted hover:text-foreground cursor-pointer",
                        onclick: move |_| handler.call(()),
                        span { "🔍 Search... ⌘K" }
                    }
                }

                if let Some(actions) = actions_slot {
                    {actions}
                }

                LanguageSwitcher {}

                // Theme Mode Switcher
                button {
                    r#type: "button",
                    class: "h-7 w-7 rounded flex items-center justify-center text-muted-foreground hover:bg-muted hover:text-foreground cursor-pointer text-xs",
                    onclick: move |_| {
                        let next = if is_dark { ThemeMode::Light } else { ThemeMode::Dark };
                        settings.write().theme = next;
                    },
                    title: "Toggle Theme",
                    if is_dark { "☀" } else { "☾" }
                }

                if let Some(user_menu) = user_menu_slot {
                    {user_menu}
                }
            }
        }
    }
}
