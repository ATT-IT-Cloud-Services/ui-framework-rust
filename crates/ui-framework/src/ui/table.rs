use dioxus::prelude::*;

#[component]
pub fn Table(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "relative w-full overflow-auto",
            table { class: "w-full caption-bottom text-xs {class}", {children} }
        }
    }
}

#[component]
pub fn TableHeader(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! { thead { class: "[&_tr]:border-b bg-muted/30 {class}", {children} } }
}

#[component]
pub fn TableBody(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! { tbody { class: "[&_tr:last-child]:border-0 {class}", {children} } }
}

#[component]
pub fn TableRow(
    #[props(default = false)] selected: bool,
    #[props(default = "")] class: &'static str,
    children: Element,
) -> Element {
    let sel_class = if selected { "bg-primary/5" } else { "hover:bg-muted/40" };
    rsx! { tr { class: "border-b border-border/60 transition-colors {sel_class} {class}", {children} } }
}

#[component]
pub fn TableHead(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! { th { class: "h-8 px-3 text-start align-middle font-semibold text-muted-foreground {class}", {children} } }
}

#[component]
pub fn TableCell(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! { td { class: "p-3 align-middle {class}", {children} } }
}
