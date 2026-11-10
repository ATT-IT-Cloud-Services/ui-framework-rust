use dioxus::prelude::*;
use crate::ui::badge::{Badge, BadgeVariant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlarmSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlarmItem {
    pub id: String,
    pub source: String,
    pub message: String,
    pub severity: AlarmSeverity,
    pub timestamp: String,
    pub acknowledged: bool,
}

#[component]
pub fn AlarmList(
    alarms: Vec<AlarmItem>,
    #[props(default = "")] class: &'static str,
    onacknowledge: Option<EventHandler<String>>,
) -> Element {
    let ack_handler = onacknowledge;

    rsx! {
        div { class: "space-y-1.5 {class}",
            for alarm in alarms {
                {
                    let id_clone = alarm.id.clone();
                    let ack = ack_handler;
                    let (badge_variant, sev_label) = match alarm.severity {
                        AlarmSeverity::Critical => (BadgeVariant::Destructive, "CRITICAL"),
                        AlarmSeverity::High => (BadgeVariant::Destructive, "HIGH"),
                        AlarmSeverity::Medium => (BadgeVariant::Warning, "MEDIUM"),
                        AlarmSeverity::Low => (BadgeVariant::Info, "LOW"),
                    };

                    rsx! {
                        div {
                            key: "{alarm.id}",
                            class: "flex items-center justify-between gap-3 p-3 rounded-lg border border-border bg-card text-xs",
                            div { class: "flex items-center gap-2.5 flex-1 min-w-0",
                                Badge { variant: badge_variant, "{sev_label}" }
                                div { class: "flex flex-col min-w-0",
                                    span { class: "font-semibold text-foreground truncate", "{alarm.source}" }
                                    span { class: "text-2xs text-muted-foreground truncate", "{alarm.message}" }
                                }
                            }
                            div { class: "flex items-center gap-2 shrink-0",
                                span { class: "text-2xs text-muted-foreground hidden sm:inline", "{alarm.timestamp}" }
                                if !alarm.acknowledged {
                                    if let Some(h) = ack {
                                        button {
                                            r#type: "button",
                                            class: "h-6 px-2 rounded border border-border bg-muted/30 hover:bg-muted text-2xs font-semibold cursor-pointer",
                                            onclick: move |_| h.call(id_clone.clone()),
                                            "Ack"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
