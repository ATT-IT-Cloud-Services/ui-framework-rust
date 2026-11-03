use dioxus::prelude::*;
use super::types::NavigationItem;
use crate::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn SidebarDual(
    items: Vec<NavigationItem>,
    #[props(default = String::new())] active_id: String,
    #[props(default = "")] class: &'static str,
    header_slot: Option<Element>,
    footer_slot: Option<Element>,
) -> Element {
    let mut selected_module_id = use_signal(|| {
        items.first().map(|i| i.id.clone()).unwrap_or_default()
    });

    let active_module = items.iter().find(|i| i.id == selected_module_id()).or_else(|| items.first()).cloned();
    let sub_items = active_module.as_ref().map(|m| m.children.clone()).unwrap_or_default();

    rsx! {
        aside { class: "flex h-full border-e border-border select-none shrink-0 {class}",
            // 56px Module Rail
            div { class: "flex h-full w-14 flex-col items-center border-e border-border/60 bg-muted/30 py-3",
                if let Some(h) = header_slot {
                    div { class: "mb-3", {h} }
                }

                div { class: "flex-1 w-full flex flex-col items-center gap-1.5 overflow-y-auto px-1",
                    for item in items.iter() {
                        {
                            let is_selected = selected_module_id() == item.id;
                            let id_clone = item.id.clone();
                            let first_char = item.label.chars().next().unwrap_or('●').to_string();

                            rsx! {
                                button {
                                    key: "{item.id}",
                                    r#type: "button",
                                    class: if is_selected {
                                        "h-10 w-10 flex items-center justify-center rounded-lg bg-primary text-primary-foreground font-bold text-xs shadow-xs cursor-pointer"
                                    } else {
                                        "h-10 w-10 flex items-center justify-center rounded-lg hover:bg-muted text-muted-foreground hover:text-foreground text-xs font-semibold cursor-pointer"
                                    },
                                    onclick: move |_| selected_module_id.set(id_clone.clone()),
                                    title: "{item.label}",
                                    "{first_char}"
                                }
                            }
                        }
                    }
                }

                if let Some(f) = footer_slot {
                    div { class: "mt-auto", {f} }
                }
            }

            // 200px Sub-menu Panel
            div { class: "flex h-full w-52 flex-col bg-sidebar text-sidebar-foreground py-3 px-2",
                if let Some(module) = &active_module {
                    div { class: "px-2 pb-2 mb-2 border-b border-border/50",
                        h4 { class: "text-xs font-semibold text-foreground truncate", "{module.label}" }
                        if let Some(tag) = &module.tag {
                            Badge { variant: BadgeVariant::Outline, "{tag}" }
                        }
                    }
                }

                div { class: "flex-1 overflow-y-auto space-y-0.5",
                    for child in sub_items {
                        {
                            let child_href = child.href.clone().unwrap_or_else(|| "#".to_string());
                            rsx! {
                                a {
                                    key: "{child.id}",
                                    href: "{child_href}",
                                    class: "block",
                                    div {
                                        class: "flex items-center justify-between rounded-md px-2.5 py-1.5 text-xs text-muted-foreground hover:bg-muted/50 hover:text-foreground cursor-pointer transition-colors",
                                        span { class: "truncate", "{child.label}" }
                                        if let Some(b) = child.badge {
                                            Badge { variant: BadgeVariant::Secondary, "{b}" }
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
