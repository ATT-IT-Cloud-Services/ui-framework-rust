use ui_framework::config::{is_allowed_combination, NavigationMode, NavigationPosition, ZIndex};

#[test]
fn test_allowed_combinations_matrix() {
    assert!(is_allowed_combination(NavigationMode::SidebarFull, NavigationPosition::Start));
    assert!(is_allowed_combination(NavigationMode::SidebarFull, NavigationPosition::End));
    assert!(!is_allowed_combination(NavigationMode::SidebarFull, NavigationPosition::Top));
    assert!(!is_allowed_combination(NavigationMode::SidebarFull, NavigationPosition::Bottom));

    assert!(is_allowed_combination(NavigationMode::Topnav, NavigationPosition::Top));
    assert!(!is_allowed_combination(NavigationMode::Topnav, NavigationPosition::Start));

    assert!(is_allowed_combination(NavigationMode::BottomNav, NavigationPosition::Bottom));
}

#[test]
fn test_z_index_hierarchy() {
    assert!(ZIndex::NAV < ZIndex::PANEL);
    assert!(ZIndex::PANEL < ZIndex::HEADER);
    assert!(ZIndex::HEADER < ZIndex::SHEET);
    assert!(ZIndex::SHEET < ZIndex::DIALOG);
    assert!(ZIndex::DIALOG < ZIndex::POPOVER);
    assert!(ZIndex::POPOVER < ZIndex::TOAST);
    assert!(ZIndex::TOAST < ZIndex::TOOLTIP);
}
