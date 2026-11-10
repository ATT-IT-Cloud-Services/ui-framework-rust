use dioxus::prelude::*;
use super::types::SupportedLocale;
use super::provider::use_i18n;

#[component]
pub fn LanguageSwitcher(
    #[props(default = false)] show_name: bool,
    #[props(default = "")] class: &'static str,
    /// Accessible name of the trigger; defaults to the locale name.
    #[props(default = None)] label: Option<String>,
    /// Emoji flags can be switched off, e.g. where the host draws its own.
    #[props(default = true)] show_flag: bool,
) -> Element {
    let mut i18n = use_i18n();
    let mut open = use_signal(|| false);

    let current = (i18n.locale)();

    rsx! {
        div {
            class: "relative inline-block text-start {class}",
            button {
                r#type: "button",
                class: "inline-flex items-center gap-1.5 px-2.5 py-1.5 text-xs font-medium rounded-md border border-input bg-background hover:bg-muted/40 transition-colors cursor-pointer",
                aria_label: label.clone().unwrap_or_else(|| current.native_name().to_string()),
                title: label.clone().unwrap_or_else(|| current.native_name().to_string()),
                onclick: move |_| open.set(!open()),
                if show_flag {
                    span { "{current.flag()}" }
                }
                if show_name {
                    span { "{current.native_name()}" }
                }
                span { class: "text-[10px] text-muted-foreground uppercase ms-0.5", "({current.as_str()})" }
            }

            if open() {
                div {
                    class: "absolute end-0 mt-1.5 w-44 rounded-md border border-border bg-popover text-popover-foreground shadow-md p-1 z-[600] animate-in fade-in-50",
                    for loc in [
                        SupportedLocale::En,
                        SupportedLocale::De,
                        SupportedLocale::Fr,
                        SupportedLocale::Es,
                        SupportedLocale::Ar,
                    ] {
                        {
                            let is_active = (i18n.locale)() == loc;
                            rsx! {
                                button {
                                    r#type: "button",
                                    class: if is_active {
                                        "w-full flex items-center justify-between px-2.5 py-1.5 text-xs rounded bg-primary/10 text-primary font-semibold cursor-pointer"
                                    } else {
                                        "w-full flex items-center justify-between px-2.5 py-1.5 text-xs rounded hover:bg-muted text-foreground cursor-pointer"
                                    },
                                    onclick: move |_| {
                                        i18n.locale.set(loc);
                                        open.set(false);
                                    },
                                    div { class: "flex items-center gap-2",
                                        if show_flag {
                                            span { "{loc.flag()}" }
                                        }
                                        span { "{loc.native_name()}" }
                                    }
                                    span { class: "text-[10px] text-muted-foreground uppercase", "{loc.direction().as_str()}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
