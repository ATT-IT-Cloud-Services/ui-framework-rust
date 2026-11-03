use dioxus::prelude::*;

#[component]
pub fn Switch(
    checked: bool,
    #[props(default = "")] class: &'static str,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: if checked {
                "inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent bg-primary transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring {class}"
            } else {
                "inline-flex h-5 w-9 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent bg-input transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring {class}"
            },
            onclick: move |_| onchange.call(!checked),
            span {
                class: if checked {
                    "pointer-events-none block h-4 w-4 rounded-full bg-background shadow-xs ring-0 transition-transform translate-x-4 rtl:-translate-x-4"
                } else {
                    "pointer-events-none block h-4 w-4 rounded-full bg-background shadow-xs ring-0 transition-transform translate-x-0"
                }
            }
        }
    }
}
