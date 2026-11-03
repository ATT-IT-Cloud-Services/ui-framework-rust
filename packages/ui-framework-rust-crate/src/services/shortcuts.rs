use dioxus::prelude::*;
use crate::ui::dialog::{Dialog, DialogDescription, DialogHeader, DialogTitle};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutItem {
    pub key: String,
    pub description: String,
    pub category: String,
}

#[component]
pub fn ShortcutsHelpDialog(
    open: bool,
    shortcuts: Vec<ShortcutItem>,
    onclose: EventHandler<()>,
) -> Element {
    rsx! {
        Dialog {
            open: open,
            onclose: move |_| onclose.call(()),
            DialogHeader {
                DialogTitle { "Keyboard Shortcuts" }
                DialogDescription { "Press keys anywhere to trigger common actions." }
            }
            div { class: "py-3 space-y-2 max-h-80 overflow-y-auto text-xs",
                for sc in shortcuts {
                    div {
                        key: "{sc.key}",
                        class: "flex items-center justify-between p-2 rounded-lg bg-muted/20 border border-border/50",
                        span { "{sc.description}" }
                        kbd { class: "px-2 py-0.5 rounded bg-muted border border-border font-mono text-2xs font-semibold uppercase",
                            "{sc.key}"
                        }
                    }
                }
            }
        }
    }
}
