use dioxus::prelude::*;
use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn ErrorState(
    #[props(default = "Failed to load data".to_string())] title: String,
    #[props(default = "An unexpected error occurred while communicating with the telemetry stream.".to_string())] message: String,
    #[props(default = "")] class: &'static str,
    onretry: Option<EventHandler<()>>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center p-8 text-center rounded-xl border border-destructive/30 bg-destructive/5 space-y-3 {class}",
            div { class: "flex h-12 w-12 items-center justify-center rounded-full bg-destructive/10 text-destructive text-xl font-bold", "⚠" }

            div { class: "space-y-1 max-w-sm",
                h3 { class: "text-sm font-semibold tracking-tight text-foreground", "{title}" }
                p { class: "text-xs text-muted-foreground leading-relaxed", "{message}" }
            }

            if let Some(h) = onretry {
                div { class: "pt-2",
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| h.call(()),
                        "Try again"
                    }
                }
            }
        }
    }
}
