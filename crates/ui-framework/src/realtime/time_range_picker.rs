use dioxus::prelude::*;

#[component]
pub fn TimeRangePicker(
    #[props(default = "live")] active_range: &'static str,
    #[props(default = "")] class: &'static str,
    onselect: Option<EventHandler<&'static str>>,
) -> Element {
    let mut selected = use_signal(|| active_range);

    let ranges = [
        ("live", "Live (50Hz)"),
        ("15m", "15m"),
        ("1h", "1h"),
        ("6h", "6h"),
        ("24h", "24h"),
    ];

    rsx! {
        div { class: "inline-flex items-center rounded-lg border border-input bg-background p-0.5 text-xs shadow-2xs select-none {class}",
            span { class: "px-2 text-2xs text-muted-foreground font-medium border-e border-border/60", "Range" }
            for (id, label) in ranges {
                {
                    let is_active = selected() == id;
                    rsx! {
                        button {
                            key: "{id}",
                            r#type: "button",
                            class: if is_active {
                                "px-2 py-0.5 rounded bg-primary text-primary-foreground font-semibold text-2xs cursor-pointer"
                            } else {
                                "px-2 py-0.5 rounded text-muted-foreground hover:bg-muted text-2xs cursor-pointer"
                            },
                            onclick: move |_| {
                                selected.set(id);
                                if let Some(h) = &onselect {
                                    h.call(id);
                                }
                            },
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}
