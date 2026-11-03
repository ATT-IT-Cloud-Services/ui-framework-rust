use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemStatus {
    #[default]
    Ok,
    Warn,
    Alarm,
    Unknown,
    Offline,
}

#[component]
pub fn StatusIndicator(
    status: SystemStatus,
    label: &'static str,
    #[props(default = "")] class: &'static str,
) -> Element {
    let color_class = match status {
        SystemStatus::Ok => "bg-emerald-500 ring-emerald-500/20",
        SystemStatus::Warn => "bg-amber-500 ring-amber-500/20",
        SystemStatus::Alarm => "bg-rose-500 ring-rose-500/20",
        SystemStatus::Unknown => "bg-slate-400 ring-slate-400/20",
        SystemStatus::Offline => "bg-zinc-600 ring-zinc-600/20",
    };

    rsx! {
        div { class: "inline-flex items-center gap-2 text-xs select-none {class}",
            span { class: "rounded-full ring-4 h-2.5 w-2.5 {color_class}" }
            span { class: "font-medium text-foreground", "{label}" }
        }
    }
}
