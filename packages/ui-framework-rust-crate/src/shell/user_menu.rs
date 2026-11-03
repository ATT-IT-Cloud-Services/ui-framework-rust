use dioxus::prelude::*;
use crate::adapters::auth_adapter::use_session;

#[component]
pub fn UserMenu(
    #[props(default = "")] class: &'static str,
    onsignout: Option<EventHandler<()>>,
) -> Element {
    let session = use_session();
    let mut open = use_signal(|| false);

    let user_name = session
        .read()
        .user
        .as_ref()
        .map(|u| u.name.clone())
        .unwrap_or_else(|| "Operator".to_string());

    let user_email = session
        .read()
        .user
        .as_ref()
        .map(|u| u.email.clone())
        .unwrap_or_else(|| "operator@enterprise.internal".to_string());

    let initials = user_name.chars().next().unwrap_or('O').to_string();

    rsx! {
        div { class: "relative inline-block {class}",
            button {
                r#type: "button",
                class: "flex h-7 w-7 items-center justify-center rounded-full bg-primary/10 text-primary font-bold text-xs border border-border cursor-pointer select-none",
                onclick: move |_| open.set(!open()),
                "{initials}"
            }

            if open() {
                div {
                    class: "absolute end-0 mt-2 w-56 p-2 rounded-lg border border-border bg-popover text-popover-foreground shadow-lg z-[600] animate-in fade-in-50 text-xs select-none",
                    div { class: "p-2 border-b border-border/60 mb-1 space-y-0.5",
                        p { class: "font-semibold text-foreground truncate", "{user_name}" }
                        p { class: "text-2xs text-muted-foreground truncate", "{user_email}" }
                    }

                    button {
                        r#type: "button",
                        class: "w-full text-start px-2 py-1.5 rounded hover:bg-muted text-foreground cursor-pointer",
                        onclick: move |_| open.set(false),
                        "User Profile"
                    }

                    if let Some(handler) = onsignout {
                        button {
                            r#type: "button",
                            class: "w-full text-start px-2 py-1.5 rounded hover:bg-destructive/10 text-destructive cursor-pointer border-t border-border/50 mt-1",
                            onclick: move |_| {
                                open.set(false);
                                handler.call(());
                            },
                            "Sign out"
                        }
                    }
                }
            }
        }
    }
}
