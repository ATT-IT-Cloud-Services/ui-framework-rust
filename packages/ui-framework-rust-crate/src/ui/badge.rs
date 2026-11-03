use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Success,
    Warning,
    Info,
    Muted,
}

#[component]
pub fn Badge(
    #[props(default)] variant: BadgeVariant,
    #[props(default = "")] class: &'static str,
    children: Element,
) -> Element {
    let variant_classes = match variant {
        BadgeVariant::Default => "border-transparent bg-primary text-primary-foreground shadow-2xs",
        BadgeVariant::Secondary => "border-transparent bg-secondary text-secondary-foreground",
        BadgeVariant::Destructive => "border-transparent bg-destructive text-destructive-foreground shadow-2xs",
        BadgeVariant::Outline => "text-foreground border border-border",
        BadgeVariant::Success => "border-emerald-200 dark:border-emerald-800/60 bg-emerald-50 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300 border",
        BadgeVariant::Warning => "border-amber-200 dark:border-amber-800/60 bg-amber-50 text-amber-700 dark:bg-amber-950/60 dark:text-amber-300 border",
        BadgeVariant::Info => "border-sky-200 dark:border-sky-800/60 bg-sky-50 text-sky-700 dark:bg-sky-950/60 dark:text-sky-300 border",
        BadgeVariant::Muted => "border-transparent bg-muted text-muted-foreground",
    };

    rsx! {
        div {
            class: "inline-flex items-center rounded-sm px-2 py-0.5 text-[11px] font-medium transition-colors select-none {variant_classes} {class}",
            {children}
        }
    }
}
