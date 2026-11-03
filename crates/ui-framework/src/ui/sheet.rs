use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SheetSide {
    Top,
    Bottom,
    Start,
    #[default]
    End,
}

#[component]
pub fn Sheet(
    open: bool,
    #[props(default)] side: SheetSide,
    #[props(default = "")] class: &'static str,
    onclose: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    if !open {
        return rsx! {};
    }

    let side_class = match side {
        SheetSide::Top => "inset-x-0 top-0 border-b border-border animate-in slide-in-from-top",
        SheetSide::Bottom => "inset-x-0 bottom-0 border-t border-border animate-in slide-in-from-bottom",
        SheetSide::Start => "inset-y-0 start-0 h-full w-3/4 sm:max-w-md border-e border-border animate-in slide-in-from-left rtl:slide-in-from-right",
        SheetSide::End => "inset-y-0 end-0 h-full w-3/4 sm:max-w-md border-s border-border animate-in slide-in-from-right rtl:slide-in-from-left",
    };

    rsx! {
        // Overlay
        div {
            class: "fixed inset-0 z-[400] bg-black/40 backdrop-blur-xs transition-opacity duration-150 animate-in fade-in-0",
            onclick: move |_| {
                if let Some(h) = &onclose {
                    h.call(());
                }
            }
        }
        // Sheet Content
        div {
            class: "fixed z-[400] bg-background p-5 shadow-2xl transition ease-in-out duration-200 {side_class} {class}",
            {children}
        }
    }
}
