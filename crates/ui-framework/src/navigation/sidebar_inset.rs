use dioxus::prelude::*;
use super::types::NavigationItem;
use super::sidebar_full::SidebarFull;

#[component]
pub fn SidebarInset(
    items: Vec<NavigationItem>,
    #[props(default = String::new())] active_id: String,
    #[props(default = "")] class: &'static str,
    header_slot: Option<Element>,
    footer_slot: Option<Element>,
) -> Element {
    rsx! {
        div { class: "p-2.5 h-full shrink-0 {class}",
            SidebarFull {
                items: items,
                active_id: active_id,
                header_slot: header_slot,
                footer_slot: footer_slot,
                class: "rounded-xl border border-border shadow-2xs h-full overflow-hidden",
            }
        }
    }
}
