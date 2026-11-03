use dioxus::prelude::*;

#[component]
pub fn TabsList(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "inline-flex h-8 items-center justify-center rounded-md bg-muted p-0.5 text-muted-foreground {class}", {children} }
    }
}

#[component]
pub fn TabsTrigger(
    active: bool,
    #[props(default = "")] class: &'static str,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: if active {
                "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-2.5 py-1 text-xs font-medium transition-all bg-background text-foreground shadow-2xs cursor-pointer {class}"
            } else {
                "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-2.5 py-1 text-xs font-medium transition-all text-muted-foreground hover:text-foreground cursor-pointer {class}"
            },
            onclick: move |e| onclick.call(e),
            {children}
        }
    }
}

#[component]
pub fn TabsContent(
    active: bool,
    #[props(default = "")] class: &'static str,
    children: Element,
) -> Element {
    if !active {
        return rsx! {};
    }
    rsx! {
        div { class: "mt-2 {class}", {children} }
    }
}
