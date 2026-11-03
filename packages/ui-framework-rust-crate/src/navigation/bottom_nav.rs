use dioxus::prelude::*;
use super::types::NavigationItem;
use super::sidebar_full::SidebarFull;

#[component]
pub fn BottomNav(
    items: Vec<NavigationItem>,
    #[props(default = String::new())] active_id: String,
    #[props(default = 4)] max_items: usize,
    #[props(default = "")] class: &'static str,
) -> Element {
    let mut sheet_open = use_signal(|| false);

    let primary_items = items.iter().take(max_items).cloned().collect::<Vec<_>>();

    rsx! {
        nav {
            class: "fixed inset-x-0 bottom-0 z-[100] flex h-14 items-center justify-around border-t border-border bg-sidebar/95 backdrop-blur-md px-2 select-none text-xs {class}",
            for item in primary_items {
                {
                    let is_active = active_id == item.id;
                    let first_char = item.label.chars().next().unwrap_or('●').to_string();

                    rsx! {
                        div {
                            key: "{item.id}",
                            class: if is_active {
                                "flex flex-1 flex-col items-center justify-center h-full min-h-[44px] min-w-[44px] px-1 gap-1 text-primary font-semibold cursor-pointer transition-colors"
                            } else {
                                "flex flex-1 flex-col items-center justify-center h-full min-h-[44px] min-w-[44px] px-1 gap-1 text-muted-foreground hover:text-foreground cursor-pointer transition-colors"
                            },
                            span { class: "text-sm", "{first_char}" }
                            span { class: "text-[11px] leading-tight text-center truncate max-w-full font-medium", "{item.label}" }
                        }
                    }
                }
            }

            button {
                r#type: "button",
                class: "flex flex-1 flex-col items-center justify-center h-full min-h-[44px] min-w-[44px] px-1 gap-1 text-muted-foreground hover:text-foreground cursor-pointer transition-colors",
                onclick: move |_| sheet_open.set(true),
                span { class: "text-sm", "☰" }
                span { class: "text-[11px] leading-tight font-medium", "Menu" }
            }
        }

        // Bottom Sheet Drawer for full navigation tree
        if sheet_open() {
            div {
                class: "fixed inset-0 z-[400] bg-black/40 backdrop-blur-xs",
                onclick: move |_| sheet_open.set(false),
            }
            div {
                class: "fixed inset-x-0 bottom-0 z-[400] h-[80vh] bg-background border-t border-border shadow-2xl flex flex-col p-0 animate-in slide-in-from-bottom duration-200",
                div {
                    class: "flex items-center justify-between p-4 border-b border-border",
                    h3 { class: "text-sm font-semibold", "Navigation Menu" }
                    button {
                        r#type: "button",
                        class: "h-6 w-6 rounded flex items-center justify-center text-muted-foreground hover:text-foreground cursor-pointer",
                        onclick: move |_| sheet_open.set(false),
                        "✕"
                    }
                }
                div { class: "flex-1 overflow-y-auto",
                    SidebarFull {
                        items: items,
                        active_id: active_id,
                        class: "w-full border-none",
                    }
                }
            }
        }
    }
}
