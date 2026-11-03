use dioxus::prelude::*;

#[component]
pub fn FullSplit(
    nav_pane: Element,
    submenu_pane: Element,
    content_pane: Element,
    info_pane: Element,
    #[props(default = "")] class: &'static str,
) -> Element {
    rsx! {
        div {
            class: "h-full min-h-[600px] rounded-xl border border-border bg-card overflow-hidden grid grid-cols-1 lg:grid-cols-12 {class}",
            div { class: "lg:col-span-2 border-e border-border/70 bg-sidebar overflow-y-auto p-2",
                {nav_pane}
            }
            div { class: "lg:col-span-2 border-e border-border/70 bg-muted/20 overflow-y-auto p-3",
                {submenu_pane}
            }
            div { class: "lg:col-span-5 bg-background overflow-y-auto p-4",
                {content_pane}
            }
            div { class: "lg:col-span-3 border-s border-border/70 bg-background/50 overflow-y-auto p-3",
                {info_pane}
            }
        }
    }
}
