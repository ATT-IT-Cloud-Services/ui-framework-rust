use dioxus::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SkeletonVariant {
    Pulse,
    #[default]
    Shimmer,
}

#[component]
pub fn Skeleton(
    #[props(default)] variant: SkeletonVariant,
    #[props(default = "")] class: &'static str,
) -> Element {
    let anim_class = match variant {
        SkeletonVariant::Pulse => "animate-pulse bg-muted/70",
        SkeletonVariant::Shimmer => "bg-gradient-to-r from-muted/60 via-muted/20 to-muted/60 bg-[length:200%_100%] animate-[shimmer_2s_ease-in-out_infinite]",
    };

    rsx! {
        div { class: "rounded-md {anim_class} {class}" }
    }
}
