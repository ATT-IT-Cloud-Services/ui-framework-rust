use dioxus::prelude::*;
use crate::navigation::types::NavigationItem;
use crate::navigation::use_navigation_tree::use_navigation_tree;
use crate::navigation::sidebar_full::SidebarFull;
use crate::navigation::sidebar_rail::SidebarRail;
use crate::navigation::sidebar_hybrid::SidebarHybrid;
use crate::navigation::sidebar_dual::SidebarDual;
use crate::navigation::sidebar_inset::SidebarInset;
use crate::navigation::topnav::Topnav;
use crate::navigation::bottom_nav::BottomNav;
use crate::settings::provider::{use_is_dark, use_layout_settings};
use crate::settings::customizer_drawer::CustomizerDrawer;
use crate::config::types::{ContentWidth, NavigationMode, NavigationPosition};
use super::header::Header;
use super::app_footer::AppFooter;
use super::command_palette::CommandPalette;

#[component]
pub fn AppShell(
    navigation_items: Vec<NavigationItem>,
    #[props(default = false)] hide_header: bool,
    #[props(default = false)] hide_navigation: bool,
    #[props(default = false)] hide_customizer: bool,
    #[props(default = false)] show_default_footer: bool,
    #[props(default = "")] class: &'static str,
    logo_slot: Option<Element>,
    header_slot: Option<Element>,
    user_menu_slot: Option<Element>,
    sidebar_header_slot: Option<Element>,
    sidebar_footer_slot: Option<Element>,
    footer_slot: Option<Element>,
    /// Forwarded to the header: every visible label can be localised by the host.
    #[props(default = "Search".to_string())] search_label: String,
    #[props(default = "Ctrl+K".to_string())] search_shortcut: String,
    #[props(default = "Toggle theme".to_string())] theme_label: String,
    #[props(default = "Change language".to_string())] language_label: String,
    #[props(default = false)] hide_search: bool,
    #[props(default = false)] hide_language: bool,
    #[props(default = false)] hide_theme: bool,
    /// Accessible name of the mobile menu trigger.
    #[props(default = "Open navigation".to_string())] menu_label: String,
    /// Optional mobile navigation trigger, forwarded to the header.
    #[props(default = None)] on_menu: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    let settings = use_layout_settings();
    let mut command_palette_open = use_signal(|| false);

    let items = use_navigation_tree(navigation_items.clone());

    let mode = settings.read().navigation_mode;
    let pos = settings.read().navigation_position;
    let width = settings.read().content_width;
    let sidebar_sticky = settings.read().sidebar_sticky;

    let is_nav_at_end = pos == NavigationPosition::End;
    let is_nav_top = mode == NavigationMode::Topnav || pos == NavigationPosition::Top;
    let is_nav_bottom = mode == NavigationMode::BottomNav || pos == NavigationPosition::Bottom;

    let is_dark = use_is_dark();

    // Content width constraint classes
    let content_width_class = match width {
        ContentWidth::Boxed => "max-w-7xl mx-auto w-full p-4 sm:p-6",
        ContentWidth::Fluid => "w-full p-4 sm:p-6",
        ContentWidth::Full => "w-full h-full p-0 m-0",
    };

    let root_class = if is_dark {
        format!("dark relative flex min-h-screen w-full flex-col bg-background text-foreground transition-colors duration-150 {class}")
    } else {
        format!("relative flex min-h-screen w-full flex-col bg-background text-foreground transition-colors duration-150 {class}")
    };

    rsx! {
        div {
            class: "{root_class}",

            // Top Header
            if !hide_header {
                Header {
                    logo_slot: logo_slot,
                    actions_slot: header_slot,
                    user_menu_slot: user_menu_slot,
                    on_search: move |_| command_palette_open.set(true),
                    search_label: search_label.clone(),
                    search_shortcut: search_shortcut.clone(),
                    theme_label: theme_label.clone(),
                    language_label: language_label.clone(),
                    hide_search: hide_search,
                    hide_language: hide_language,
                    hide_theme: hide_theme,
                    on_menu: on_menu,
                    menu_label: menu_label.clone(),
                }
            }

            // Topnav
            if is_nav_top && !hide_navigation {
                Topnav { items: items.read().clone() }
            }

            // Main layout area (Sidebar + Content Body)
            div {
                class: if is_nav_at_end {
                    "flex flex-1 w-full items-start flex-row-reverse"
                } else {
                    "flex flex-1 w-full items-start"
                },

                // Desktop Sidebar
                if !is_nav_top && !is_nav_bottom && !hide_navigation {
                    div {
                        class: if sidebar_sticky {
                            // The sidebar scrolls inside itself; its header stays visible.
                            "hidden lg:flex shrink-0 sticky top-[var(--header-height,48px)] h-[calc(100vh-var(--header-height,48px)-var(--footer-height,0px))] overflow-hidden z-[100]"
                        } else {
                            "hidden lg:flex shrink-0 min-h-full h-auto"
                        },
                        match mode {
                            NavigationMode::SidebarRail => rsx! {
                                SidebarRail {
                                    items: items.read().clone(),
                                    header_slot: sidebar_header_slot,
                                    footer_slot: sidebar_footer_slot,
                                }
                            },
                            NavigationMode::SidebarHybrid => rsx! {
                                SidebarHybrid {
                                    items: items.read().clone(),
                                    header_slot: sidebar_header_slot,
                                    footer_slot: sidebar_footer_slot,
                                }
                            },
                            NavigationMode::SidebarDual => rsx! {
                                SidebarDual {
                                    items: items.read().clone(),
                                    header_slot: sidebar_header_slot,
                                    footer_slot: sidebar_footer_slot,
                                }
                            },
                            NavigationMode::SidebarInset => rsx! {
                                SidebarInset {
                                    items: items.read().clone(),
                                    header_slot: sidebar_header_slot,
                                    footer_slot: sidebar_footer_slot,
                                }
                            },
                            _ => rsx! {
                                SidebarFull {
                                    items: items.read().clone(),
                                    header_slot: sidebar_header_slot,
                                    footer_slot: sidebar_footer_slot,
                                }
                            },
                        }
                    }
                }

                // Content Viewport
                main {
                    class: if mode == NavigationMode::SidebarInset {
                        "flex-1 min-w-0 flex flex-col focus:outline-none p-2.5"
                    } else {
                        "flex-1 min-w-0 flex flex-col focus:outline-none"
                    },
                    if mode == NavigationMode::SidebarInset {
                        div { class: "flex-1 rounded-xl border border-border bg-card text-card-foreground shadow-2xs overflow-hidden flex flex-col",
                            div { class: "flex-1 flex flex-col {content_width_class}", {children} }
                        }
                    } else {
                        div { class: "flex-1 flex flex-col {content_width_class}", {children} }
                    }
                }
            }

            // Bottom Nav
            if is_nav_bottom && !hide_navigation {
                div { class: "block pb-14",
                    BottomNav { items: items.read().clone() }
                }
            }

            // Footer
            if let Some(footer) = footer_slot {
                {footer}
            } else if show_default_footer {
                AppFooter {}
            }

            // Customizer Drawer
            if !hide_customizer {
                CustomizerDrawer {}
            }

            // Command Palette
            CommandPalette {
                open: command_palette_open(),
                items: items.read().clone(),
                onclose: move |_| command_palette_open.set(false),
            }
        }
    }
}
