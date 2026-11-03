use dioxus::prelude::*;

#[component]
pub fn PageHeader(
    title: String,
    #[props(default = None)] subtitle: Option<String>,
    #[props(default = "")] class: &'static str,
    badge: Option<Element>,
    actions: Option<Element>,
    tabs: Option<Element>,
) -> Element {
    rsx! {
        div {
            class: "border-b border-border/80 bg-background/50 pb-3 pt-1 space-y-3 {class}",
            div {
                class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3",
                div {
                    class: "space-y-1",
                    div {
                        class: "flex items-center gap-2 flex-wrap",
                        h1 { class: "text-lg font-semibold tracking-tight text-foreground", "{title}" }
                        if let Some(b) = badge {
                            {b}
                        }
                    }
                    if let Some(sub) = subtitle {
                        p { class: "text-xs text-muted-foreground", "{sub}" }
                    }
                }

                if let Some(act) = actions {
                    div { class: "flex items-center gap-2 flex-wrap", {act} }
                }
            }

            if let Some(t) = tabs {
                div { class: "pt-1", {t} }
            }
        }
    }
}
