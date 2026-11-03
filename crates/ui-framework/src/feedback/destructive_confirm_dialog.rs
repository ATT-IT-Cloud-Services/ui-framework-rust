use dioxus::prelude::*;
use crate::ui::dialog::{Dialog, DialogDescription, DialogFooter, DialogHeader, DialogTitle};
use crate::ui::button::{Button, ButtonVariant};
use crate::ui::input::Input;

#[component]
pub fn DestructiveConfirmDialog(
    open: bool,
    title: String,
    description: String,
    #[props(default = None)] required_text: Option<String>,
    #[props(default = "Delete Permanently".to_string())] confirm_label: String,
    onclose: EventHandler<()>,
    onconfirm: EventHandler<()>,
) -> Element {
    let mut typed = use_signal(String::new);

    let is_valid = if let Some(req) = &required_text {
        typed() == *req
    } else {
        true
    };

    rsx! {
        Dialog {
            open: open,
            onclose: move |_| onclose.call(()),
            DialogHeader {
                DialogTitle { class: "text-destructive flex items-center gap-2",
                    span { "⚠" }
                    span { "{title}" }
                }
                DialogDescription { "{description}" }
            }

            if let Some(req) = required_text {
                div { class: "space-y-2 py-2 text-xs",
                    label { class: "text-2xs text-muted-foreground block",
                        "Please type "
                        strong { class: "text-foreground font-mono", "{req}" }
                        " to confirm:"
                    }
                    Input {
                        value: "{typed()}",
                        oninput: move |e: FormEvent| typed.set(e.value()),
                    }
                }
            }

            DialogFooter {
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| onclose.call(()),
                    "Cancel"
                }
                Button {
                    variant: ButtonVariant::Destructive,
                    disabled: !is_valid,
                    onclick: move |_| onconfirm.call(()),
                    "{confirm_label}"
                }
            }
        }
    }
}
