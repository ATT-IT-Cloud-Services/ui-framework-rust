use dioxus::prelude::*;
use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn StickyActionBar(
    #[props(default = false)] has_unsaved_changes: bool,
    #[props(default = false)] is_saving: bool,
    #[props(default = "Save")] save_label: &'static str,
    #[props(default = "Cancel")] cancel_label: &'static str,
    #[props(default = "")] class: &'static str,
    onsave: Option<EventHandler<()>>,
    onsaveandclose: Option<EventHandler<()>>,
    oncancel: Option<EventHandler<()>>,
) -> Element {
    rsx! {
        div {
            class: "fixed inset-x-0 bottom-0 z-[250] flex items-center justify-between border-t border-border bg-background/95 backdrop-blur-md px-4 sm:px-6 py-2.5 shadow-md {class}",

            div { class: "flex items-center gap-2",
                if has_unsaved_changes {
                    div { class: "flex items-center gap-1.5 text-xs text-amber-600 dark:text-amber-400 font-medium",
                        span { "⚠" }
                        span { "Unsaved changes" }
                    }
                }
            }

            div { class: "flex items-center gap-2",
                if let Some(h) = oncancel {
                    Button {
                        variant: ButtonVariant::Outline,
                        disabled: is_saving,
                        onclick: move |_| h.call(()),
                        "{cancel_label}"
                    }
                }
                if let Some(h) = onsaveandclose {
                    Button {
                        variant: ButtonVariant::Secondary,
                        disabled: is_saving,
                        onclick: move |_| h.call(()),
                        "Save & Close"
                    }
                }
                if let Some(h) = onsave {
                    Button {
                        variant: ButtonVariant::Default,
                        disabled: is_saving,
                        onclick: move |_| h.call(()),
                        if is_saving { "Saving..." } else { "{save_label}" }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BulkActionItem {
    pub label: String,
    pub is_destructive: bool,
}

#[component]
pub fn BulkActionBar(
    selected_count: usize,
    actions: Vec<BulkActionItem>,
    #[props(default = "")] class: &'static str,
    onclear: EventHandler<()>,
    onaction: EventHandler<String>,
) -> Element {
    if selected_count == 0 {
        return rsx! {};
    }

    rsx! {
        div {
            class: "fixed bottom-6 left-1/2 -translate-x-1/2 z-[250] flex items-center gap-3 rounded-xl border border-border bg-foreground text-background px-4 py-2 shadow-2xl animate-in slide-in-from-bottom-3 select-none text-xs {class}",

            div { class: "flex items-center gap-2 font-medium",
                span { class: "flex h-5 w-5 items-center justify-center rounded-full bg-primary text-primary-foreground text-2xs font-bold",
                    "{selected_count}"
                }
                span { "Selected" }
                button {
                    r#type: "button",
                    class: "text-muted-foreground hover:text-background ms-1 cursor-pointer",
                    onclick: move |_| onclear.call(()),
                    "✕"
                }
            }

            div { class: "h-4 w-px bg-muted-foreground/30" }

            div { class: "flex items-center gap-1.5",
                for act in actions {
                    {
                        let label_clone = act.label.clone();
                        rsx! {
                            button {
                                key: "{act.label}",
                                r#type: "button",
                                class: if act.is_destructive {
                                    "px-2.5 py-1 text-2xs font-medium rounded bg-destructive text-destructive-foreground hover:bg-destructive/90 cursor-pointer"
                                } else {
                                    "px-2.5 py-1 text-2xs font-medium rounded bg-background/15 hover:bg-background/25 text-background cursor-pointer"
                                },
                                onclick: move |_| onaction.call(label_clone.clone()),
                                "{act.label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
