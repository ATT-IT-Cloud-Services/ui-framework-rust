use dioxus::prelude::*;
use crate::settings::provider::use_layout_settings;
use crate::settings::types::ToastPosition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToastItem {
    pub id: u64,
    pub message: String,
    pub description: Option<String>,
    pub toast_type: ToastType,
}

#[derive(Clone, Copy)]
pub struct ToastContext {
    pub items: Signal<Vec<ToastItem>>,
}

pub fn use_toast() -> impl Fn(&str, ToastType, Option<&str>) {
    let ctx = use_context::<ToastContext>();
    move |message: &str, toast_type: ToastType, description: Option<&str>| {
        let mut list = ctx.items;
        let id = (list().len() as u64) + 1;
        list.write().push(ToastItem {
            id,
            message: message.to_string(),
            description: description.map(|d| d.to_string()),
            toast_type,
        });
    }
}

#[component]
pub fn OverlayProvider(children: Element) -> Element {
    let items = use_signal(Vec::<ToastItem>::new);
    use_context_provider(|| ToastContext { items });
    let settings = use_layout_settings();

    let pos = settings.read().overlays.toast_position;
    let pos_class = match pos {
        ToastPosition::TopStart => "top-4 start-4 items-start",
        ToastPosition::TopCenter => "top-4 start-1/2 -translate-x-1/2 items-center",
        ToastPosition::TopEnd => "top-4 end-4 items-end",
        ToastPosition::BottomStart => "bottom-4 start-4 items-start",
        ToastPosition::BottomCenter => "bottom-4 start-1/2 -translate-x-1/2 items-center",
        ToastPosition::BottomEnd => "bottom-4 end-4 items-end",
    };

    rsx! {
        {children}

        // Floating Toast Stack
        div {
            class: "fixed z-[700] flex flex-col gap-2 pointer-events-none p-2 {pos_class}",
            for item in items() {
                {
                    let (border, icon) = match item.toast_type {
                        ToastType::Success => ("border-emerald-500/40 bg-popover text-foreground", "✓"),
                        ToastType::Error => ("border-rose-500/40 bg-popover text-foreground", "✕"),
                        ToastType::Warning => ("border-amber-500/40 bg-popover text-foreground", "⚠"),
                        ToastType::Info => ("border-sky-500/40 bg-popover text-foreground", "ℹ"),
                    };

                    rsx! {
                        div {
                            key: "{item.id}",
                            class: "pointer-events-auto rounded-xl border p-3.5 shadow-xl text-xs w-80 space-y-1 animate-in fade-in-50 slide-in-from-bottom-2 {border}",
                            div { class: "flex items-center gap-2 font-semibold",
                                span { "{icon}" }
                                span { "{item.message}" }
                            }
                            if let Some(desc) = item.description {
                                p { class: "text-2xs text-muted-foreground", "{desc}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
