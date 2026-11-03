use dioxus::prelude::*;
use super::types::NavigationItem;
use crate::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn SidebarRail(
    items: Vec<NavigationItem>,
    #[props(default = String::new())] active_id: String,
    #[props(default = "")] class: &'static str,
    header_slot: Option<Element>,
    footer_slot: Option<Element>,
) -> Element {
    let mut docked_id = use_signal(|| None::<String>);
    let mut open_popover_id = use_signal(|| None::<String>);

    let docked_item = items.iter().find(|i| Some(i.id.clone()) == docked_id()).cloned();

    rsx! {
        div { class: "flex h-full",
            // 64px Primary Icon Rail
            aside {
                class: "flex h-full w-16 flex-col items-center border-e border-border bg-sidebar text-sidebar-foreground py-3 select-none shrink-0 {class}",

                if let Some(header) = header_slot {
                    div { class: "mb-3", {header} }
                }

                div {
                    class: "flex-1 w-full flex flex-col items-center gap-1.5 overflow-y-auto px-1.5",
                    for item in items.iter() {
                        {
                            let has_children = !item.children.is_empty();
                            let is_active = active_id == item.id;
                            let is_docked = Some(item.id.clone()) == docked_id();
                            let item_id = item.id.clone();
                            let label_first = item.label.chars().next().unwrap_or('●').to_string();

                            rsx! {
                                div {
                                    key: "{item.id}",
                                    class: "relative",
                                    button {
                                        r#type: "button",
                                        class: if is_docked {
                                            "relative flex h-10 w-10 items-center justify-center rounded-md text-xs font-bold bg-primary text-primary-foreground shadow-xs cursor-pointer"
                                        } else if is_active {
                                            "relative flex h-10 w-10 items-center justify-center rounded-md text-xs font-semibold bg-primary/10 text-primary cursor-pointer"
                                        } else {
                                            "relative flex h-10 w-10 items-center justify-center rounded-md text-xs text-muted-foreground hover:bg-muted/70 hover:text-foreground cursor-pointer"
                                        },
                                        onclick: move |_| {
                                            if has_children {
                                                if open_popover_id() == Some(item_id.clone()) {
                                                    open_popover_id.set(None);
                                                } else {
                                                    open_popover_id.set(Some(item_id.clone()));
                                                }
                                            }
                                        },
                                        title: "{item.label}",
                                        "{label_first}"
                                    }

                                    // Flyout Popover
                                    if has_children && open_popover_id() == Some(item.id.clone()) {
                                        div {
                                            class: "absolute start-full top-0 ms-2 z-[600] w-60 p-2 rounded-lg border border-border bg-popover text-popover-foreground shadow-xl animate-in fade-in-50",
                                            div {
                                                class: "flex items-center justify-between pb-1.5 mb-1 border-b border-border text-xs font-semibold",
                                                span { class: "truncate", "{item.label}" }
                                                button {
                                                    r#type: "button",
                                                    class: "text-[10px] px-1.5 py-0.5 rounded border border-border bg-muted/40 hover:bg-muted text-muted-foreground hover:text-foreground cursor-pointer",
                                                    onclick: {
                                                        let id_for_dock = item.id.clone();
                                                        move |_| {
                                                            docked_id.set(Some(id_for_dock.clone()));
                                                            open_popover_id.set(None);
                                                        }
                                                    },
                                                    "Dauerhaft"
                                                }
                                            }
                                            ul {
                                                class: "space-y-0.5",
                                                for child in item.children.iter() {
                                                    li {
                                                        key: "{child.id}",
                                                        div {
                                                            class: "flex items-center justify-between px-2 py-1 text-xs rounded hover:bg-muted/50 text-foreground cursor-pointer",
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

                if let Some(footer) = footer_slot {
                    div { class: "mt-auto pt-2", {footer} }
                }
            }

            // Permanently Docked Submenu Panel (Pin / Ausklappen)
            if let Some(docked) = docked_item {
                aside {
                    class: "flex h-full w-56 flex-col border-e border-border bg-sidebar/95 backdrop-blur-md p-3 select-none text-xs shrink-0 animate-in slide-in-from-left-2 duration-150",
                    div {
                        class: "flex items-center justify-between pb-2 mb-2 border-b border-border/60",
                        span { class: "font-semibold text-foreground truncate", "{docked.label}" }
                        button {
                            r#type: "button",
                            class: "h-6 w-6 rounded flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer",
                            onclick: move |_| docked_id.set(None),
                            "✕"
                        }
                    }
                    div {
                        class: "flex-1 overflow-y-auto space-y-1",
                        for child in docked.children {
                            div {
                                key: "{child.id}",
                                class: "flex items-center justify-between px-2.5 py-1.5 text-xs rounded hover:bg-muted/60 text-foreground cursor-pointer",
                                span { "{child.label}" }
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
