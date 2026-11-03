use dioxus::prelude::*;
use crate::settings::provider::use_layout_settings;
use crate::config::types::{ContrastMode, ThemeMode};

#[component]
pub fn ShiftModeToggle(#[props(default = "")] class: &'static str) -> Element {
    let mut settings = use_layout_settings();

    let is_night = settings.read().theme == ThemeMode::Dark && settings.read().contrast == ContrastMode::Normal;
    let is_control_room = settings.read().contrast == ContrastMode::High;
    let is_day = settings.read().theme == ThemeMode::Light;

    rsx! {
        div { class: "inline-flex items-center rounded-lg border border-border bg-muted/40 p-1 text-xs select-none {class}",
            button {
                r#type: "button",
                class: if is_day { "px-2.5 py-1 rounded-md bg-background text-foreground shadow-2xs font-semibold cursor-pointer" } else { "px-2.5 py-1 text-muted-foreground hover:text-foreground cursor-pointer" },
                onclick: move |_| {
                    settings.write().theme = ThemeMode::Light;
                    settings.write().contrast = ContrastMode::Normal;
                },
                "Day"
            }
            button {
                r#type: "button",
                class: if is_night { "px-2.5 py-1 rounded-md bg-background text-foreground shadow-2xs font-semibold cursor-pointer" } else { "px-2.5 py-1 text-muted-foreground hover:text-foreground cursor-pointer" },
                onclick: move |_| {
                    settings.write().theme = ThemeMode::Dark;
                    settings.write().contrast = ContrastMode::Normal;
                },
                "Night"
            }
            button {
                r#type: "button",
                class: if is_control_room { "px-2.5 py-1 rounded-md bg-background text-primary shadow-2xs font-bold cursor-pointer" } else { "px-2.5 py-1 text-muted-foreground hover:text-foreground cursor-pointer" },
                onclick: move |_| {
                    settings.write().theme = ThemeMode::Dark;
                    settings.write().contrast = ContrastMode::High;
                },
                "Control Room"
            }
        }
    }
}
