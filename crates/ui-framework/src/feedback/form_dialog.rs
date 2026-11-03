use dioxus::prelude::*;
use crate::ui::dialog::{Dialog, DialogDescription, DialogFooter, DialogHeader, DialogTitle};
use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn FormDialog(
    open: bool,
    title: String,
    #[props(default = None)] description: Option<String>,
    #[props(default = "Save".to_string())] submit_label: String,
    #[props(default = false)] is_submitting: bool,
    onclose: EventHandler<()>,
    onsubmit: EventHandler<()>,
    children: Element,
) -> Element {
    rsx! {
        Dialog {
            open: open,
            onclose: move |_| onclose.call(()),
            DialogHeader {
                DialogTitle { "{title}" }
                if let Some(desc) = description {
                    DialogDescription { "{desc}" }
                }
            }
            div { class: "py-2 space-y-3",
                {children}
            }
            DialogFooter {
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| onclose.call(()),
                    "Cancel"
                }
                Button {
                    variant: ButtonVariant::Default,
                    disabled: is_submitting,
                    onclick: move |_| onsubmit.call(()),
                    if is_submitting { "Saving..." } else { "{submit_label}" }
                }
            }
        }
    }
}
