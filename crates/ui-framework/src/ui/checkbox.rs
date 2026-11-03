use dioxus::prelude::*;

#[component]
pub fn Checkbox(
    checked: bool,
    #[props(default = "")] label: &'static str,
    #[props(default = "")] class: &'static str,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        label {
            class: "inline-flex items-center gap-2 cursor-pointer select-none {class}",
            div {
                class: if checked {
                    "h-4 w-4 rounded-xs border border-primary bg-primary text-primary-foreground flex items-center justify-center text-xs transition-colors"
                } else {
                    "h-4 w-4 rounded-xs border border-primary/50 bg-background flex items-center justify-center text-xs transition-colors"
                },
                onclick: move |_| onchange.call(!checked),
                if checked {
                    "✓"
                }
            }
            if !label.is_empty() {
                span { class: "text-xs text-foreground", "{label}" }
            }
        }
    }
}
