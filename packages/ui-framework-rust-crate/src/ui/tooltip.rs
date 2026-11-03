use dioxus::prelude::*;

#[component]
pub fn Tooltip(
    text: &'static str,
    #[props(default = "")] class: &'static str,
    children: Element,
) -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        div {
            class: "relative inline-block {class}",
            onmouseenter: move |_| visible.set(true),
            onmouseleave: move |_| visible.set(false),
            {children}
            if visible() {
                div {
                    class: "absolute bottom-full start-1/2 -translate-x-1/2 mb-1.5 z-[800] overflow-hidden rounded bg-foreground px-2 py-0.5 text-[10px] font-medium text-background shadow-md animate-in fade-in-0 pointer-events-none whitespace-nowrap",
                    "{text}"
                }
            }
        }
    }
}
