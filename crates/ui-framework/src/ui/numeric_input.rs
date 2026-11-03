use dioxus::prelude::*;

#[component]
pub fn NumericInput(
    value: f64,
    #[props(default = 0.0)] min: f64,
    #[props(default = 1000000.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default = "")] unit: &'static str,
    #[props(default = "")] class: &'static str,
    onchange: EventHandler<f64>,
) -> Element {
    rsx! {
        div {
            class: "inline-flex items-center rounded-md border border-input bg-background shadow-2xs h-8 text-xs select-none {class}",
            button {
                r#type: "button",
                disabled: value <= min,
                class: "flex h-full w-7 items-center justify-center border-e border-border text-muted-foreground hover:bg-muted disabled:opacity-30 cursor-pointer",
                onclick: move |_| {
                    if value - step >= min {
                        onchange.call(value - step);
                    }
                },
                "−"
            }
            div {
                class: "flex items-center px-2",
                span { class: "font-mono font-medium text-center w-12", "{value}" }
                if !unit.is_empty() {
                    span { class: "text-2xs text-muted-foreground ms-1", "{unit}" }
                }
            }
            button {
                r#type: "button",
                disabled: value >= max,
                class: "flex h-full w-7 items-center justify-center border-s border-border text-muted-foreground hover:bg-muted disabled:opacity-30 cursor-pointer",
                onclick: move |_| {
                    if value + step <= max {
                        onchange.call(value + step);
                    }
                },
                "+"
            }
        }
    }
}
