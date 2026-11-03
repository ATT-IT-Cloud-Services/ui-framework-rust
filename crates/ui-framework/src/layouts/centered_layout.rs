use dioxus::prelude::*;

#[component]
pub fn CenteredLayout(
    #[props(default = "")] class: &'static str,
    header_slot: Option<Element>,
    footer_slot: Option<Element>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "min-h-screen w-full flex flex-col items-center justify-center p-4 sm:p-6 bg-muted/30 select-none {class}",
            if let Some(h) = header_slot {
                div { class: "mb-4", {h} }
            }
            div { class: "w-full max-w-md rounded-xl border border-border bg-card p-6 sm:p-8 shadow-sm space-y-4",
                {children}
            }
            if let Some(f) = footer_slot {
                div { class: "mt-4 text-center", {f} }
            }
        }
    }
}
