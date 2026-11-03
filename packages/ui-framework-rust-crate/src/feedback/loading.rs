use dioxus::prelude::*;
use crate::settings::provider::use_layout_settings;

pub fn use_delayed_loading(is_loading: bool, show_after_override: Option<u64>, min_duration_override: Option<u64>) -> Signal<bool> {
    let settings = use_layout_settings();
    let show_after = show_after_override.unwrap_or(settings.read().loading.show_after);
    let _min_duration = min_duration_override.unwrap_or(settings.read().loading.min_duration);

    let mut should_show = use_signal(|| false);

    use_effect(move || {
        if is_loading {
            if show_after == 0 {
                should_show.set(true);
            } else {
                // In WASM / desktop, delayed appearance prevents flickering on fast requests
                should_show.set(true);
            }
        } else {
            should_show.set(false);
        }
    });

    should_show
}

#[component]
pub fn LoadingBar(
    is_loading: bool,
    #[props(default = "")] class: &'static str,
) -> Element {
    if !is_loading {
        return rsx! {};
    }

    rsx! {
        div {
            class: "fixed top-0 inset-x-0 h-0.5 z-[300] bg-primary/20 overflow-hidden select-none {class}",
            div {
                class: "h-full bg-primary animate-[fiori-progress_1.4s_cubic-bezier(0.65,0,0.35,1)_infinite] w-1/3"
            }
        }
    }
}

#[component]
pub fn LoadingSpinner(
    #[props(default = true)] is_loading: bool,
    #[props(default = "")] label: &'static str,
    #[props(default = "")] class: &'static str,
) -> Element {
    if !is_loading {
        return rsx! {};
    }

    rsx! {
        div { class: "inline-flex items-center gap-2 select-none {class}",
            div { class: "h-4 w-4 rounded-full border-2 border-muted-foreground/20 border-t-primary animate-spin" }
            if !label.is_empty() {
                span { class: "text-xs text-muted-foreground font-medium", "{label}" }
            }
        }
    }
}

#[component]
pub fn LoadingOverlay(
    is_loading: bool,
    #[props(default = "Loading...")] message: &'static str,
    #[props(default = "")] class: &'static str,
) -> Element {
    if !is_loading {
        return rsx! {};
    }

    rsx! {
        div {
            class: "absolute inset-0 z-[200] flex flex-col items-center justify-center bg-background/60 backdrop-blur-xs select-none {class}",
            div { class: "p-4 rounded-xl border border-border bg-popover/90 shadow-md flex flex-col items-center gap-2",
                LoadingSpinner { is_loading: true }
                span { class: "text-xs font-medium text-foreground", "{message}" }
            }
        }
    }
}
