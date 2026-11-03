use dioxus::prelude::*;
use crate::ui::button::Button;

#[component]
pub fn EmptyState(
    #[props(default = "No data available".to_string())] title: String,
    #[props(default = "There are no records matching your criteria.".to_string())] description: String,
    #[props(default = "")] class: &'static str,
    action_label: Option<String>,
    onaction: Option<EventHandler<()>>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center p-8 text-center rounded-xl border border-dashed border-border/80 bg-muted/10 space-y-3 {class}",
            div { class: "flex h-12 w-12 items-center justify-center rounded-full bg-muted text-muted-foreground/80 text-xl font-bold", "📂" }

            div { class: "space-y-1 max-w-sm",
                h3 { class: "text-sm font-semibold tracking-tight text-foreground", "{title}" }
                p { class: "text-xs text-muted-foreground leading-relaxed", "{description}" }
            }

            if let Some(label) = action_label {
                div { class: "pt-2",
                    Button {
                        onclick: move |_| {
                            if let Some(h) = &onaction {
                                h.call(());
                            }
                        },
                        "{label}"
                    }
                }
            }
        }
    }
}
