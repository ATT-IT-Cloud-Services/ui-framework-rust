use dioxus::prelude::*;

#[component]
pub fn Popover(
    open: bool,
    #[props(default = "")] class: &'static str,
    onclose: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    if !open {
        return rsx! {};
    }

    rsx! {
        div {
            class: "fixed inset-0 z-[600] bg-transparent",
            onclick: move |_| {
                if let Some(h) = &onclose {
                    h.call(());
                }
            }
        }
        div {
            class: "absolute z-[600] w-72 rounded-lg border border-border bg-popover p-3 text-popover-foreground shadow-md animate-in fade-in-50 zoom-in-95 {class}",
            {children}
        }
    }
}
