use dioxus::prelude::*;
use super::types::NavigationItem;
use super::sidebar_full::SidebarFull;
use super::sidebar_rail::SidebarRail;

#[component]
pub fn SidebarHybrid(
    items: Vec<NavigationItem>,
    #[props(default = String::new())] active_id: String,
    #[props(default = "")] class: &'static str,
    header_slot: Option<Element>,
    footer_slot: Option<Element>,
) -> Element {
    let mut expanded = use_signal(|| true);

    let toggle_button = rsx! {
        button {
            r#type: "button",
            class: "h-7 px-2 rounded-md border border-border bg-background text-muted-foreground hover:text-foreground text-[10px] font-semibold cursor-pointer",
            onclick: move |_| expanded.set(!expanded()),
            if expanded() { "◀ Einklappen" } else { "▶" }
        }
    };

    rsx! {
        div { class: "h-full {class}",
            if expanded() {
                SidebarFull {
                    items: items.clone(),
                    active_id: active_id.clone(),
                    header_slot: rsx! {
                        div { class: "flex items-center justify-between",
                            span { class: "font-semibold text-xs tracking-wider uppercase text-foreground", "Navigation" }
                            {toggle_button}
                        }
                    },
                    footer_slot: footer_slot,
                }
            } else {
                SidebarRail {
                    items: items,
                    active_id: active_id,
                    header_slot: toggle_button,
                    footer_slot: footer_slot,
                }
            }
        }
    }
}
