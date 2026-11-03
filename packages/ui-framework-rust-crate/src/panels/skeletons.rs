use dioxus::prelude::*;
use crate::ui::skeleton::Skeleton;

#[component]
pub fn TableSkeleton(
    #[props(default = 5)] rows: usize,
    #[props(default = 4)] cols: usize,
    #[props(default = "")] class: &'static str,
) -> Element {
    rsx! {
        div { class: "space-y-3 rounded-xl border border-border p-4 bg-card {class}",
            div { class: "flex items-center justify-between pb-2 border-b border-border/50",
                Skeleton { class: "h-7 w-48" }
                div { class: "flex gap-2",
                    Skeleton { class: "h-7 w-20" }
                    Skeleton { class: "h-7 w-20" }
                }
            }
            div { class: "space-y-2",
                for r in 0..rows {
                    div { key: "{r}", class: "flex gap-4 py-2 border-b border-border/40 last:border-none",
                        for c in 0..cols {
                            Skeleton { key: "{c}", class: "h-4 flex-1" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DashboardSkeleton(#[props(default = "")] class: &'static str) -> Element {
    rsx! {
        div { class: "space-y-6 {class}",
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4",
                for i in 0..4 {
                    Skeleton { key: "{i}", class: "h-24 w-full rounded-xl" }
                }
            }
            div { class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                Skeleton { class: "lg:col-span-2 h-72 rounded-xl" }
                Skeleton { class: "h-72 rounded-xl" }
            }
            TableSkeleton { rows: 4 }
        }
    }
}
