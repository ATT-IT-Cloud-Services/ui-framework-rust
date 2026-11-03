/// Strict single z-index scale for the entire application framework.
/// Prevents collisions between navigation, panels, headers, dialogs, overlays, and toasts.
pub struct ZIndex;

impl ZIndex {
    pub const BASE: i32 = 0;
    pub const NAV: i32 = 100;
    pub const PANEL: i32 = 200;
    pub const ACTIONBAR: i32 = 250;
    pub const HEADER: i32 = 300;
    pub const SHEET: i32 = 400;
    pub const DIALOG: i32 = 500;
    pub const POPOVER: i32 = 600;
    pub const TOAST: i32 = 700;
    pub const TOOLTIP: i32 = 800;
}
