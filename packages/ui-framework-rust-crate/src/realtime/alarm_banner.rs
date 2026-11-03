use dioxus::prelude::*;
use crate::ui::button::Button;

#[component]
pub fn AlarmBanner(
    source: String,
    message: String,
    #[props(default = "")] class: &'static str,
    onacknowledge: Option<EventHandler<()>>,
) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-between gap-3 px-4 py-2 bg-rose-600 text-white shadow-md text-xs select-none animate-in fade-in-50 {class}",
            div { class: "flex items-center gap-2.5 truncate",
                span { class: "font-bold text-base", "⚠" }
                span { class: "font-bold uppercase tracking-wider text-[11px] bg-white/20 px-1.5 py-0.5 rounded", "CRITICAL" }
                span { class: "font-semibold truncate", "{source}:" }
                span { class: "truncate opacity-95", "{message}" }
            }

            if let Some(h) = onacknowledge {
                Button {
                    class: "h-6 text-2xs bg-white text-rose-700 hover:bg-white/90 font-semibold px-2.5",
                    onclick: move |_| h.call(()),
                    "Acknowledge"
                }
            }
        }
    }
}
