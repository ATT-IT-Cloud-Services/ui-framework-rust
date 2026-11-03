use dioxus::prelude::*;
use crate::settings::provider::use_layout_settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitDisplayMode {
    #[default]
    Docked,
    OverlapThird,
    OverlapHalf,
    Maximized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitRatio {
    Half,
    #[default]
    TwoThirds,
    ThreeQuarters,
}

#[component]
pub fn SplitWithInfoPanel(
    info_panel: Element,
    #[props(default = "Information")] panel_title: &'static str,
    #[props(default)] default_mode: SplitDisplayMode,
    #[props(default)] default_ratio: SplitRatio,
    #[props(default = "")] class: &'static str,
    children: Element,
) -> Element {
    let settings = use_layout_settings();
    let mut mode = use_signal(|| default_mode);
    let mut ratio = use_signal(|| default_ratio);
    let mut collapsed = use_signal(|| false);

    let is_overlap = mode() == SplitDisplayMode::OverlapThird || mode() == SplitDisplayMode::OverlapHalf;
    let is_maximized = mode() == SplitDisplayMode::Maximized;

    let blur_class = if settings.read().overlays.blur_left_on_menu_open {
        match settings.read().overlays.backdrop_blur {
            crate::settings::types::BackdropBlur::None => "backdrop-blur-none",
            crate::settings::types::BackdropBlur::Sm => "backdrop-blur-xs",
            crate::settings::types::BackdropBlur::Md => "backdrop-blur-sm",
            crate::settings::types::BackdropBlur::Lg => "backdrop-blur-md",
        }
    } else {
        "backdrop-blur-none"
    };

    rsx! {
        div { class: "relative flex flex-col h-full min-h-[600px] space-y-2 {class}",
            // Top Toolbar: Mode Switcher & Controls
            div { class: "flex items-center justify-between px-1 flex-wrap gap-2 select-none",
                div { class: "flex items-center gap-2",
                    // Mode Switcher Buttons
                    div { class: "inline-flex items-center rounded-md border border-input bg-muted/40 p-0.5 text-2xs",
                        button {
                            r#type: "button",
                            class: if mode() == SplitDisplayMode::Docked && !collapsed() {
                                "px-2 py-1 rounded bg-background text-foreground font-semibold shadow-2xs cursor-pointer"
                            } else {
                                "px-2 py-1 rounded text-muted-foreground hover:text-foreground cursor-pointer"
                            },
                            onclick: move |_| {
                                mode.set(SplitDisplayMode::Docked);
                                collapsed.set(false);
                            },
                            "Docked"
                        }
                        button {
                            r#type: "button",
                            class: if mode() == SplitDisplayMode::OverlapThird && !collapsed() {
                                "px-2 py-1 rounded bg-background text-foreground font-semibold shadow-2xs cursor-pointer"
                            } else {
                                "px-2 py-1 rounded text-muted-foreground hover:text-foreground cursor-pointer"
                            },
                            onclick: move |_| {
                                mode.set(SplitDisplayMode::OverlapThird);
                                collapsed.set(false);
                            },
                            "Overlap 1/3"
                        }
                        button {
                            r#type: "button",
                            class: if mode() == SplitDisplayMode::OverlapHalf && !collapsed() {
                                "px-2 py-1 rounded bg-background text-foreground font-semibold shadow-2xs cursor-pointer"
                            } else {
                                "px-2 py-1 rounded text-muted-foreground hover:text-foreground cursor-pointer"
                            },
                            onclick: move |_| {
                                mode.set(SplitDisplayMode::OverlapHalf);
                                collapsed.set(false);
                            },
                            "Overlap 1/2"
                        }
                        button {
                            r#type: "button",
                            class: if mode() == SplitDisplayMode::Maximized && !collapsed() {
                                "px-2 py-1 rounded bg-background text-foreground font-semibold shadow-2xs cursor-pointer"
                            } else {
                                "px-2 py-1 rounded text-muted-foreground hover:text-foreground cursor-pointer"
                            },
                            onclick: move |_| {
                                mode.set(SplitDisplayMode::Maximized);
                                collapsed.set(false);
                            },
                            "Maximized"
                        }
                    }

                    // Ratio (only in docked mode)
                    if mode() == SplitDisplayMode::Docked && !collapsed() {
                        div { class: "inline-flex items-center rounded-md border border-input bg-muted/40 p-0.5 text-2xs",
                            button {
                                r#type: "button",
                                class: if ratio() == SplitRatio::Half { "px-1.5 py-0.5 rounded bg-background font-semibold shadow-2xs" } else { "px-1.5 py-0.5 rounded text-muted-foreground" },
                                onclick: move |_| ratio.set(SplitRatio::Half),
                                "1/2"
                            }
                            button {
                                r#type: "button",
                                class: if ratio() == SplitRatio::TwoThirds { "px-1.5 py-0.5 rounded bg-background font-semibold shadow-2xs" } else { "px-1.5 py-0.5 rounded text-muted-foreground" },
                                onclick: move |_| ratio.set(SplitRatio::TwoThirds),
                                "2/3"
                            }
                            button {
                                r#type: "button",
                                class: if ratio() == SplitRatio::ThreeQuarters { "px-1.5 py-0.5 rounded bg-background font-semibold shadow-2xs" } else { "px-1.5 py-0.5 rounded text-muted-foreground" },
                                onclick: move |_| ratio.set(SplitRatio::ThreeQuarters),
                                "3/4"
                            }
                        }
                    }
                }

                // Collapse Toggle
                button {
                    r#type: "button",
                    class: "h-7 px-2.5 rounded-md text-xs font-medium text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer transition-colors",
                    onclick: move |_| collapsed.set(!collapsed()),
                    if collapsed() { "Show Panel ▶" } else { "Hide Panel ✕" }
                }
            }

            // Main Content Card
            div { class: "relative flex-1 rounded-xl border border-border bg-card overflow-hidden flex flex-col",
                // Case 1: Collapsed
                if collapsed() {
                    div { class: "h-full w-full overflow-y-auto p-4", {children} }
                } else if is_maximized {
                    // Case 2: Maximized Panel
                    div { class: "h-full w-full overflow-y-auto p-5 bg-background animate-in fade-in-50",
                        div { class: "flex justify-end pb-2 mb-2 border-b border-border",
                            button {
                                r#type: "button",
                                class: "h-7 px-2 text-xs rounded border border-border bg-muted/40 hover:bg-muted cursor-pointer",
                                onclick: move |_| mode.set(SplitDisplayMode::Docked),
                                "Restore Layout"
                            }
                        }
                        {info_panel}
                    }
                } else if is_overlap {
                    // Case 3: Overlap Mode (Table stays 100% full width, panel floats on right anchored to viewport height!)
                    div { class: "relative h-full w-full overflow-hidden flex flex-col",
                        // 100% full width table behind
                        div { class: "h-full w-full overflow-y-auto p-4", {children} }

                        // Viewport Anchored Overlay Container (Between header and footer!)
                        div {
                            class: "fixed inset-x-0 z-[200] pointer-events-none transition-all duration-200",
                            style: "top: calc(var(--header-height, 48px) + var(--nav-height, 0px)); bottom: var(--footer-height, 0px);",

                            // Left Backdrop Blur
                            div {
                                class: if mode() == SplitDisplayMode::OverlapThird {
                                    "absolute inset-0 bg-black/20 pointer-events-auto cursor-pointer transition-all duration-200 pe-[34%] {blur_class}"
                                } else {
                                    "absolute inset-0 bg-black/20 pointer-events-auto cursor-pointer transition-all duration-200 pe-[50%] {blur_class}"
                                },
                                onclick: move |_| collapsed.set(true),
                            }

                            // Floating Full-Height Overlap Panel
                            div {
                                class: if mode() == SplitDisplayMode::OverlapThird {
                                    "absolute inset-y-0 end-0 pointer-events-auto flex flex-col border-s border-border bg-background/95 backdrop-blur-md shadow-2xl p-5 overflow-hidden w-full sm:w-1/3 min-w-[390px] animate-in slide-in-from-right-4"
                                } else {
                                    "absolute inset-y-0 end-0 pointer-events-auto flex flex-col border-s border-border bg-background/95 backdrop-blur-md shadow-2xl p-5 overflow-hidden w-full sm:w-1/2 animate-in slide-in-from-right-4"
                                },
                                div { class: "flex items-center justify-between pb-3 mb-2 border-b border-border shrink-0",
                                    span { class: "text-2xs font-bold text-primary uppercase tracking-wider", "Viewport Overlay Panel" }
                                    button {
                                        r#type: "button",
                                        class: "h-6 w-6 rounded flex items-center justify-center text-muted-foreground hover:text-foreground cursor-pointer",
                                        onclick: move |_| collapsed.set(true),
                                        "✕"
                                    }
                                }
                                div { class: "flex-1 overflow-y-auto pe-1",
                                    {info_panel}
                                }
                            }
                        }
                    }
                } else {
                    // Case 4: Docked Side-by-Side Mode
                    div {
                        class: "h-full w-full grid grid-cols-12 overflow-hidden",
                        div {
                            class: match ratio() {
                                SplitRatio::Half => "col-span-12 md:col-span-6 h-full overflow-y-auto p-4 border-e border-border/80",
                                SplitRatio::TwoThirds => "col-span-12 md:col-span-8 h-full overflow-y-auto p-4 border-e border-border/80",
                                SplitRatio::ThreeQuarters => "col-span-12 md:col-span-9 h-full overflow-y-auto p-4 border-e border-border/80",
                            },
                            {children}
                        }
                        div {
                            class: match ratio() {
                                SplitRatio::Half => "col-span-12 md:col-span-6 h-full overflow-y-auto p-4 bg-background/50",
                                SplitRatio::TwoThirds => "col-span-12 md:col-span-4 h-full overflow-y-auto p-4 bg-background/50",
                                SplitRatio::ThreeQuarters => "col-span-12 md:col-span-3 h-full overflow-y-auto p-4 bg-background/50",
                            },
                            {info_panel}
                        }
                    }
                }
            }
        }
    }
}
