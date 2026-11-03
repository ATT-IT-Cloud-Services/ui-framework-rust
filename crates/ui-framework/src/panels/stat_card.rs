use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatTrend {
    pub value: String,
    pub is_up: bool,
}

#[component]
pub fn StatCard(
    title: String,
    value: String,
    #[props(default = None)] subtitle: Option<String>,
    #[props(default = None)] trend: Option<StatTrend>,
    #[props(default = "")] class: &'static str,
    children: Option<Element>,
) -> Element {
    rsx! {
        div {
            class: "rounded-xl border border-border bg-card p-4 text-card-foreground shadow-2xs space-y-2 select-none {class}",
            div { class: "flex items-center justify-between",
                span { class: "text-2xs font-semibold uppercase tracking-wider text-muted-foreground truncate", "{title}" }
            }

            div { class: "flex items-baseline justify-between gap-2",
                div { class: "text-xl sm:text-2xl font-bold tracking-tight text-foreground truncate", "{value}" }

                if let Some(t) = trend {
                    span {
                        class: if t.is_up {
                            "flex items-center gap-1 text-2xs font-semibold px-1.5 py-0.5 rounded text-emerald-700 bg-emerald-50 dark:text-emerald-300 dark:bg-emerald-950/50"
                        } else {
                            "flex items-center gap-1 text-2xs font-semibold px-1.5 py-0.5 rounded text-rose-700 bg-rose-50 dark:text-rose-300 dark:bg-rose-950/50"
                        },
                        if t.is_up { "▲ " } else { "▼ " }
                        "{t.value}"
                    }
                }
            }

            if let Some(sub) = subtitle {
                p { class: "text-2xs text-muted-foreground truncate", "{sub}" }
            }

            if let Some(c) = children {
                {c}
            }
        }
    }
}
