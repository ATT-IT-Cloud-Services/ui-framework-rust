use dioxus::prelude::*;

#[component]
pub fn MasterDetail(
    master: Element,
    detail: Element,
    selected_id: Option<String>,
    #[props(default = "")] class: &'static str,
    on_clear: Option<EventHandler<()>>,
) -> Element {
    rsx! {
        div {
            class: "h-full min-h-[550px] rounded-xl border border-border bg-card overflow-hidden grid grid-cols-1 md:grid-cols-12 {class}",

            // Master Column (e.g. 4 cols)
            div {
                class: if selected_id.is_some() {
                    "hidden md:flex md:col-span-4 flex-col border-e border-border/80 bg-background/50 overflow-y-auto"
                } else {
                    "flex col-span-12 md:col-span-4 flex-col border-e border-border/80 bg-background/50 overflow-y-auto"
                },
                {master}
            }

            // Detail Column (e.g. 8 cols)
            div {
                class: if selected_id.is_some() {
                    "flex col-span-12 md:col-span-8 flex-col bg-background overflow-y-auto p-4 sm:p-6"
                } else {
                    "hidden md:flex md:col-span-8 flex-col bg-background overflow-y-auto p-4 sm:p-6"
                },
                if let Some(handler) = on_clear {
                    div { class: "md:hidden pb-3 mb-2 border-b border-border",
                        button {
                            r#type: "button",
                            class: "text-xs font-semibold text-primary flex items-center gap-1 cursor-pointer",
                            onclick: move |_| handler.call(()),
                            "← Back to list"
                        }
                    }
                }
                {detail}
            }
        }
    }
}
