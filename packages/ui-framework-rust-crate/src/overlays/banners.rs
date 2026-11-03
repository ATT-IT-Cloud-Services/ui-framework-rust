use dioxus::prelude::*;

#[component]
pub fn AnnouncementBanner(
    title: Option<String>,
    message: String,
    #[props(default = "info")] variant: &'static str,
    #[props(default = "")] class: &'static str,
    action_label: Option<String>,
    onaction: Option<EventHandler<()>>,
) -> Element {
    let mut dismissed = use_signal(|| false);

    if dismissed() {
        return rsx! {};
    }

    let (bg, border, text) = match variant {
        "warning" => ("bg-amber-500/10", "border-amber-500/20", "text-amber-700 dark:text-amber-400"),
        "promo" => ("bg-primary/5", "border-primary/20", "text-foreground"),
        _ => ("bg-sky-500/10", "border-sky-500/20", "text-sky-700 dark:text-sky-300"),
    };

    rsx! {
        div {
            class: "flex items-center justify-between gap-3 px-4 py-2 text-xs border-b select-none {bg} {border} {text} {class}",
            div { class: "flex items-center gap-2 truncate",
                span { "ℹ" }
                if let Some(t) = title {
                    strong { "{t}:" }
                }
                span { class: "truncate", "{message}" }
                if let Some(act) = action_label {
                    button {
                        r#type: "button",
                        class: "font-semibold underline ms-2 cursor-pointer",
                        onclick: move |_| {
                            if let Some(h) = &onaction {
                                h.call(());
                            }
                        },
                        "{act}"
                    }
                }
            }

            button {
                r#type: "button",
                class: "text-muted-foreground hover:text-foreground cursor-pointer text-xs",
                onclick: move |_| dismissed.set(true),
                "✕"
            }
        }
    }
}

#[component]
pub fn InlineAlert(
    #[props(default = "info")] alert_type: &'static str,
    #[props(default = None)] title: Option<String>,
    #[props(default = "")] class: &'static str,
    children: Element,
) -> Element {
    let (border, bg, text) = match alert_type {
        "success" => ("border-emerald-500/30", "bg-emerald-500/10", "text-emerald-700 dark:text-emerald-300"),
        "warning" => ("border-amber-500/30", "bg-amber-500/10", "text-amber-700 dark:text-amber-400"),
        "error" => ("border-destructive/30", "bg-destructive/10", "text-destructive"),
        _ => ("border-sky-500/30", "bg-sky-500/10", "text-sky-700 dark:text-sky-300"),
    };

    rsx! {
        div { class: "flex items-start gap-2.5 rounded-lg border p-3 text-xs {border} {bg} {text} {class}",
            span { class: "font-bold mt-0.5", "●" }
            div { class: "space-y-0.5 flex-1",
                if let Some(t) = title {
                    strong { class: "block font-semibold", "{t}" }
                }
                div { class: "text-2xs opacity-90", {children} }
            }
        }
    }
}

#[component]
pub fn ProgressToast(
    title: String,
    progress: f64,
    #[props(default = "")] class: &'static str,
) -> Element {
    let completed = progress >= 100.0;
    rsx! {
        div { class: "rounded-xl border border-border bg-popover p-3 text-popover-foreground shadow-lg space-y-2 text-xs w-72 select-none {class}",
            div { class: "flex items-center justify-between",
                span { class: "font-semibold truncate", "{title}" }
                span { class: "font-mono font-bold text-primary", "{progress as i64}%" }
            }
            div { class: "w-full bg-muted rounded-full h-1.5 overflow-hidden",
                div {
                    class: if completed { "bg-emerald-500 h-full transition-all" } else { "bg-primary h-full transition-all" },
                    style: "width: {progress.clamp(0.0, 100.0)}%;"
                }
            }
        }
    }
}
