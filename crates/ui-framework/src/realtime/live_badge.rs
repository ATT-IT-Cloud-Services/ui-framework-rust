use dioxus::prelude::*;

#[component]
pub fn LiveBadge(
    #[props(default = "LIVE")] label: &'static str,
    #[props(default = true)] is_live: bool,
    #[props(default = "")] class: &'static str,
) -> Element {
    rsx! {
        span {
            class: if is_live {
                "inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wider border bg-emerald-500/10 text-emerald-700 dark:text-emerald-400 border-emerald-500/20 select-none {class}"
            } else {
                "inline-flex items-center gap-1.5 px-2 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wider border bg-muted text-muted-foreground border-border select-none {class}"
            },
            span { class: "relative flex h-2 w-2",
                if is_live {
                    span { class: "animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-500 opacity-75" }
                }
                span { class: "relative inline-flex rounded-full h-2 w-2 bg-emerald-500" }
            }
            span { "{label}" }
        }
    }
}
