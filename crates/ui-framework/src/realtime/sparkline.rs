use dioxus::prelude::*;

#[component]
pub fn Sparkline(
    data: Vec<f64>,
    #[props(default = 100)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(default = "currentColor")] color: &'static str,
    #[props(default = "")] class: &'static str,
) -> Element {
    if data.len() < 2 {
        return rsx! { div { class: "bg-muted/20 {class}", style: "width: {width}px; height: {height}px;" } };
    }

    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max - min).max(1.0);

    let points = data
        .iter()
        .enumerate()
        .map(|(idx, val)| {
            let x = (idx as f64 / (data.len() - 1) as f64) * (width as f64 - 4.0) + 2.0;
            let y = height as f64 - ((val - min) / range) * (height as f64 - 6.0) - 3.0;
            format!("{:.1},{:.1}", x, y)
        })
        .collect::<Vec<_>>()
        .join(" ");

    rsx! {
        svg {
            width: "{width}",
            height: "{height}",
            class: "overflow-visible select-none {class}",
            polyline {
                fill: "none",
                stroke: "{color}",
                stroke_width: "1.5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                points: "{points}",
            }
        }
    }
}
