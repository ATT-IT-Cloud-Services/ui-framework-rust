use super::types::{NavigationMatrixEntry, NavigationMode, NavigationPosition};

pub fn get_navigation_matrix(mode: NavigationMode) -> NavigationMatrixEntry {
    match mode {
        NavigationMode::SidebarFull => NavigationMatrixEntry {
            mode,
            allowed_positions: vec![NavigationPosition::Start, NavigationPosition::End],
            default_position: NavigationPosition::Start,
            supports_hover_expand: false,
            supports_dual_rail: false,
        },
        NavigationMode::SidebarRail => NavigationMatrixEntry {
            mode,
            allowed_positions: vec![NavigationPosition::Start, NavigationPosition::End],
            default_position: NavigationPosition::Start,
            supports_hover_expand: true,
            supports_dual_rail: false,
        },
        NavigationMode::SidebarHybrid => NavigationMatrixEntry {
            mode,
            allowed_positions: vec![NavigationPosition::Start, NavigationPosition::End],
            default_position: NavigationPosition::Start,
            supports_hover_expand: true,
            supports_dual_rail: false,
        },
        NavigationMode::SidebarDual => NavigationMatrixEntry {
            mode,
            allowed_positions: vec![NavigationPosition::Start, NavigationPosition::End],
            default_position: NavigationPosition::Start,
            supports_hover_expand: false,
            supports_dual_rail: true,
        },
        NavigationMode::SidebarInset => NavigationMatrixEntry {
            mode,
            allowed_positions: vec![NavigationPosition::Start, NavigationPosition::End],
            default_position: NavigationPosition::Start,
            supports_hover_expand: false,
            supports_dual_rail: false,
        },
        NavigationMode::Topnav => NavigationMatrixEntry {
            mode,
            allowed_positions: vec![NavigationPosition::Top],
            default_position: NavigationPosition::Top,
            supports_hover_expand: false,
            supports_dual_rail: false,
        },
        NavigationMode::BottomNav => NavigationMatrixEntry {
            mode,
            allowed_positions: vec![NavigationPosition::Bottom, NavigationPosition::Top],
            default_position: NavigationPosition::Bottom,
            supports_hover_expand: false,
            supports_dual_rail: false,
        },
    }
}

pub fn is_allowed_combination(mode: NavigationMode, position: NavigationPosition) -> bool {
    let entry = get_navigation_matrix(mode);
    entry.allowed_positions.contains(&position)
}

pub fn get_allowed_positions(mode: NavigationMode) -> Vec<NavigationPosition> {
    get_navigation_matrix(mode).allowed_positions
}

pub fn get_default_position_for_mode(mode: NavigationMode) -> NavigationPosition {
    get_navigation_matrix(mode).default_position
}
