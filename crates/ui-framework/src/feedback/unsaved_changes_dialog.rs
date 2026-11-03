use dioxus::prelude::*;
use crate::ui::dialog::{Dialog, DialogDescription, DialogFooter, DialogHeader, DialogTitle};
use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn UnsavedChangesDialog(
    open: bool,
    oncancel: EventHandler<()>,
    ondiscard: EventHandler<()>,
    onsave: Option<EventHandler<()>>,
) -> Element {
    rsx! {
        Dialog {
            open: open,
            onclose: move |_| oncancel.call(()),
            DialogHeader {
                DialogTitle { class: "flex items-center gap-2 text-amber-600",
                    span { "⚠" }
                    span { "Unsaved Changes" }
                }
                DialogDescription { "You have unsaved changes that will be lost if you leave without saving. How would you like to proceed?" }
            }
            DialogFooter {
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| oncancel.call(()),
                    "Cancel"
                }
                Button {
                    variant: ButtonVariant::Destructive,
                    onclick: move |_| ondiscard.call(()),
                    "Discard Changes"
                }
                if let Some(h) = onsave {
                    Button {
                        variant: ButtonVariant::Default,
                        onclick: move |_| h.call(()),
                        "Save & Continue"
                    }
                }
            }
        }
    }
}
