use dioxus::prelude::*;
use crate::ui::sheet::{Sheet, SheetSide};
use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn BottomDrawerConfirm(
    open: bool,
    title: String,
    description: String,
    #[props(default = "Confirm".to_string())] confirm_label: String,
    #[props(default = false)] is_destructive: bool,
    onclose: EventHandler<()>,
    onconfirm: EventHandler<()>,
) -> Element {
    rsx! {
        Sheet {
            open: open,
            side: SheetSide::Bottom,
            onclose: move |_| onclose.call(()),
            div { class: "p-4 max-w-lg mx-auto space-y-4",
                div { class: "space-y-1 text-start",
                    h3 { class: "text-sm font-semibold text-foreground", "{title}" }
                    p { class: "text-xs text-muted-foreground", "{description}" }
                }
                div { class: "flex items-center justify-end gap-2 pt-2",
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| onclose.call(()),
                        "Cancel"
                    }
                    Button {
                        variant: if is_destructive { ButtonVariant::Destructive } else { ButtonVariant::Default },
                        onclick: move |_| onconfirm.call(()),
                        "{confirm_label}"
                    }
                }
            }
        }
    }
}
