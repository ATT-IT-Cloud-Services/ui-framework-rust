use dioxus::prelude::*;

#[component]
pub fn FocusLayout(
    title: String,
    #[props(default = None)] subtitle: Option<String>,
    #[props(default = "")] class: &'static str,
    onclose: Option<EventHandler<()>>,
    footer: Option<Element>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 z-[500] flex flex-col bg-background text-foreground select-none overflow-hidden {class}",
            // Top minimal header
            header { class: "flex h-12 items-center justify-between border-b border-border px-4 sm:px-6 bg-background shrink-0",
                div { class: "space-y-0.5",
                    h2 { class: "text-xs font-semibold text-foreground", "{title}" }
                    if let Some(sub) = subtitle {
                        p { class: "text-[11px] text-muted-foreground", "{sub}" }
                    }
                }

                if let Some(handler) = onclose {
                    button {
                        r#type: "button",
                        class: "flex h-8 w-8 items-center justify-center rounded-md text-muted-foreground hover:bg-muted hover:text-foreground cursor-pointer text-sm",
                        onclick: move |_| handler.call(()),
                        "✕"
                    }
                }
            }

            // Distraction free body
            main { class: "flex-1 overflow-y-auto p-4 sm:p-8 max-w-4xl mx-auto w-full",
                {children}
            }

            if let Some(f) = footer {
                footer { class: "border-t border-border px-4 py-3 bg-muted/20 shrink-0",
                    div { class: "max-w-4xl mx-auto flex items-center justify-end gap-2",
                        {f}
                    }
                }
            }
        }
    }
}
