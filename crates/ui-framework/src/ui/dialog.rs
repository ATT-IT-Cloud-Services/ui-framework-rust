use dioxus::prelude::*;

#[component]
pub fn Dialog(
    open: bool,
    #[props(default = "")] class: &'static str,
    onclose: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    if !open {
        return rsx! {};
    }

    rsx! {
        // Overlay
        div {
            class: "fixed inset-0 z-[500] bg-black/40 backdrop-blur-xs transition-opacity duration-150 animate-in fade-in-0",
            onclick: move |_| {
                if let Some(h) = &onclose {
                    h.call(());
                }
            }
        }
        // Content
        div {
            class: "fixed left-[50%] top-[50%] z-[500] grid w-full max-w-lg translate-x-[-50%] translate-y-[-50%] gap-3 border border-border bg-background p-6 shadow-xl duration-150 rounded-xl animate-in zoom-in-95 {class}",
            {children}
        }
    }
}

#[component]
pub fn DialogHeader(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! { div { class: "flex flex-col space-y-1.5 text-start {class}", {children} } }
}

#[component]
pub fn DialogTitle(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! { h3 { class: "text-base font-semibold leading-none tracking-tight text-foreground {class}", {children} } }
}

#[component]
pub fn DialogDescription(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! { p { class: "text-xs text-muted-foreground {class}", {children} } }
}

#[component]
pub fn DialogFooter(#[props(default = "")] class: &'static str, children: Element) -> Element {
    rsx! { div { class: "flex flex-col-reverse sm:flex-row sm:justify-end gap-2 pt-3 border-t border-border/50 {class}", {children} } }
}
