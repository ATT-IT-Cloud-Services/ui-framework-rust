use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DataQuality {
    #[default]
    Good,
    Uncertain,
    Bad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ValueTrend {
    #[default]
    Stable,
    Up,
    Down,
}

#[component]
pub fn LiveValue(
    value: String,
    #[props(default = None)] label: Option<String>,
    #[props(default = "")] unit: &'static str,
    #[props(default)] trend: ValueTrend,
    #[props(default)] quality: DataQuality,
    #[props(default = "")] class: &'static str,
) -> Element {
    let quality_class = match quality {
        DataQuality::Good => "text-foreground",
        DataQuality::Uncertain => "text-amber-600 dark:text-amber-400",
        DataQuality::Bad => "text-rose-600 line-through opacity-70",
    };

    let trend_symbol = match trend {
        ValueTrend::Up => "▲",
        ValueTrend::Down => "▼",
        ValueTrend::Stable => "—",
    };

    let trend_color = match trend {
        ValueTrend::Up => "text-emerald-500",
        ValueTrend::Down => "text-rose-500",
        ValueTrend::Stable => "text-muted-foreground",
    };

    rsx! {
        div { class: "inline-flex flex-col space-y-0.5 select-none {class}",
            if let Some(l) = label {
                span { class: "text-2xs font-medium text-muted-foreground", "{l}" }
            }

            div { class: "flex items-baseline gap-1.5 font-mono",
                span { class: "text-xl font-bold tracking-tight {quality_class}", "{value}" }
                if !unit.is_empty() {
                    span { class: "text-xs text-muted-foreground font-sans font-medium", "{unit}" }
                }
                span { class: "text-2xs ms-1 {trend_color}", "{trend_symbol}" }
                if quality != DataQuality::Good {
                    span { class: "text-2xs text-amber-500 ms-0.5", "⚠" }
                }
            }
        }
    }
}
