use dioxus::prelude::*;

#[component]
pub fn Slider(
    value: f64,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default = false)] show_value: bool,
    #[props(default = "")] value_suffix: &'static str,
    #[props(default = "")] class: &'static str,
    onchange: EventHandler<f64>,
) -> Element {
    let _pct = ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0);

    rsx! {
        div {
            class: "flex flex-col gap-1.5 w-full select-none {class}",
            if show_value {
                div {
                    class: "flex justify-between items-center text-2xs text-muted-foreground font-mono",
                    span { "{min}{value_suffix}" }
                    span { class: "font-semibold text-foreground", "{value}{value_suffix}" }
                    span { "{max}{value_suffix}" }
                }
            }
            div {
                class: "relative flex w-full items-center cursor-pointer h-4",
                input {
                    r#type: "range",
                    min: "{min}",
                    max: "{max}",
                    step: "{step}",
                    value: "{value}",
                    class: "w-full h-1.5 bg-muted rounded-full appearance-none cursor-pointer accent-primary",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<f64>() {
                            onchange.call(v);
                        }
                    }
                }
            }
        }
    }
}
