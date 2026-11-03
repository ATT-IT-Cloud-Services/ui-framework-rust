use dioxus::prelude::*;

#[component]
pub fn ThreePane(
    left_pane: Element,
    center_pane: Element,
    right_pane: Element,
    #[props(default = "")] class: &'static str,
) -> Element {
    rsx! {
        div {
            class: "h-full min-h-[550px] rounded-xl border border-border bg-card overflow-hidden grid grid-cols-1 lg:grid-cols-12 {class}",
            // Left Pane
            div { class: "lg:col-span-3 border-e border-border/70 bg-muted/20 overflow-y-auto p-3",
                {left_pane}
            }
            // Center Pane
            div { class: "lg:col-span-4 border-e border-border/70 bg-background/50 overflow-y-auto p-3",
                {center_pane}
            }
            // Right Pane
            div { class: "lg:col-span-5 bg-background overflow-y-auto p-4 sm:p-6",
                {right_pane}
            }
        }
    }
}
