use dioxus::prelude::*;
use crate::settings::component_customizer::use_component_style;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
    Subtle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
    IconSm,
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    #[props(default = "button")] r#type: &'static str,
    #[props(default = "")] class: &'static str,
    custom_id: Option<&'static str>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let custom_style = use_component_style("button", custom_id);

    let variant_classes = match variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground shadow-2xs hover:bg-primary/90 active:bg-primary/95",
        ButtonVariant::Destructive => "bg-destructive text-destructive-foreground shadow-2xs hover:bg-destructive/90 active:bg-destructive/95",
        ButtonVariant::Outline => "border border-input bg-background shadow-2xs hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground shadow-2xs hover:bg-secondary/80",
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
        ButtonVariant::Subtle => "bg-muted text-muted-foreground hover:bg-muted/80 hover:text-foreground",
    };

    let size_classes = match size {
        ButtonSize::Default => "h-8 px-3 py-1.5 text-xs",
        ButtonSize::Sm => "h-7 rounded-sm px-2.5 text-[11px]",
        ButtonSize::Lg => "h-9 rounded-md px-4 text-sm",
        ButtonSize::Icon => "h-8 w-8 p-0",
        ButtonSize::IconSm => "h-7 w-7 p-0",
    };

    let custom_class_str = custom_style.class_name.unwrap_or_default();

    rsx! {
        button {
            r#type: "{r#type}",
            disabled: disabled,
            class: "inline-flex items-center justify-center whitespace-nowrap rounded-md font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 select-none cursor-pointer {variant_classes} {size_classes} {custom_class_str} {class}",
            onclick: move |e| {
                if let Some(handler) = &onclick {
                    handler.call(e);
                }
            },
            {children}
        }
    }
}
