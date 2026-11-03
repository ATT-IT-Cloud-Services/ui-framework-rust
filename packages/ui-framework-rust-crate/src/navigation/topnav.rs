use dioxus::prelude::*;
use super::types::NavigationItem;
use crate::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn Topnav(
    items: Vec<NavigationItem>,
    #[props(default = String::new())] active_id: String,
    #[props(default = "")] class: &'static str,
) -> Element {
    let mut open_dropdown_id = use_signal(|| None::<String>);

    rsx! {
        nav { class: "flex items-center gap-1 border-b border-border bg-sidebar px-4 h-10 select-none overflow-x-auto text-xs {class}",
            for item in items.iter() {
                {
                    let has_children = !item.children.is_empty();
                    let is_active = active_id == item.id;
                    let item_id = item.id.clone();

                    let item_href = item.href.clone().unwrap_or_else(|| "#".to_string());
                    rsx! {
                        div {
                            key: "{item.id}",
                            class: "relative",
                            if !has_children {
                                a {
                                    href: "{item_href}",
                                    class: "block",
                                    div {
                                        class: if is_active {
                                            "inline-flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-semibold bg-primary/10 text-primary cursor-pointer transition-colors"
                                        } else {
                                            "inline-flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium text-muted-foreground hover:bg-muted/70 hover:text-foreground cursor-pointer transition-colors"
                                        },
                                        span { "{item.label}" }
                                        if let Some(badge) = &item.badge {
                                            Badge { variant: BadgeVariant::Secondary, "{badge}" }
                                        }
                                    }
                                }
                            } else {
                                button {
                                    r#type: "button",
                                    class: if is_active {
                                        "inline-flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-semibold bg-primary/10 text-primary cursor-pointer transition-colors"
                                    } else {
                                        "inline-flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs font-medium text-muted-foreground hover:bg-muted/70 hover:text-foreground cursor-pointer transition-colors"
                                    },
                                    onclick: move |_| {
                                        if open_dropdown_id() == Some(item_id.clone()) {
                                            open_dropdown_id.set(None);
                                        } else {
                                            open_dropdown_id.set(Some(item_id.clone()));
                                        }
                                    },
                                    span { "{item.label}" }
                                    if let Some(badge) = &item.badge {
                                        Badge { variant: BadgeVariant::Secondary, "{badge}" }
                                    }
                                    span { class: "text-[10px] text-muted-foreground ms-0.5", "▼" }
                                }

                                if open_dropdown_id() == Some(item.id.clone()) {
                                    div {
                                        class: "absolute start-0 top-full mt-1 z-[600] w-52 rounded-md border border-border bg-popover p-1 shadow-lg animate-in fade-in-50",
                                        for child in item.children.iter() {
                                            {
                                                let child_href = child.href.clone().unwrap_or_else(|| "#".to_string());
                                                rsx! {
                                                    a {
                                                        key: "{child.id}",
                                                        href: "{child_href}",
                                                        class: "block",
                                                        div {
                                                            class: "flex items-center justify-between px-2.5 py-1.5 text-xs rounded hover:bg-muted text-foreground cursor-pointer",
                                                            span { "{child.label}" }
                                                            if let Some(b) = &child.badge {
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
                }
            }
        }
    }
}
