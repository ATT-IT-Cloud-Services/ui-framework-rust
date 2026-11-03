pub mod adapters;
pub mod config;
pub mod feedback;
pub mod i18n;
pub mod layouts;
pub mod navigation;
pub mod overlays;
pub mod panels;
pub mod realtime;
pub mod services;
pub mod settings;
pub mod shell;
pub mod ui;

// Re-export primary types and components cleanly
pub use adapters::{AuthAdapter, AuthUser, SessionProvider, SessionStatus, use_session};
pub use config::{
    is_allowed_combination, BrandColor, ContentWidth, ContrastMode, Density, NavigationMatrixEntry,
    NavigationMode, NavigationPosition, Radius, ThemeMode, ZIndex,
};
pub use feedback::*;
pub use i18n::{use_i18n, use_t, I18nContext, I18nProvider, LanguageSwitcher, SupportedLocale, TextDirection};
pub use layouts::*;
pub use navigation::*;
pub use overlays::*;
pub use panels::*;
pub use realtime::*;
pub use services::*;
pub use settings::{
    get_mono_font_definition, get_sans_font_definition, use_component_style, use_is_dark, use_layout_settings,
    BackdropBlur, ComponentRegistry, ContentLayoutOptions, ContentOverflowMode, ContentPadding,
    CustomComponentStyle, CustomizerDrawer, FontFamilyPreset, LayoutSettings,
    LayoutSettingsProvider, LoadingIndicatorType, LoadingSettings, MobileLayoutBehavior,
    MonoFontPreset, OverlaySettings, PopoverTriggerMode, SkeletonStyle, ToastPosition, ToastStyle,
    TooltipDelay, TypographySettings,
};
pub use shell::*;
pub use ui::*;
