use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityEntry {
    pub id: String,
    pub actor: String,
    pub action: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachmentEntry {
    pub id: String,
    pub name: String,
    pub size: String,
}

#[component]
pub fn InfoPanel(
    title: String,
    #[props(default = None)] subtitle: Option<String>,
    #[props(default = Vec::new())] key_values: Vec<KeyValuePair>,
    #[props(default = Vec::new())] activities: Vec<ActivityEntry>,
    #[props(default = Vec::new())] attachments: Vec<AttachmentEntry>,
    #[props(default = "")] class: &'static str,
    onclose: Option<EventHandler<()>>,
    footer_actions: Option<Element>,
    children: Option<Element>,
) -> Element {
    rsx! {
        aside {
            class: "flex flex-col h-full space-y-4 select-none text-xs {class}",

            // Header
            div { class: "flex items-center justify-between pb-3 border-b border-border",
                div { class: "space-y-0.5",
                    h3 { class: "font-semibold text-foreground text-sm", "{title}" }
                    if let Some(sub) = subtitle {
                        p { class: "text-2xs text-muted-foreground", "{sub}" }
                    }
                }

                if let Some(h) = onclose {
                    button {
                        r#type: "button",
                        class: "h-7 w-7 rounded-md flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer text-xs",
                        onclick: move |_| h.call(()),
                        "✕"
                    }
                }
            }

            // Body
            div { class: "flex-1 overflow-y-auto space-y-5 pe-1",
                // Key-Values
                if !key_values.is_empty() {
                    div { class: "space-y-2",
                        h4 { class: "text-2xs font-semibold uppercase tracking-wider text-muted-foreground", "Properties" }
                        dl { class: "grid grid-cols-1 gap-2 rounded-md border border-border/60 bg-muted/20 p-3",
                            for kv in key_values {
                                div { key: "{kv.key}", class: "flex items-center justify-between",
                                    dt { class: "text-muted-foreground text-2xs", "{kv.key}" }
                                    dd { class: "font-medium text-foreground text-end", "{kv.value}" }
                                }
                            }
                        }
                    }
                }

                // Activity timeline
                if !activities.is_empty() {
                    div { class: "space-y-2",
                        h4 { class: "text-2xs font-semibold uppercase tracking-wider text-muted-foreground", "Activity" }
                        ul { class: "space-y-2",
                            for act in activities {
                                li { key: "{act.id}", class: "text-2xs border-s-2 border-border ps-2.5 py-0.5 space-y-0.5",
                                    p { class: "text-foreground",
                                        span { class: "font-semibold", "{act.actor}" }
                                        " "
                                        "{act.action}"
                                    }
                                    span { class: "text-muted-foreground text-[10px] block", "{act.timestamp}" }
                                }
                            }
                        }
                    }
                }

                // Attachments
                if !attachments.is_empty() {
                    div { class: "space-y-2",
                        h4 { class: "text-2xs font-semibold uppercase tracking-wider text-muted-foreground", "Attachments ({attachments.len()})" }
                        ul { class: "space-y-1.5",
                            for att in attachments {
                                li { key: "{att.id}", class: "flex items-center justify-between rounded-md border border-border/60 p-2 hover:bg-muted/40 transition-colors",
                                    span { class: "font-medium text-foreground truncate max-w-[150px]", "{att.name}" }
                                    span { class: "text-2xs text-muted-foreground", "{att.size}" }
                                }
                            }
                        }
                    }
                }

                if let Some(c) = children {
                    {c}
                }
            }

            if let Some(act) = footer_actions {
                div { class: "pt-3 border-t border-border flex items-center justify-end gap-2", {act} }
            }
        }
    }
}
