use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InfoCardVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
    Promo,
}

#[component]
pub fn InfoCard(
    title: String,
    #[props(default = None)] description: Option<String>,
    #[props(default)] variant: InfoCardVariant,
    #[props(default = false)] dismissible: bool,
    #[props(default = "")] class: &'static str,
    action_label: Option<String>,
    onaction: Option<EventHandler<()>>,
) -> Element {
    let mut dismissed = use_signal(|| false);

    if dismissed() {
        return rsx! {};
    }

    let (border, bg, text, icon) = match variant {
        InfoCardVariant::Info => ("border-sky-200 dark:border-sky-900/60", "bg-sky-50/70 dark:bg-sky-950/30", "text-sky-900 dark:text-sky-200", "ℹ"),
        InfoCardVariant::Success => ("border-emerald-200 dark:border-emerald-900/60", "bg-emerald-50/70 dark:bg-emerald-950/30", "text-emerald-900 dark:text-emerald-200", "✓"),
        InfoCardVariant::Warning => ("border-amber-200 dark:border-amber-900/60", "bg-amber-50/70 dark:bg-amber-950/30", "text-amber-900 dark:text-amber-200", "⚠"),
        InfoCardVariant::Error => ("border-rose-200 dark:border-rose-900/60", "bg-rose-50/70 dark:bg-rose-950/30", "text-rose-900 dark:text-rose-200", "✕"),
        InfoCardVariant::Promo => ("border-primary/30", "bg-primary/5 dark:bg-primary/10", "text-foreground", "✦"),
    };

    rsx! {
        div {
            class: "relative flex items-start gap-3 rounded-lg border p-3.5 text-xs shadow-2xs transition-colors {border} {bg} {text} {class}",
            span { class: "font-bold text-sm shrink-0 mt-0.5", "{icon}" }

            div { class: "flex-1 space-y-1",
                h4 { class: "font-semibold text-foreground text-xs", "{title}" }
                if let Some(desc) = description {
                    p { class: "text-2xs opacity-90 leading-relaxed", "{desc}" }
                }

                if let Some(label) = action_label {
                    div { class: "pt-1.5",
                        button {
                            r#type: "button",
                            class: "h-6 px-2.5 rounded bg-primary text-primary-foreground text-2xs font-semibold cursor-pointer",
                            onclick: move |_| {
                                if let Some(h) = &onaction {
                                    h.call(());
                                }
                            },
                            "{label}"
                        }
                    }
                }
            }

            if dismissible {
                button {
                    r#type: "button",
                    class: "text-muted-foreground hover:text-foreground opacity-70 hover:opacity-100 cursor-pointer p-0.5 text-xs",
                    onclick: move |_| dismissed.set(true),
                    "✕"
                }
            }
        }
    }
}
