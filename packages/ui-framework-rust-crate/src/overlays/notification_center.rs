use dioxus::prelude::*;
use crate::ui::badge::{Badge, BadgeVariant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationEntry {
    pub id: String,
    pub title: String,
    pub message: String,
    pub timestamp: String,
    pub read: bool,
}

#[component]
pub fn NotificationCenter(
    #[props(default = Vec::new())] notifications: Vec<NotificationEntry>,
    #[props(default = "")] class: &'static str,
) -> Element {
    let mut items = use_signal(|| {
        if notifications.is_empty() {
            vec![
                NotificationEntry {
                    id: "n-1".to_string(),
                    title: "Anomaly in Turbine 4".to_string(),
                    message: "Bearing vibration reached warning threshold (4.8 mm/s).".to_string(),
                    timestamp: "10m ago".to_string(),
                    read: false,
                },
                NotificationEntry {
                    id: "n-2".to_string(),
                    title: "Shift report generated".to_string(),
                    message: "Telemetry logs successfully archived.".to_string(),
                    timestamp: "1h ago".to_string(),
                    read: false,
                },
            ]
        } else {
            notifications
        }
    });

    let mut open = use_signal(|| false);

    let unread_count = items().iter().filter(|n| !n.read).count();

    rsx! {
        div { class: "relative inline-block {class}",
            button {
                r#type: "button",
                class: "relative flex h-8 w-8 items-center justify-center rounded-md text-muted-foreground hover:bg-muted hover:text-foreground cursor-pointer text-sm",
                onclick: move |_| open.set(!open()),
                "🔔"
                if unread_count > 0 {
                    span { class: "absolute top-1.5 end-1.5 flex h-2 w-2 rounded-full bg-primary ring-2 ring-background animate-pulse" }
                }
            }

            if open() {
                div {
                    class: "absolute end-0 mt-2 w-80 p-3 rounded-xl border border-border bg-popover text-popover-foreground shadow-2xl z-[600] animate-in fade-in-50 text-xs select-none",
                    div { class: "flex items-center justify-between pb-2 mb-2 border-b border-border",
                        div { class: "flex items-center gap-1.5",
                            span { class: "font-semibold text-foreground", "Notifications" }
                            if unread_count > 0 {
                                Badge { variant: BadgeVariant::Default, "{unread_count} new" }
                            }
                        }
                        if unread_count > 0 {
                            button {
                                r#type: "button",
                                class: "text-2xs text-muted-foreground hover:text-foreground cursor-pointer",
                                onclick: move |_| {
                                    items.write().iter_mut().for_each(|n| n.read = true);
                                },
                                "Mark all read"
                            }
                        }
                    }

                    div { class: "space-y-1.5 max-h-80 overflow-y-auto",
                        for item in items() {
                            div {
                                key: "{item.id}",
                                class: if item.read {
                                    "p-2.5 rounded-lg border border-transparent hover:bg-muted/40 text-muted-foreground cursor-pointer"
                                } else {
                                    "p-2.5 rounded-lg border border-primary/20 bg-primary/5 text-foreground font-medium cursor-pointer"
                                },
                                onclick: {
                                    let id = item.id.clone();
                                    move |_| {
                                        if let Some(entry) = items.write().iter_mut().find(|n| n.id == id) {
                                            entry.read = true;
                                        }
                                    }
                                },
                                div { class: "flex justify-between items-center gap-1",
                                    span { class: "font-semibold text-foreground truncate", "{item.title}" }
                                    span { class: "text-[10px] text-muted-foreground", "{item.timestamp}" }
                                }
                                p { class: "text-2xs opacity-90 line-clamp-2 mt-0.5", "{item.message}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
