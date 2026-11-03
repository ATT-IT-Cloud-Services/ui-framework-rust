use dioxus::prelude::*;

#[component]
pub fn Separator(
    #[props(default = "horizontal")] orientation: &'static str,
    #[props(default = "")] class: &'static str,
) -> Element {
    let orientation_class = if orientation == "vertical" {
        "h-full w-[1px]"
    } else {
        "h-[1px] w-full"
    };

    rsx! {
        div { class: "shrink-0 bg-border {orientation_class} {class}" }
    }
}
