use dioxus::prelude::*;
use crate::ui::button::{Button, ButtonVariant};
use crate::i18n::provider::use_t;

#[component]
pub fn NotFound(onback: Option<EventHandler<()>>) -> Element {
    let t = use_t();
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-[400px] text-center p-8 space-y-4",
            div { class: "flex h-14 w-14 items-center justify-center rounded-full bg-muted text-muted-foreground text-2xl font-bold", "❓" }
            div { class: "space-y-1 max-w-sm",
                h2 { class: "text-base font-semibold tracking-tight text-foreground", "{t(\"errors.notFoundTitle\")}" }
                p { class: "text-xs text-muted-foreground", "{t(\"errors.notFoundDesc\")}" }
            }
            if let Some(h) = onback {
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| h.call(()),
                    "{t(\"common.back\")}"
                }
            }
        }
    }
}

#[component]
pub fn Forbidden(onback: Option<EventHandler<()>>) -> Element {
    let t = use_t();
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-[400px] text-center p-8 space-y-4",
            div { class: "flex h-14 w-14 items-center justify-center rounded-full bg-rose-50 text-rose-600 text-2xl font-bold", "⛔" }
            div { class: "space-y-1 max-w-sm",
                h2 { class: "text-base font-semibold tracking-tight text-foreground", "{t(\"errors.forbiddenTitle\")}" }
                p { class: "text-xs text-muted-foreground", "{t(\"errors.forbiddenDesc\")}" }
            }
            if let Some(h) = onback {
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| h.call(()),
                    "{t(\"common.back\")}"
                }
            }
        }
    }
}

#[component]
pub fn ServerError(onretry: Option<EventHandler<()>>) -> Element {
    let t = use_t();
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-[400px] text-center p-8 space-y-4",
            div { class: "flex h-14 w-14 items-center justify-center rounded-full bg-destructive/10 text-destructive text-2xl font-bold", "💥" }
            div { class: "space-y-1 max-w-sm",
                h2 { class: "text-base font-semibold tracking-tight text-foreground", "{t(\"errors.serverErrorTitle\")}" }
                p { class: "text-xs text-muted-foreground", "{t(\"errors.serverErrorDesc\")}" }
            }
            if let Some(h) = onretry {
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| h.call(()),
                    "{t(\"errors.retry\")}"
                }
            }
        }
    }
}

#[component]
pub fn Maintenance() -> Element {
    let t = use_t();
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-[400px] text-center p-8 space-y-4",
            div { class: "flex h-14 w-14 items-center justify-center rounded-full bg-amber-50 text-amber-600 text-2xl font-bold", "🔧" }
            div { class: "space-y-1 max-w-sm",
                h2 { class: "text-base font-semibold tracking-tight text-foreground", "{t(\"errors.maintenanceTitle\")}" }
                p { class: "text-xs text-muted-foreground", "{t(\"errors.maintenanceDesc\")}" }
            }
        }
    }
}
