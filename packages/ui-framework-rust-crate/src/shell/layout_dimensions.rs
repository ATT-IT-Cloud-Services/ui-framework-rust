use dioxus::prelude::*;
use crate::config::types::NavigationMode;
use crate::settings::provider::use_layout_settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutDimensions {
    pub sidebar_width: u32,
    pub header_height: u32,
    pub nav_height: u32,
    pub footer_height: u32,
    pub is_mobile: bool,
}

impl Default for LayoutDimensions {
    fn default() -> Self {
        Self {
            sidebar_width: 260,
            header_height: 48,
            nav_height: 0,
            footer_height: 36,
            is_mobile: false,
        }
    }
}

pub fn use_layout_dimensions() -> LayoutDimensions {
    let settings = use_layout_settings();
    let s = settings.read();

    let sidebar_width = match s.navigation_mode {
        NavigationMode::SidebarRail => 64,
        NavigationMode::SidebarHybrid => 260,
        NavigationMode::SidebarDual => 264,
        NavigationMode::SidebarInset => 276,
        NavigationMode::Topnav | NavigationMode::BottomNav => 0,
        NavigationMode::SidebarFull => 260,
    };

    let header_height = if s.header_sticky { 48 } else { 0 };
    let nav_height = if s.navigation_mode == NavigationMode::Topnav { 40 } else { 0 };

    LayoutDimensions {
        sidebar_width,
        header_height,
        nav_height,
        footer_height: 36,
        is_mobile: false,
    }
}
