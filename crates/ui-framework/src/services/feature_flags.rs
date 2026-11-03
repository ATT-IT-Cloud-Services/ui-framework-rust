use std::collections::HashMap;
use dioxus::prelude::*;

#[derive(Clone, Default)]
pub struct FeatureFlagContext {
    pub flags: Signal<HashMap<String, bool>>,
}

pub fn use_feature_flag(flag: &str) -> bool {
    if let Some(ctx) = try_consume_context::<FeatureFlagContext>() {
        ctx.flags.read().get(flag).copied().unwrap_or(false)
    } else {
        true
    }
}

#[component]
pub fn Feature(
    flag: &'static str,
    fallback: Option<Element>,
    children: Element,
) -> Element {
    let enabled = use_feature_flag(flag);
    if enabled {
        rsx! { {children} }
    } else if let Some(f) = fallback {
        rsx! { {f} }
    } else {
        rsx! {}
    }
}
