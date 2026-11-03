use dioxus::prelude::*;

#[component]
pub fn RadioGroup(
    #[props(default = "")] class: &'static str,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "grid gap-2 select-none {class}",
            {children}
        }
    }
}

#[component]
pub fn RadioGroupItem(
    checked: bool,
    #[props(default = "")] value: &'static str,
    #[props(default = "")] label: &'static str,
    #[props(default = "")] class: &'static str,
    oncheck: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "flex items-center gap-2 cursor-pointer select-none {class}",
            onclick: move |_| oncheck.call(()),
            div {
                class: if checked {
                    "h-4 w-4 rounded-full border border-primary bg-primary flex items-center justify-center cursor-pointer transition-colors"
                } else {
                    "h-4 w-4 rounded-full border border-input bg-background flex items-center justify-center cursor-pointer transition-colors"
                },
                if checked {
                    span { class: "h-1.5 w-1.5 rounded-full bg-white block" }
                }
            }
            if !label.is_empty() {
                span { class: "text-xs text-foreground cursor-pointer", "{label}" }
            }
        }
    }
}
