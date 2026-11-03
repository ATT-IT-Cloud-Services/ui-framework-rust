use dioxus::prelude::*;

#[component]
pub fn SinglePane(
    header: Option<Element>,
    footer: Option<Element>,
    #[props(default = "")] class: &'static str,
    children: Element,
) -> Element {
    rsx! {
        div { class: "flex flex-col min-h-full space-y-4 {class}",
            if let Some(h) = header {
                div { {h} }
            }
            div { class: "flex-1", {children} }
            if let Some(f) = footer {
                div { class: "pt-4 border-t border-border/60", {f} }
            }
        }
    }
}
