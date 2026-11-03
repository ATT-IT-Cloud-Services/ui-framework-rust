use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FooterVariant {
    #[default]
    Default,
    Minimal,
    Status,
    Dense,
}

#[component]
pub fn AppFooter(
    #[props(default)] variant: FooterVariant,
    #[props(default = false)] sticky: bool,
    #[props(default = true)] border: bool,
    #[props(default = "")] class: &'static str,
    left_slot: Option<Element>,
    center_slot: Option<Element>,
    right_slot: Option<Element>,
    children: Option<Element>,
) -> Element {
    let height_class = match variant {
        FooterVariant::Default => "h-10 text-xs px-4 sm:px-6",
        FooterVariant::Minimal => "h-8 text-[11px] px-4",
        FooterVariant::Status => "h-9 text-[11px] px-4 sm:px-6 font-mono",
        FooterVariant::Dense => "h-7 text-[10px] px-3 font-mono",
    };

    let border_class = if border { "border-t border-border/80" } else { "" };
    let sticky_class = if sticky { "sticky bottom-0 z-[300]" } else { "" };

    rsx! {
        footer {
            class: "flex w-full items-center justify-between bg-background/95 backdrop-blur-md text-muted-foreground select-none shrink-0 {height_class} {border_class} {sticky_class} {class}",

            div { class: "flex items-center gap-3 truncate",
                if let Some(left) = left_slot {
                    {left}
                } else {
                    span { class: "truncate", "© ATT IT / Philippe Simon Pflug. All rights reserved." }
                }
            }

            if let Some(center) = center_slot {
                div { class: "hidden md:flex items-center gap-2", {center} }
            }

            div { class: "flex items-center gap-3 shrink-0 ms-auto",
                if let Some(right) = right_slot {
                    {right}
                } else {
                    div { class: "flex items-center gap-2 text-2xs",
                        span { class: "inline-flex h-2 w-2 rounded-full bg-emerald-500" }
                        span { "Operational" }
                        span { class: "text-muted-foreground/60", "•" }
                        span { "v1.0.0" }
                    }
                }
            }

            if let Some(c) = children {
                {c}
            }
        }
    }
}
