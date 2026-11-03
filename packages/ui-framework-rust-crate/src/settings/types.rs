use serde::{Deserialize, Serialize};
pub use crate::config::types::{BrandColor, ContentWidth, ContrastMode, Density, NavigationMode, NavigationPosition, Radius, ThemeMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SidebarState {
    #[default]
    Expanded,
    Collapsed,
    HoverExpand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ToastPosition {
    TopStart,
    TopCenter,
    TopEnd,
    BottomStart,
    #[default]
    BottomEnd,
    BottomCenter,
}

impl ToastPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TopStart => "top-start",
            Self::TopCenter => "top-center",
            Self::TopEnd => "top-end",
            Self::BottomStart => "bottom-start",
            Self::BottomEnd => "bottom-end",
            Self::BottomCenter => "bottom-center",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ToastStyle {
    #[default]
    Default,
    Solid,
    Outlined,
    Minimal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum TooltipDelay {
    Instant,
    #[default]
    Short,
    Long,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PopoverTriggerMode {
    #[default]
    Click,
    Hover,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OverlayAnimation {
    None,
    #[default]
    Subtle,
    Smooth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BackdropBlur {
    None,
    #[default]
    Sm,
    Md,
    Lg,
}

impl BackdropBlur {
    pub fn as_px(&self) -> &'static str {
        match self {
            Self::None => "0px",
            Self::Sm => "4px",
            Self::Md => "8px",
            Self::Lg => "16px",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SkeletonStyle {
    #[default]
    Shimmer,
    Pulse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum LoadingIndicatorType {
    #[default]
    Bar,
    Spinner,
    Both,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadingSettings {
    pub show_after: u64, // ms delay (e.g. 200)
    pub min_duration: u64, // ms minimum visibility (e.g. 350)
    pub skeleton_style: SkeletonStyle,
    pub indicator_type: LoadingIndicatorType,
}

impl Default for LoadingSettings {
    fn default() -> Self {
        Self {
            show_after: 200,
            min_duration: 350,
            skeleton_style: SkeletonStyle::Shimmer,
            indicator_type: LoadingIndicatorType::Bar,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FontFamilyPreset {
    #[default]
    Inter,
    Geist,
    Roboto,
    Jakarta,
    IbmPlex,
    System,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MonoFontPreset {
    #[default]
    Jetbrains,
    GeistMono,
    FiraCode,
    IbmPlexMono,
    SystemMono,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypographySettings {
    pub font_sans: FontFamilyPreset,
    pub custom_sans_font: Option<String>,
    pub font_mono: MonoFontPreset,
    pub custom_mono_font: Option<String>,
    pub font_heading: FontFamilyPreset,
    pub custom_heading_font: Option<String>,
    pub font_size: String,
    pub letter_spacing: String,
}

impl Default for TypographySettings {
    fn default() -> Self {
        Self {
            font_sans: FontFamilyPreset::Inter,
            custom_sans_font: None,
            font_mono: MonoFontPreset::Jetbrains,
            custom_mono_font: None,
            font_heading: FontFamilyPreset::Inter,
            custom_heading_font: None,
            font_size: "14px".to_string(),
            letter_spacing: "normal".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ContentPadding {
    None,
    Compact,
    #[default]
    Comfortable,
    Spacious,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MobileLayoutBehavior {
    #[default]
    Stack,
    Drawer,
    Tabs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ContentOverflowMode {
    #[default]
    AutoScroll,
    FillViewport,
    Expand,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentLayoutOptions {
    pub padding: ContentPadding,
    pub mobile_padding: ContentPadding,
    pub mobile_behavior: MobileLayoutBehavior,
    pub overflow_mode: ContentOverflowMode,
}

impl Default for ContentLayoutOptions {
    fn default() -> Self {
        Self {
            padding: ContentPadding::Comfortable,
            mobile_padding: ContentPadding::Compact,
            mobile_behavior: MobileLayoutBehavior::Stack,
            overflow_mode: ContentOverflowMode::AutoScroll,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverlaySettings {
    pub toast_position: ToastPosition,
    pub toast_duration: u64, // ms
    pub toast_max_visible: usize,
    pub toast_style: ToastStyle,
    pub toast_icons: bool,
    pub toast_close_button: bool,
    pub toast_expand_on_hover: bool,
    pub tooltip_delay: TooltipDelay,
    pub popover_trigger: PopoverTriggerMode,
    pub overlay_animation: OverlayAnimation,
    pub backdrop_blur: BackdropBlur,
    pub blur_left_on_menu_open: bool,
}

impl Default for OverlaySettings {
    fn default() -> Self {
        Self {
            toast_position: ToastPosition::BottomEnd,
            toast_duration: 4000,
            toast_max_visible: 4,
            toast_style: ToastStyle::Default,
            toast_icons: true,
            toast_close_button: true,
            toast_expand_on_hover: true,
            tooltip_delay: TooltipDelay::Short,
            popover_trigger: PopoverTriggerMode::Click,
            overlay_animation: OverlayAnimation::Subtle,
            backdrop_blur: BackdropBlur::Sm,
            blur_left_on_menu_open: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutSettings {
    pub navigation_mode: NavigationMode,
    pub navigation_position: NavigationPosition,
    pub sidebar_state: SidebarState,
    pub header_sticky: bool,
    pub sidebar_sticky: bool,
    pub show_breadcrumbs: bool,

    pub content_width: ContentWidth,
    pub content_layout: ContentLayoutOptions,
    pub density: Density,
    pub radius: Radius,
    pub typography: TypographySettings,
    pub brand_color: BrandColor,
    pub custom_brand_hex: Option<String>,
    pub theme: ThemeMode,
    pub contrast: ContrastMode,

    pub overlays: OverlaySettings,
    pub loading: LoadingSettings,
    pub locale: String,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            navigation_mode: NavigationMode::SidebarFull,
            navigation_position: NavigationPosition::Start,
            sidebar_state: SidebarState::Expanded,
            header_sticky: true,
            sidebar_sticky: true,
            show_breadcrumbs: true,

            content_width: ContentWidth::Fluid,
            content_layout: ContentLayoutOptions::default(),
            density: Density::Comfortable,
            radius: Radius::Md,
            typography: TypographySettings::default(),
            brand_color: BrandColor::Blue,
            custom_brand_hex: Some("#0064d9".to_string()),
            theme: ThemeMode::System,
            contrast: ContrastMode::Normal,

            overlays: OverlaySettings::default(),
            loading: LoadingSettings::default(),
            locale: "en".to_string(),
        }
    }
}
