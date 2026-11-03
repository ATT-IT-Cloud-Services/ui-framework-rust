use dioxus::prelude::*;
use crate::navigation::types::NavigationItem;

#[component]
pub fn CommandPalette(
    open: bool,
    items: Vec<NavigationItem>,
    onclose: EventHandler<()>,
) -> Element {
    let mut search_query = use_signal(String::new);

    if !open {
        return rsx! {};
    }

    let filtered = items
        .iter()
        .filter(|i| {
            let q = search_query().to_lowercase();
            i.label.to_lowercase().contains(&q) || i.section.as_ref().map(|s| s.to_lowercase().contains(&q)).unwrap_or(false)
        })
        .cloned()
        .collect::<Vec<_>>();

    rsx! {
        div {
            class: "fixed inset-0 z-[600] bg-black/40 backdrop-blur-xs flex items-start justify-center pt-20 p-4 animate-in fade-in-0",
            onclick: move |_| onclose.call(()),
            div {
                class: "w-full max-w-lg rounded-xl border border-border bg-popover text-popover-foreground shadow-2xl overflow-hidden select-none",
                onclick: move |e| e.stop_propagation(),

                div { class: "flex items-center border-b border-border px-3.5 h-11",
                    span { class: "me-2 text-muted-foreground", "🔍" }
                    input {
                        r#type: "text",
                        value: "{search_query()}",
                        placeholder: "Type a command or search navigation...",
                        class: "w-full bg-transparent text-xs outline-none placeholder:text-muted-foreground",
                        oninput: move |e| search_query.set(e.value()),
                    }
                }

                div { class: "max-h-[300px] overflow-y-auto p-1.5 space-y-0.5",
                    if filtered.is_empty() {
                        div { class: "py-6 text-center text-xs text-muted-foreground", "No matching commands found." }
                    } else {
                        for item in filtered {
                            div {
                                key: "{item.id}",
                                class: "flex items-center justify-between px-3 py-2 rounded text-xs hover:bg-muted text-foreground cursor-pointer transition-colors",
                                onclick: move |_| onclose.call(()),
                                span { "{item.label}" }
                                if let Some(sec) = &item.section {
                                    span { class: "text-2xs text-muted-foreground", "{sec}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
