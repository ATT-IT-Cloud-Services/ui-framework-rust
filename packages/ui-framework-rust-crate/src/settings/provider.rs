use dioxus::prelude::*;
use super::types::LayoutSettings;

#[derive(Clone, Copy)]
pub struct LayoutSettingsSignal(pub Signal<LayoutSettings>);

pub fn use_layout_settings() -> Signal<LayoutSettings> {
    use_context::<Signal<LayoutSettings>>()
}

#[component]
pub fn LayoutSettingsProvider(
    #[props(default)] initial: LayoutSettings,
    children: Element,
) -> Element {
    let settings = use_signal(|| initial);
    use_context_provider(|| settings);

    rsx! {
        {children}
    }
}
