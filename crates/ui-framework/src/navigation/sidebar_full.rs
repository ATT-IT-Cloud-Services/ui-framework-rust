use dioxus::prelude::*;
use super::types::NavigationItem;
use crate::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn SidebarFull(
    items: Vec<NavigationItem>,
    #[props(default = String::new())] active_id: String,
    #[props(default = "")] class: &'static str,
    header_slot: Option<Element>,
    footer_slot: Option<Element>,
) -> Element {
    let mut expanded_ids = use_signal(|| {
        // Auto-expand all sections that have children by default
        vec![
            "dispatch".to_string(),
            "telemetry".to_string(),
            "reports".to_string(),
        ]
    });

    // Group items by section
    let mut sections: Vec<(String, Vec<NavigationItem>)> = Vec::new();
    for item in items {
        let sec = item.section.clone().unwrap_or_else(|| "General".to_string());
        if let Some((_, list)) = sections.iter_mut().find(|(s, _)| s == &sec) {
            list.push(item);
        } else {
            sections.push((sec, vec![item]));
        }
    }

    rsx! {
        aside {
            class: "flex h-full w-64 flex-col border-e border-border bg-sidebar text-sidebar-foreground transition-all duration-200 select-none {class}",

            if let Some(header) = header_slot {
                div { class: "p-3 border-b border-border/60", {header} }
            }

            div {
                class: "flex-1 overflow-y-auto px-2 py-3 space-y-4",
                for (section_name, section_items) in sections {
                    div {
                        key: "{section_name}",
                        class: "space-y-1",
                        span { class: "px-2 text-2xs font-semibold uppercase tracking-wider text-muted-foreground/80 block", "{section_name}" }
                        ul {
                            class: "space-y-0.5",
                            for item in section_items {
                                {
                                    let has_children = !item.children.is_empty();
                                    let is_active = active_id == item.id;
                                    let is_expanded = expanded_ids().contains(&item.id);
                                    let item_id = item.id.clone();
                                    let href_val = item.href.clone().unwrap_or_else(|| "#".to_string());

                                    rsx! {
                                        li {
                                            key: "{item.id}",
                                            if !has_children {
                                                a {
                                                    href: "{href_val}",
                                                    class: "block",
                                                    div {
                                                        class: if is_active {
                                                            "group relative flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-xs font-semibold bg-primary/10 text-primary cursor-pointer transition-colors"
                                                        } else {
                                                            "group relative flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-xs font-medium text-muted-foreground hover:bg-muted/50 hover:text-foreground cursor-pointer transition-colors"
                                                        },
                                                        span { class: "truncate flex-1 text-start", "{item.label}" }
                                                        if let Some(tag) = &item.tag {
                                                            Badge { variant: BadgeVariant::Outline, "{tag}" }
                                                        }
                                                        if let Some(badge) = &item.badge {
                                                            Badge { variant: BadgeVariant::Secondary, "{badge}" }
                                                        }
                                                    }
                                                }
                                            } else {
                                                div {
                                                    class: if is_active {
                                                        "group relative flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-xs font-semibold bg-primary/10 text-primary cursor-pointer transition-colors"
                                                    } else {
                                                        "group relative flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-xs font-medium text-muted-foreground hover:bg-muted/50 hover:text-foreground cursor-pointer transition-colors"
                                                    },
                                                    onclick: move |_| {
                                                        let mut ids = expanded_ids();
                                                        if ids.contains(&item_id) {
                                                            ids.retain(|id| id != &item_id);
                                                        } else {
                                                            ids.push(item_id.clone());
                                                        }
                                                        expanded_ids.set(ids);
                                                    },
                                                    span { class: "truncate flex-1 text-start", "{item.label}" }
                                                    if let Some(tag) = &item.tag {
                                                        Badge { variant: BadgeVariant::Outline, "{tag}" }
                                                    }
                                                    if let Some(badge) = &item.badge {
                                                        Badge { variant: BadgeVariant::Secondary, "{badge}" }
                                                    }
                                                    span {
                                                        class: if is_expanded { "text-2xs text-muted-foreground rotate-90 transition-transform cursor-pointer" } else { "text-2xs text-muted-foreground transition-transform cursor-pointer" },
                                                        "▶"
                                                    }
                                                }

                                                // Submenu children links
                                                if is_expanded {
                                                    ul {
                                                        class: "mt-0.5 space-y-0.5 border-s border-border/40 ms-4 ps-1 animate-in fade-in-50 duration-150",
                                                        for child in item.children {
                                                            {
                                                                let is_child_active = active_id == child.id;
                                                                let child_href = child.href.clone().unwrap_or_else(|| "#".to_string());
                                                                rsx! {
                                                                    li {
                                                                        key: "{child.id}",
                                                                        a {
                                                                            href: "{child_href}",
                                                                            class: "block",
                                                                            div {
                                                                                class: if is_child_active {
                                                                                    "flex items-center justify-between rounded px-2.5 py-1.5 text-2xs font-semibold bg-primary/10 text-primary cursor-pointer hover:bg-primary/15 transition-colors"
                                                                                } else {
                                                                                    "flex items-center justify-between rounded px-2.5 py-1.5 text-2xs text-muted-foreground hover:bg-muted/40 hover:text-foreground cursor-pointer transition-colors"
                                                                                },
                                                                                span { class: "truncate", "{child.label}" }
                                                                                if let Some(b) = &child.badge {
                                                                                    span { class: "text-[10px] text-muted-foreground font-mono", "{b}" }
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
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(footer) = footer_slot {
                div { class: "p-3 border-t border-border/60 bg-muted/20", {footer} }
            }
        }
    }
}
