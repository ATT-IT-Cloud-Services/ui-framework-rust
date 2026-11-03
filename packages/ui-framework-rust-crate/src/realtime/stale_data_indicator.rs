use dioxus::prelude::*;

#[component]
pub fn StaleDataIndicator(
    is_stale: bool,
    #[props(default = "")] class: &'static str,
) -> Element {
    if !is_stale {
        return rsx! {};
    }

    rsx! {
        div { class: "inline-flex items-center gap-1.5 px-2 py-0.5 rounded text-2xs font-medium border bg-amber-500/10 border-amber-500/20 text-amber-700 dark:text-amber-400 select-none {class}",
            span { "🕒" }
            span { "Stale Data" }
        }
    }
}
