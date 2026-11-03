use dioxus::prelude::*;
use ui_framework::ui::badge::{Badge, BadgeVariant};

struct DemoCardInfo {
    id: &'static str,
    title: &'static str,
    category: &'static str,
    description: &'static str,
    href: &'static str,
    tag: Option<&'static str>,
}

const DEMO_CARDS: &[DemoCardInfo] = &[
    // Flagship
    DemoCardInfo {
        id: "flagship",
        title: "Enterprise Operations Showcase",
        category: "Flagship Showcase",
        description: "Full mission control: mock login, dual-rail nav, SplitWithInfoPanel, role switching (Admin/Editor/Viewer), and notification center.",
        href: "/demo/combined/enterprise-showcase",
        tag: Some("FLAGSHIP"),
    },
    // Navigation
    DemoCardInfo {
        id: "nav-sidebar-full",
        title: "Full Sidebar Navigation",
        category: "Navigation Modes",
        description: "Classic 260px wide enterprise navigation with section headings, animated accordions, and badge indicators.",
        href: "/demo/navigation/sidebar-full",
        tag: None,
    },
    DemoCardInfo {
        id: "nav-sidebar-rail",
        title: "Icon Rail Navigation",
        category: "Navigation Modes",
        description: "Ultra-compact 64px icon rail maximizing horizontal space with floating popovers and Pin/Dock button.",
        href: "/demo/navigation/sidebar-rail",
        tag: None,
    },
    DemoCardInfo {
        id: "nav-sidebar-hybrid",
        title: "Hybrid Sidebar Navigation",
        category: "Navigation Modes",
        description: "Switchable between rail and full with toggle button and optional hover expand.",
        href: "/demo/navigation/sidebar-hybrid",
        tag: Some("POPULAR"),
    },
    DemoCardInfo {
        id: "nav-sidebar-dual",
        title: "Dual Rail Navigation",
        category: "Navigation Modes",
        description: "56px primary module rail + dedicated 200px secondary panel displaying module submenus.",
        href: "/demo/navigation/sidebar-dual",
        tag: None,
    },
    DemoCardInfo {
        id: "nav-sidebar-inset",
        title: "Inset Floating Sidebar",
        category: "Navigation Modes",
        description: "Floating inset navigation with rounded corners and elevated content card container.",
        href: "/demo/navigation/sidebar-inset",
        tag: None,
    },
    DemoCardInfo {
        id: "nav-topnav",
        title: "Top Navigation Bar",
        category: "Navigation Modes",
        description: "Horizontal top bar with mega-menu dropdowns and responsive overflow into 'More'.",
        href: "/demo/navigation/topnav",
        tag: None,
    },
    DemoCardInfo {
        id: "nav-bottom-nav",
        title: "Bottom Tab Navigation",
        category: "Navigation Modes",
        description: "Touch-friendly bottom tab bar with full text without truncation + full-height Sheet drawer.",
        href: "/demo/navigation/bottom-nav",
        tag: None,
    },
    // Layouts
    DemoCardInfo {
        id: "layout-single-pane",
        title: "SinglePane Template",
        category: "Content Layouts",
        description: "Standard uncluttered full-width layout for dashboards and overview reports.",
        href: "/demo/layouts/single-pane",
        tag: None,
    },
    DemoCardInfo {
        id: "layout-master-detail",
        title: "MasterDetail Template",
        category: "Content Layouts",
        description: "Searchable master list (300-400px) + detail pane with mobile list->detail back drilldown.",
        href: "/demo/layouts/master-detail",
        tag: Some("RECOMMENDED"),
    },
    DemoCardInfo {
        id: "layout-three-pane",
        title: "ThreePane Template",
        category: "Content Layouts",
        description: "Three resizable columns: [App Navigation] | [Sub-list] | [Detail Content].",
        href: "/demo/layouts/three-pane",
        tag: None,
    },
    DemoCardInfo {
        id: "layout-split-info-panel",
        title: "SplitWithInfoPanel Template",
        category: "Content Layouts",
        description: "Main workspace + right panel with Docked, Overlap 1/3, Overlap 1/2, and display-height binding with left blur.",
        href: "/demo/layouts/split-info-panel",
        tag: Some("CORE PATTERN"),
    },
    DemoCardInfo {
        id: "layout-full-split",
        title: "FullSplit Template",
        category: "Content Layouts",
        description: "4-column layout: Navigation | Submenu | Content | Info Panel, each column resizable.",
        href: "/demo/layouts/full-split",
        tag: None,
    },
    DemoCardInfo {
        id: "layout-object-page",
        title: "Enterprise ObjectPage Template",
        category: "Content Layouts",
        description: "Entity overview header, status, key attributes bar, sticky anchor tabs, and sticky action bar.",
        href: "/demo/layouts/object-page",
        tag: Some("OBJECT PAGE"),
    },
    DemoCardInfo {
        id: "layout-settings-layout",
        title: "SettingsLayout Template",
        category: "Content Layouts",
        description: "Equal-height cards with static sub-navigation and typography customization tab.",
        href: "/demo/layouts/settings-layout",
        tag: None,
    },
    DemoCardInfo {
        id: "layout-focus-layout",
        title: "FocusLayout Template",
        category: "Content Layouts",
        description: "Distraction-free full-screen modal layout hiding navigation with top close button.",
        href: "/demo/layouts/focus-layout",
        tag: None,
    },
    // Realtime
    DemoCardInfo {
        id: "realtime-operator-console",
        title: "Operator SCADA Telemetry Console",
        category: "Cyber-Physical & Realtime",
        description: "Simulated 50Hz live telemetry stream, alarms, high-contrast shift modes, and press-and-hold breakers.",
        href: "/demo/realtime/operator-console",
        tag: Some("LIVE STREAM"),
    },
];

#[component]
pub fn GalleryView() -> Element {
    let categories = ["Flagship Showcase", "Navigation Modes", "Content Layouts", "Cyber-Physical & Realtime"];

    rsx! {
        div { class: "min-h-screen bg-background text-foreground flex flex-col select-none",
            // Header
            header { class: "sticky top-0 z-[300] flex h-14 items-center justify-between border-b border-border bg-background/95 px-4 sm:px-8 backdrop-blur-md",
                div { class: "flex items-center gap-3",
                    div { class: "flex h-7 w-7 items-center justify-center rounded-md bg-primary text-primary-foreground font-bold text-xs shadow-2xs", "R" }
                    div { class: "flex items-center gap-2",
                        span { class: "font-bold text-sm tracking-tight text-foreground", "Rust/UI Enterprise Framework" }
                        Badge { variant: BadgeVariant::Outline, "Dioxus 0.7 • Rust" }
                    }
                }
            }

            // Hero Banner
            section { class: "border-b border-border bg-muted/20 py-12 px-4 sm:px-8",
                div { class: "max-w-5xl mx-auto space-y-4",
                    div { class: "inline-flex items-center gap-2 px-2.5 py-1 rounded-full bg-primary/10 text-primary text-2xs font-semibold",
                        span { "✦" }
                        span { "100% Rust & Dioxus 0.7 Architecture" }
                    }

                    h1 { class: "text-3xl sm:text-4xl font-extrabold tracking-tight text-foreground",
                        "Enterprise UI Framework in Rust"
                    }

                    p { class: "text-sm text-muted-foreground leading-relaxed max-w-3xl",
                        "Host-agnostic, zero-backend, zero-auth UI foundation engineered for cyber-physical operations, SCADA control rooms, and enterprise platforms. Calm, dense-but-airy, neutral palette with dynamic brand tokens, full dark mode, and complete RTL internationalization."
                    }

                    div { class: "flex items-center gap-3 pt-2",
                        a {
                            href: "/demo/combined/enterprise-showcase",
                            class: "inline-flex items-center gap-2 px-4 py-2 rounded-md bg-primary text-primary-foreground text-xs font-semibold shadow-xs hover:bg-primary/90 cursor-pointer",
                            "Launch Flagship Showcase ▶"
                        }
                    }
                }
            }

            // Gallery Categories
            main { class: "flex-1 max-w-7xl mx-auto w-full p-4 sm:p-8 space-y-12",
                for cat in categories {
                    {
                        let items = DEMO_CARDS.iter().filter(|d| d.category == cat).collect::<Vec<_>>();
                        rsx! {
                            div { key: "{cat}", class: "space-y-4",
                                div { class: "flex items-center gap-2 pb-2 border-b border-border/80",
                                    h2 { class: "text-sm font-bold uppercase tracking-wider text-foreground", "{cat}" }
                                    span { class: "text-2xs text-muted-foreground", "({items.len()} demos)" }
                                }

                                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                                    for item in items {
                                        div {
                                            key: "{item.id}",
                                            class: "flex flex-col justify-between rounded-xl border border-border bg-card p-4 hover:border-primary/50 transition-all shadow-2xs",
                                            div { class: "space-y-1.5",
                                                div { class: "flex items-center justify-between gap-2",
                                                    h3 { class: "text-xs font-semibold text-foreground", "{item.title}" }
                                                    if let Some(t) = item.tag {
                                                        Badge { variant: BadgeVariant::Secondary, "{t}" }
                                                    }
                                                }
                                                p { class: "text-2xs text-muted-foreground leading-relaxed", "{item.description}" }
                                            }
                                            div { class: "pt-4",
                                                a {
                                                    href: "{item.href}",
                                                    class: "text-xs font-semibold text-primary hover:underline",
                                                    "Open Demo →"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Footer
            footer { class: "border-t border-border bg-muted/20 py-6 px-4 text-center text-xs text-muted-foreground",
                "Rust Enterprise UI Framework & Demo Gallery • MIT License © ATT IT / Philippe Simon Pflug."
            }
        }
    }
}
