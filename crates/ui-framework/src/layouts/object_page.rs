use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectPageAttribute {
    pub label: String,
    pub value: String,
}

#[component]
pub fn ObjectPage(
    title: String,
    #[props(default = None)] subtitle: Option<String>,
    #[props(default = Vec::new())] attributes: Vec<ObjectPageAttribute>,
    #[props(default = "")] class: &'static str,
    status: Option<Element>,
    header_actions: Option<Element>,
    footer_action_bar: Option<Element>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "relative flex flex-col min-h-full pb-16 space-y-4 {class}",
            // 1. Enterprise Object Page Header
            div { class: "rounded-xl border border-border bg-card p-5 shadow-2xs space-y-4",
                div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3",
                    div { class: "space-y-1",
                        div { class: "flex items-center gap-2.5 flex-wrap",
                            h1 { class: "text-xl font-bold tracking-tight text-foreground", "{title}" }
                            if let Some(st) = status {
                                {st}
                            }
                        }
                        if let Some(sub) = subtitle {
                            p { class: "text-xs text-muted-foreground", "{sub}" }
                        }
                    }

                    if let Some(act) = header_actions {
                        div { class: "flex items-center gap-2 flex-wrap", {act} }
                    }
                }

                // Attributes bar
                if !attributes.is_empty() {
                    div { class: "grid grid-cols-2 sm:grid-cols-4 gap-4 pt-3 border-t border-border/60 text-xs",
                        for attr in attributes {
                            div { key: "{attr.label}", class: "space-y-0.5",
                                span { class: "text-2xs text-muted-foreground block", "{attr.label}" }
                                span { class: "font-semibold text-foreground block", "{attr.value}" }
                            }
                        }
                    }
                }
            }

            // Sections / Children
            div { class: "space-y-4",
                {children}
            }

            // Sticky footer
            if let Some(footer) = footer_action_bar {
                div { class: "fixed inset-x-0 bottom-0 z-[250] flex items-center justify-end gap-2 border-t border-border bg-background/95 backdrop-blur-md px-6 py-2.5 shadow-lg",
                    {footer}
                }
            }
        }
    }
}
