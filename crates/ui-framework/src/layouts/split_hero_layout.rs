use dioxus::prelude::*;

#[component]
pub fn SplitHeroLayout(
    #[props(default = "")] class: &'static str,
    hero_content: Option<Element>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "min-h-screen w-full grid grid-cols-1 lg:grid-cols-2 {class}",
            // Form / Interactive Area
            div { class: "flex flex-col justify-center items-center p-6 sm:p-12 lg:p-16 bg-background",
                div { class: "w-full max-w-md space-y-6",
                    {children}
                }
            }

            // Hero Brand Showcase
            div { class: "hidden lg:flex flex-col justify-between p-12 lg:p-16 bg-primary text-primary-foreground select-none",
                div { class: "flex items-center gap-2",
                    div { class: "h-7 w-7 rounded-md bg-background/20 backdrop-blur-xs flex items-center justify-center font-bold text-sm", "E" }
                    span { class: "font-semibold text-sm tracking-tight", "Enterprise Platform" }
                }

                div { class: "my-auto py-8",
                    if let Some(hero) = hero_content {
                        {hero}
                    } else {
                        div { class: "space-y-3",
                            h2 { class: "text-2xl font-bold tracking-tight", "Modern operations, simplified." }
                            p { class: "text-sm opacity-90 max-w-md", "Resilient cyber-physical and enterprise dashboards engineered for high density, mission-critical telemetry, and real-time control." }
                        }
                    }
                }

                div { class: "text-xs opacity-70",
                    "© ATT IT / Philippe Simon Pflug. All rights reserved."
                }
            }
        }
    }
}
