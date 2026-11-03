use dioxus::prelude::*;
use ui_framework::layouts::object_page::{ObjectPage, ObjectPageAttribute};
use ui_framework::panels::status_badge::{StatusBadge, EntityStatus};
use ui_framework::panels::action_bar::StickyActionBar;

#[component]
pub fn OrderDetailView() -> Element {
    let mut has_changes = use_signal(|| false);

    let attributes = vec![
        ObjectPageAttribute { label: "Order Number".to_string(), value: "ORD-2026-9042".to_string() },
        ObjectPageAttribute { label: "Customer".to_string(), value: "Bavaria Power Systems".to_string() },
        ObjectPageAttribute { label: "Fulfillment Date".to_string(), value: "2026-03-25".to_string() },
        ObjectPageAttribute { label: "Total Amount".to_string(), value: "€38,400.50".to_string() },
    ];

    rsx! {
        ObjectPage {
            title: "Order ORD-2026-9042".to_string(),
            subtitle: Some("High-pressure cryogenic industrial components dispatch manifest".to_string()),
            attributes: attributes,
            status: rsx! { StatusBadge { status: EntityStatus::Active } },
            footer_action_bar: rsx! {
                StickyActionBar {
                    has_unsaved_changes: has_changes(),
                    onsave: move |_| has_changes.set(false),
                    oncancel: move |_| has_changes.set(false),
                }
            },

            div { class: "rounded-xl border border-border bg-card p-5 space-y-3 text-xs",
                h3 { class: "font-semibold text-sm", "Logistics & Quality Assurance" }
                p { class: "text-muted-foreground leading-relaxed",
                    "All components have cleared ultrasonic flaw detection. Gate pass authorization is logged under ISO-9001 certification."
                }
            }
        }
    }
}
