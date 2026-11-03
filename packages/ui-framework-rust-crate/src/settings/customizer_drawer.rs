use dioxus::prelude::*;
use super::types::{BackdropBlur, Density, FontFamilyPreset, LayoutSettings, NavigationMode, NavigationPosition, ThemeMode};
use super::provider::use_layout_settings;

#[component]
pub fn CustomizerDrawer() -> Element {
    let mut settings = use_layout_settings();
    let mut is_open = use_signal(|| false);
    let mut test_toast_message = use_signal(|| None::<String>);

    rsx! {
        // Floating Settings FAB Trigger Button
        button {
            r#type: "button",
            class: "fixed end-4 bottom-5 z-[500] flex h-10 w-10 items-center justify-center rounded-full bg-primary text-primary-foreground shadow-lg hover:bg-primary/90 transition-transform hover:scale-105 active:scale-95 cursor-pointer",
            onclick: move |_| is_open.set(!is_open()),
            title: "Layout & Theme Customizer",
            "⚙"
        }

        // Test Toast popup
        if let Some(msg) = test_toast_message() {
            div {
                class: "fixed bottom-16 end-4 z-[700] bg-foreground text-background text-xs py-2 px-3.5 rounded-md shadow-xl animate-in fade-in-50",
                "{msg}"
            }
        }

        // Drawer
        if is_open() {
            // Backdrop
            div {
                class: "fixed inset-0 z-[400] bg-black/40 backdrop-blur-xs transition-opacity",
                onclick: move |_| is_open.set(false),
            }

            // Drawer content
            div {
                class: "fixed inset-y-0 end-0 z-[400] w-full sm:max-w-md bg-background border-s border-border shadow-2xl flex flex-col h-full animate-in slide-in-from-right duration-200 select-none text-xs",
                
                // Header
                div {
                    class: "flex items-center justify-between p-4 border-b border-border bg-muted/20",
                    div {
                        h3 { class: "font-semibold text-sm text-foreground", "Layout & Appearance Customizer" }
                        p { class: "text-2xs text-muted-foreground", "Configure navigation modes, typography, density, and tokens." }
                    }
                    button {
                        r#type: "button",
                        class: "h-7 w-7 rounded-md flex items-center justify-center text-muted-foreground hover:text-foreground hover:bg-muted cursor-pointer",
                        onclick: move |_| is_open.set(false),
                        "✕"
                    }
                }

                // Scrollable Controls
                div {
                    class: "flex-1 overflow-y-auto p-4 space-y-6",

                    // 1. Navigation Mode
                    div {
                        class: "space-y-2",
                        span { class: "text-2xs font-semibold uppercase tracking-wider text-muted-foreground block", "Navigation Mode" }
                        div {
                            class: "grid grid-cols-2 gap-2",
                            for mode in [
                                NavigationMode::SidebarFull,
                                NavigationMode::SidebarRail,
                                NavigationMode::SidebarHybrid,
                                NavigationMode::SidebarDual,
                                NavigationMode::SidebarInset,
                                NavigationMode::Topnav,
                                NavigationMode::BottomNav,
                            ] {
                                {
                                    let is_active = settings.read().navigation_mode == mode;
                                    rsx! {
                                        button {
                                            r#type: "button",
                                            class: if is_active {
                                                "p-2.5 rounded-lg border border-primary bg-primary/10 text-primary font-semibold text-start cursor-pointer"
                                            } else {
                                                "p-2.5 rounded-lg border border-border hover:bg-muted text-foreground text-start cursor-pointer"
                                            },
                                            onclick: move |_| settings.write().navigation_mode = mode,
                                            "{mode.as_str()}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 2. Position
                    div {
                        class: "space-y-2",
                        span { class: "text-2xs font-semibold uppercase tracking-wider text-muted-foreground block", "Navigation Position" }
                        div {
                            class: "grid grid-cols-4 gap-1.5",
                            for pos in [NavigationPosition::Start, NavigationPosition::End, NavigationPosition::Top, NavigationPosition::Bottom] {
                                {
                                    let is_active = settings.read().navigation_position == pos;
                                    rsx! {
                                        button {
                                            r#type: "button",
                                            class: if is_active {
                                                "py-1 px-2 rounded border border-primary bg-primary text-primary-foreground font-semibold text-center cursor-pointer"
                                            } else {
                                                "py-1 px-2 rounded border border-border hover:bg-muted text-foreground text-center cursor-pointer"
                                            },
                                            onclick: move |_| settings.write().navigation_position = pos,
                                            "{pos.as_str()}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 3. Theme & Density
                    div {
                        class: "space-y-3 pt-2 border-t border-border",
                        div {
                            class: "flex items-center justify-between",
                            span { class: "font-medium text-foreground", "Theme" }
                            div {
                                class: "flex items-center gap-1 bg-muted p-0.5 rounded",
                                button {
                                    r#type: "button",
                                    class: if settings.read().theme == ThemeMode::Light { "bg-background shadow-xs px-2 py-0.5 rounded font-semibold text-primary" } else { "px-2 py-0.5 text-muted-foreground" },
                                    onclick: move |_| settings.write().theme = ThemeMode::Light,
                                    "Light"
                                }
                                button {
                                    r#type: "button",
                                    class: if settings.read().theme == ThemeMode::Dark { "bg-background shadow-xs px-2 py-0.5 rounded font-semibold text-primary" } else { "px-2 py-0.5 text-muted-foreground" },
                                    onclick: move |_| settings.write().theme = ThemeMode::Dark,
                                    "Dark"
                                }
                                button {
                                    r#type: "button",
                                    class: if settings.read().theme == ThemeMode::System { "bg-background shadow-xs px-2 py-0.5 rounded font-semibold text-primary" } else { "px-2 py-0.5 text-muted-foreground" },
                                    onclick: move |_| settings.write().theme = ThemeMode::System,
                                    "System"
                                }
                            }
                        }

                        div {
                            class: "flex items-center justify-between",
                            span { class: "font-medium text-foreground", "Density" }
                            div {
                                class: "flex items-center gap-1 bg-muted p-0.5 rounded",
                                button {
                                    r#type: "button",
                                    class: if settings.read().density == Density::Comfortable { "bg-background shadow-xs px-2 py-0.5 rounded font-semibold text-primary" } else { "px-2 py-0.5 text-muted-foreground" },
                                    onclick: move |_| settings.write().density = Density::Comfortable,
                                    "Comfortable"
                                }
                                button {
                                    r#type: "button",
                                    class: if settings.read().density == Density::Compact { "bg-background shadow-xs px-2 py-0.5 rounded font-semibold text-primary" } else { "px-2 py-0.5 text-muted-foreground" },
                                    onclick: move |_| settings.write().density = Density::Compact,
                                    "Compact"
                                }
                            }
                        }
                    }

                    // 4. Loading & showAfter Delay
                    div {
                        class: "space-y-3 pt-2 border-t border-border",
                        div {
                            class: "flex items-center justify-between",
                            span { class: "text-2xs font-semibold uppercase tracking-wider text-muted-foreground", "Loading & showAfter Delay" }
                            button {
                                r#type: "button",
                                class: "px-2 py-0.5 rounded bg-primary/10 text-primary text-[10px] font-semibold cursor-pointer",
                                onclick: move |_| {
                                    test_toast_message.set(Some("showAfter delay simulated successfully!".to_string()));
                                },
                                "Test showAfter"
                            }
                        }

                        div {
                            class: "grid grid-cols-2 gap-3",
                            div {
                                class: "space-y-1",
                                span { class: "text-2xs text-muted-foreground block", "showAfter (Threshold)" }
                                select {
                                    class: "w-full h-8 px-2 rounded border border-input bg-background text-xs cursor-pointer",
                                    value: "{settings.read().loading.show_after}",
                                    onchange: move |e| {
                                        if let Ok(val) = e.value().parse::<u64>() {
                                            settings.write().loading.show_after = val;
                                        }
                                    },
                                    option { value: "0", "0ms (Instant - may flash)" }
                                    option { value: "150", "150ms (Snappy)" }
                                    option { value: "200", "200ms (Recommended)" }
                                    option { value: "350", "350ms (Conservative)" }
                                }
                            }
                            div {
                                class: "space-y-1",
                                span { class: "text-2xs text-muted-foreground block", "Min Duration (Visible)" }
                                select {
                                    class: "w-full h-8 px-2 rounded border border-input bg-background text-xs cursor-pointer",
                                    value: "{settings.read().loading.min_duration}",
                                    onchange: move |e| {
                                        if let Ok(val) = e.value().parse::<u64>() {
                                            settings.write().loading.min_duration = val;
                                        }
                                    },
                                    option { value: "0", "0ms (None)" }
                                    option { value: "200", "200ms" }
                                    option { value: "350", "350ms (Recommended)" }
                                    option { value: "500", "500ms" }
                                }
                            }
                        }
                    }

                    // 5. Left Backdrop Blur on Menu Open
                    div {
                        class: "space-y-2 pt-2 border-t border-border",
                        span { class: "text-2xs font-semibold uppercase tracking-wider text-muted-foreground block", "Backdrop & Left Blur on Menu Open" }
                        div {
                            class: "grid grid-cols-4 gap-1.5",
                            for (blur, label) in [(BackdropBlur::None, "0px"), (BackdropBlur::Sm, "4px"), (BackdropBlur::Md, "8px"), (BackdropBlur::Lg, "16px")] {
                                {
                                    let is_active = settings.read().overlays.backdrop_blur == blur;
                                    rsx! {
                                        button {
                                            r#type: "button",
                                            class: if is_active {
                                                "py-1 px-1 rounded border border-primary bg-primary text-primary-foreground font-semibold text-center cursor-pointer"
                                            } else {
                                                "py-1 px-1 rounded border border-border hover:bg-muted text-foreground text-center cursor-pointer"
                                            },
                                            onclick: move |_| settings.write().overlays.backdrop_blur = blur,
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // 6. Typography
                    div {
                        class: "space-y-2 pt-2 border-t border-border",
                        span { class: "text-2xs font-semibold uppercase tracking-wider text-muted-foreground block", "Typography (Body Sans Font)" }
                        div {
                            class: "grid grid-cols-3 gap-1.5",
                            for (font, label) in [
                                (FontFamilyPreset::Inter, "Inter"),
                                (FontFamilyPreset::Geist, "Geist"),
                                (FontFamilyPreset::Roboto, "Roboto"),
                                (FontFamilyPreset::Jakarta, "Jakarta"),
                                (FontFamilyPreset::IbmPlex, "IBM Plex"),
                                (FontFamilyPreset::System, "System"),
                            ] {
                                {
                                    let is_active = settings.read().typography.font_sans == font;
                                    rsx! {
                                        button {
                                            r#type: "button",
                                            class: if is_active {
                                                "py-1 px-1.5 rounded border border-primary bg-primary text-primary-foreground font-semibold text-center cursor-pointer"
                                            } else {
                                                "py-1 px-1.5 rounded border border-border hover:bg-muted text-foreground text-center cursor-pointer"
                                            },
                                            onclick: move |_| settings.write().typography.font_sans = font.clone(),
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Footer
                div {
                    class: "p-4 border-t border-border bg-muted/20 flex gap-2",
                    button {
                        r#type: "button",
                        class: "flex-1 py-1.5 px-3 rounded-md border border-input bg-background hover:bg-muted text-foreground font-semibold text-xs cursor-pointer",
                        onclick: move |_| settings.set(LayoutSettings::default()),
                        "Reset to Default"
                    }
                }
            }
        }
    }
}
