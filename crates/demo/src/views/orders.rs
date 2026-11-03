use dioxus::prelude::*;
use ui_framework::panels::data_table::{DataTable, TableColumn};
use ui_framework::panels::status_badge::StatusBadge;
use ui_framework::feedback::wizard_dialog::{WizardDialog, WizardStepInfo};
use ui_framework::ui::input::Input;
use ui_framework::ui::slider::Slider;
use ui_framework::ui::numeric_input::NumericInput;
use ui_framework::ui::checkbox::Checkbox;
use ui_framework::overlays::toasts::{use_toast, ToastType};
use crate::mock_data::orders::{get_mock_orders, MockOrder};
use crate::mock_data::customers::get_mock_customers;
use crate::mock_data::products::get_mock_products;

#[component]
pub fn OrdersView() -> Element {
    let mut data = use_signal(get_mock_orders);
    let mut wizard_open = use_signal(|| false);
    let mut current_step = use_signal(|| 0usize);
    let toast = use_toast();

    // Wizard Form State
    let customers = get_mock_customers();
    let products = get_mock_products();

    let mut selected_customer = use_signal(|| customers.first().map(|c| c.name.clone()).unwrap_or_default());
    let mut destination = use_signal(|| "Oslo, Norway".to_string());
    let mut quantity = use_signal(|| 12.0f64);
    let unit_price = 1250.0f64;
    let mut compliance_checked = use_signal(|| true);

    let columns = vec![
        TableColumn {
            key: "order_number",
            title: "Order #",
            can_sort: true,
            render: |ord: &MockOrder| rsx! {
                span { class: "font-mono font-semibold text-primary", "{ord.order_number}" }
            },
        },
        TableColumn {
            key: "customer",
            title: "Customer",
            can_sort: true,
            render: |ord: &MockOrder| rsx! {
                div { class: "flex flex-col",
                    span { class: "font-medium text-foreground", "{ord.customer_name}" }
                    span { class: "text-2xs text-muted-foreground", "{ord.customer_email}" }
                }
            },
        },
        TableColumn {
            key: "status",
            title: "Status",
            can_sort: true,
            render: |ord: &MockOrder| rsx! {
                StatusBadge { status: ord.status }
            },
        },
        TableColumn {
            key: "amount",
            title: "Amount",
            can_sort: true,
            render: |ord: &MockOrder| rsx! {
                span { class: "font-mono font-semibold text-foreground", "€{ord.amount:.2}" }
            },
        },
        TableColumn {
            key: "destination",
            title: "Destination",
            can_sort: false,
            render: |ord: &MockOrder| rsx! {
                span { class: "text-muted-foreground", "{ord.destination}" }
            },
        },
    ];

    let total_amount = quantity() * unit_price;

    let steps = vec![
        WizardStepInfo { id: "s1".to_string(), title: "1. Customer".to_string() },
        WizardStepInfo { id: "s2".to_string(), title: "2. Allocation".to_string() },
        WizardStepInfo { id: "s3".to_string(), title: "3. Review".to_string() },
    ];

    rsx! {
        div { class: "space-y-4",
            div { class: "flex items-center justify-between pb-2 border-b border-border/80",
                div {
                    h2 { class: "text-base font-semibold text-foreground", "Dispatch & Fulfillment Orders" }
                    p { class: "text-xs text-muted-foreground", "Manage components dispatch manifests and quality approvals." }
                }
                button {
                    r#type: "button",
                    class: "h-8 px-3 rounded-md bg-primary text-primary-foreground font-semibold text-xs cursor-pointer flex items-center gap-1.5",
                    onclick: move |_| {
                        current_step.set(0);
                        wizard_open.set(true);
                    },
                    "＋ Create Order"
                }
            }

            DataTable {
                columns: columns,
                data: data(),
                enable_selection: true,
            }

            // Multi-step Wizard Modal
            WizardDialog {
                open: wizard_open(),
                title: "Create Industrial Purchase Order".to_string(),
                subtitle: Some("3-step wizard with component allocation, slider quantity, and live price calc.".to_string()),
                steps: steps,
                current_step: current_step(),
                finish_label: "Submit Order".to_string(),
                onstepchange: move |new_idx| current_step.set(new_idx),
                onclose: move |_| wizard_open.set(false),
                onfinish: move |_| {
                    let num = format!("ORD-2026-{}", 9100 + data().len());
                    let new_ord = MockOrder {
                        id: format!("ord-{}", data().len() + 1),
                        order_number: num.clone(),
                        customer_name: selected_customer(),
                        customer_email: "orders@customer.internal".to_string(),
                        date: "2026-03-31".to_string(),
                        amount: total_amount,
                        status: ui_framework::panels::status_badge::EntityStatus::Active,
                        item_count: quantity() as usize,
                        payment_method: "Invoice".to_string(),
                        destination: destination(),
                    };
                    data.write().insert(0, new_ord);
                    wizard_open.set(false);
                    toast(&format!("Order {num} created!"), ToastType::Success, Some("Manifest dispatched to logistics"));
                },

                if current_step() == 0 {
                    div { class: "space-y-4 text-xs",
                        div { class: "space-y-1",
                            label { class: "text-2xs font-semibold text-muted-foreground uppercase", "Customer Entity" }
                            select {
                                class: "w-full h-8 px-2 rounded border border-input bg-background text-xs cursor-pointer",
                                value: "{selected_customer()}",
                                onchange: move |e| selected_customer.set(e.value()),
                                for cust in customers {
                                    option { value: "{cust.name}", "{cust.name} ({cust.country})" }
                                }
                            }
                        }
                        div { class: "space-y-1",
                            label { class: "text-2xs font-semibold text-muted-foreground uppercase", "Destination Facility" }
                            Input {
                                value: "{destination()}",
                                oninput: move |e: FormEvent| destination.set(e.value()),
                            }
                        }
                    }
                } else if current_step() == 1 {
                    div { class: "space-y-4 text-xs",
                        div { class: "p-3 rounded-lg border border-border bg-muted/20 space-y-3",
                            div { class: "flex items-center justify-between",
                                span { class: "font-semibold text-foreground", "Order Quantity" }
                                NumericInput {
                                    value: quantity(),
                                    min: 1.0,
                                    max: 100.0,
                                    unit: "units",
                                    onchange: move |v| quantity.set(v),
                                }
                            }
                            Slider {
                                value: quantity(),
                                min: 1.0,
                                max: 100.0,
                                show_value: true,
                                value_suffix: " units",
                                onchange: move |v| quantity.set(v),
                            }
                        }

                        div { class: "flex justify-between items-center p-3 rounded-lg border border-primary/20 bg-primary/5 text-xs",
                            span { class: "text-muted-foreground", "Calculated Subtotal:" }
                            span { class: "font-mono text-base font-bold text-primary", "€{total_amount:.2}" }
                        }
                    }
                } else {
                    div { class: "space-y-3 text-xs",
                        div { class: "p-3 rounded-lg border border-border bg-card space-y-1.5",
                            div { class: "flex justify-between",
                                span { class: "text-muted-foreground", "Customer:" }
                                span { class: "font-semibold", "{selected_customer()}" }
                            }
                            div { class: "flex justify-between",
                                span { class: "text-muted-foreground", "Destination:" }
                                span { class: "font-semibold", "{destination()}" }
                            }
                            div { class: "flex justify-between pt-1 border-t border-border/60",
                                span { class: "font-bold", "Total Payable:" }
                                span { class: "font-mono font-bold text-primary text-sm", "€{total_amount:.2}" }
                            }
                        }
                        div { class: "pt-2",
                            Checkbox {
                                checked: compliance_checked(),
                                label: "Verified ISO-9001 compliance standards cleared",
                                onchange: move |c| compliance_checked.set(c),
                            }
                        }
                    }
                }
            }
        }
    }
}
