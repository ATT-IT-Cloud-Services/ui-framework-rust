use dioxus::prelude::*;
use crate::ui::dialog::{Dialog, DialogDescription, DialogFooter, DialogHeader, DialogTitle};
use crate::ui::button::Button;

#[component]
pub fn SessionExpiredDialog(
    open: bool,
    #[props(default = "Sign in again".to_string())] action_label: String,
    onaction: EventHandler<()>,
) -> Element {
    rsx! {
        Dialog {
            open: open,
            DialogHeader {
                DialogTitle { class: "flex items-center gap-2 text-primary",
                    span { "🔒" }
                    span { "Session Expired" }
                }
                DialogDescription { "Your session has expired due to inactivity. Please sign in again to continue without losing your workspace state." }
            }
            DialogFooter {
                Button {
                    class: "w-full text-xs font-semibold",
                    onclick: move |_| onaction.call(()),
                    "{action_label}"
                }
            }
        }
    }
}
