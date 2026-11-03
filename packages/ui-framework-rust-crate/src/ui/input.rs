use dioxus::prelude::*;
use crate::settings::component_customizer::use_component_style;

#[component]
pub fn Input(
    #[props(default = "text")] r#type: &'static str,
    #[props(default = String::new())] value: String,
    #[props(default = "")] placeholder: &'static str,
    #[props(default)] disabled: bool,
    #[props(default = "")] class: &'static str,
    custom_id: Option<&'static str>,
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
) -> Element {
    let custom_style = use_component_style("input", custom_id);
    let custom_class_str = custom_style.class_name.unwrap_or_default();

    rsx! {
        input {
            r#type: "{r#type}",
            value: "{value}",
            placeholder: "{placeholder}",
            disabled: disabled,
            class: "flex h-8 w-full rounded-md border border-input bg-background px-2.5 py-1 text-xs shadow-2xs transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 {custom_class_str} {class}",
            oninput: move |e| {
                if let Some(h) = &oninput {
                    h.call(e);
                }
            },
            onchange: move |e| {
                if let Some(h) = &onchange {
                    h.call(e);
                }
            }
        }
    }
}
