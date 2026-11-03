use dioxus::prelude::*;
use crate::ui::button::{Button, ButtonVariant};

#[component]
pub fn InlineConfirm(
    label: String,
    #[props(default = "Yes".to_string())] confirm_label: String,
    #[props(default = "No".to_string())] cancel_label: String,
    #[props(default = ButtonVariant::Destructive)] variant: ButtonVariant,
    #[props(default = "")] class: &'static str,
    onconfirm: EventHandler<()>,
) -> Element {
    let mut asking = use_signal(|| false);

    if asking() {
        rsx! {
            div { class: "inline-flex items-center gap-1.5 animate-in fade-in-50 select-none {class}",
                span { class: "text-2xs text-muted-foreground", "Are you sure?" }
                Button {
                    variant: ButtonVariant::Destructive,
                    size: crate::ui::button::ButtonSize::Sm,
                    onclick: move |_| {
                        asking.set(false);
                        onconfirm.call(());
                    },
                    "{confirm_label}"
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: crate::ui::button::ButtonSize::Sm,
                    onclick: move |_| asking.set(false),
                    "{cancel_label}"
                }
            }
        }
    } else {
        rsx! {
            Button {
                variant: variant,
                class: class,
                onclick: move |_| asking.set(true),
                "{label}"
            }
        }
    }
}
