use dioxus::prelude::*;
use crate::ui::sheet::{Sheet, SheetSide};
use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn SideSheetForm(
    open: bool,
    title: String,
    #[props(default = None)] description: Option<String>,
    #[props(default = "Save Changes".to_string())] submit_label: String,
    #[props(default = false)] is_submitting: bool,
    onclose: EventHandler<()>,
    onsubmit: EventHandler<()>,
    children: Element,
) -> Element {
    rsx! {
        Sheet {
            open: open,
            side: SheetSide::End,
            onclose: move |_| onclose.call(()),
            div { class: "flex flex-col h-full space-y-4",
                div { class: "pb-2 border-b border-border",
                    h3 { class: "text-sm font-semibold text-foreground", "{title}" }
                    if let Some(desc) = description {
                        p { class: "text-xs text-muted-foreground pt-0.5", "{desc}" }
                    }
                }
                div { class: "flex-1 overflow-y-auto py-2 space-y-3",
                    {children}
                }
                div { class: "pt-3 border-t border-border flex justify-end gap-2",
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
}
