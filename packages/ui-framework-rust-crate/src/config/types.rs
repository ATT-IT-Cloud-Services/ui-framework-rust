use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum NavigationMode {
    #[default]
    SidebarFull,
    SidebarRail,
    SidebarHybrid,
    SidebarDual,
    SidebarInset,
    Topnav,
    BottomNav,
}

impl NavigationMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SidebarFull => "sidebar-full",
            Self::SidebarRail => "sidebar-rail",
            Self::SidebarHybrid => "sidebar-hybrid",
            Self::SidebarDual => "sidebar-dual",
            Self::SidebarInset => "sidebar-inset",
            Self::Topnav => "topnav",
            Self::BottomNav => "bottom-nav",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum NavigationPosition {
    #[default]
    Start,
    End,
    Top,
    Bottom,
}

impl NavigationPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ContentWidth {
    Boxed,
    #[default]
    Fluid,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Density {
    #[default]
    Comfortable,
    Compact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Radius {
    Sm,
    #[default]
    Md,
    Lg,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BrandColor {
    #[default]
    Blue,
    Teal,
    Violet,
    Green,
    Orange,
    Custom(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ThemeMode {
    Light,
    Dark,
    #[default]
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ContrastMode {
    #[default]
    Normal,
    High,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationMatrixEntry {
    pub mode: NavigationMode,
    pub allowed_positions: Vec<NavigationPosition>,
    pub default_position: NavigationPosition,
    pub supports_hover_expand: bool,
    pub supports_dual_rail: bool,
}
