use dioxus::prelude::*;
use crate::settings::component_customizer::use_component_style;

#[component]
pub fn Card(
    #[props(default = "")] class: &'static str,
    custom_id: Option<&'static str>,
    children: Element,
) -> Element {
    let custom_style = use_component_style("card", custom_id);
    let custom_class_str = custom_style.class_name.unwrap_or_default();

    rsx! {
        div {
            class: "rounded-lg border border-border bg-card text-card-foreground shadow-2xs {custom_class_str} {class}",
            {children}
        }
    }
}

#[component]
pub fn CardHeader(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! { div { class: "flex flex-col space-y-1.5 p-4 {class}", {children} } }
}

#[component]
pub fn CardTitle(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! { h3 { class: "text-sm font-semibold leading-none tracking-tight text-foreground {class}", {children} } }
}

#[component]
pub fn CardDescription(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! { p { class: "text-xs text-muted-foreground {class}", {children} } }
}

#[component]
pub fn CardContent(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! { div { class: "p-4 pt-0 {class}", {children} } }
}

#[component]
pub fn CardFooter(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! { div { class: "flex items-center p-4 pt-0 {class}", {children} } }
}
