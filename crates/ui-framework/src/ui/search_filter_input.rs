use dioxus::prelude::*;

#[component]
pub fn SearchFilterInput(
    value: String,
    #[props(default = "Search items...")] placeholder: &'static str,
    #[props(default = "")] class: &'static str,
    onchange: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            class: "relative flex items-center w-full rounded-md border border-input bg-background shadow-2xs h-8 text-xs {class}",
            span { class: "ms-2.5 text-muted-foreground", "🔍" }
            input {
                r#type: "text",
                value: "{value}",
                placeholder: "{placeholder}",
                class: "w-full bg-transparent px-2 text-xs outline-none placeholder:text-muted-foreground",
                oninput: move |e| onchange.call(e.value()),
            }
            if !value.is_empty() {
                button {
                    r#type: "button",
                    class: "me-2 text-muted-foreground hover:text-foreground cursor-pointer text-xs",
                    onclick: move |_| onchange.call("".to_string()),
                    "✕"
                }
            }
        }
    }
}
