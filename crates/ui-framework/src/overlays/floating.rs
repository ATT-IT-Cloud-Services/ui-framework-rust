use dioxus::prelude::*;
use crate::ui::button::Button;

#[component]
pub fn FilterPopover(
    title: Option<String>,
    #[props(default = 0)] active_count: usize,
    #[props(default = "")] class: &'static str,
    onapply: Option<EventHandler<()>>,
    onreset: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div { class: "relative inline-block {class}",
            button {
                r#type: "button",
                class: "inline-flex items-center gap-1.5 h-8 px-2.5 rounded-md border border-input bg-background hover:bg-muted text-xs font-medium cursor-pointer",
                onclick: move |_| open.set(!open()),
                span { "⚡ Filter" }
                if active_count > 0 {
                    span { class: "flex h-4 w-4 items-center justify-center rounded-full bg-primary text-primary-foreground text-[10px] font-bold", "{active_count}" }
                }
            }

            if open() {
                div { class: "absolute start-0 top-full mt-1.5 z-[600] w-72 rounded-xl border border-border bg-popover p-3 shadow-xl space-y-3 animate-in fade-in-50 text-xs",
                    div { class: "flex items-center justify-between pb-2 border-b border-border",
                        span { class: "font-semibold text-foreground", "{title.clone().unwrap_or_else(|| \"Filters\".to_string())}" }
                        if let Some(r) = onreset {
                            button {
                                r#type: "button",
                                class: "text-2xs text-muted-foreground hover:text-foreground cursor-pointer",
                                onclick: move |_| r.call(()),
                                "Reset"
                            }
                        }
                    }

                    div { class: "space-y-2 py-1",
                        {children}
                    }

                    if let Some(a) = onapply {
                        div { class: "pt-2 border-t border-border flex justify-end",
                            Button {
                                onclick: move |_| {
                                    open.set(false);
                                    a.call(());
                                },
                                "Apply"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuickActionItem {
    pub id: String,
    pub label: String,
}

#[component]
pub fn QuickActionsMenu(
    actions: Vec<QuickActionItem>,
    onaction: EventHandler<String>,
    #[props(default = "")] class: &'static str,
) -> Element {
    let mut open = use_signal(|| false);

    rsx! {
        div { class: "fixed bottom-6 end-6 z-[250] flex flex-col items-end gap-2 {class}",
            if open() {
                div { class: "flex flex-col items-end gap-2 mb-2 animate-in slide-in-from-bottom-2",
                    for act in actions {
                        {
                            let id_clone = act.id.clone();
                            rsx! {
                                button {
                                    key: "{act.id}",
                                    r#type: "button",
                                    class: "flex items-center gap-2 rounded-full bg-popover border border-border px-3.5 py-1.5 text-xs font-semibold shadow-md hover:bg-muted text-foreground cursor-pointer transition-transform hover:scale-105 active:scale-95",
                                    onclick: move |_| {
                                        open.set(false);
                                        onaction.call(id_clone.clone());
                                    },
                                    "{act.label}"
                                }
                            }
                        }
                    }
                }
            }

            button {
                r#type: "button",
                class: "flex h-11 w-11 items-center justify-center rounded-full bg-primary text-primary-foreground shadow-xl hover:bg-primary/90 cursor-pointer font-bold text-lg active:scale-95 transition-transform",
                onclick: move |_| open.set(!open()),
                if open() { "✕" } else { "＋" }
            }
        }
    }
}
