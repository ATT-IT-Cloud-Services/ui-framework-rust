use serde::{Deserialize, Serialize};
use ui_framework::panels::status_badge::EntityStatus;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MockOrder {
    pub id: String,
    pub order_number: String,
    pub customer_name: String,
    pub customer_email: String,
    pub date: String,
    pub amount: f64,
    pub status: EntityStatus,
    pub item_count: usize,
    pub payment_method: String,
    pub destination: String,
}

pub fn get_mock_orders() -> Vec<MockOrder> {
    vec![
        MockOrder {
            id: "ord-101".to_string(),
            order_number: "ORD-2026-9041".to_string(),
            customer_name: "Nordic Automation AS".to_string(),
            customer_email: "procurement@nordic-auto.no".to_string(),
            date: "2026-03-24".to_string(),
            amount: 14250.0,
            status: EntityStatus::Completed,
            item_count: 8,
            payment_method: "Invoice".to_string(),
            destination: "Oslo, Norway".to_string(),
        },
        MockOrder {
            id: "ord-102".to_string(),
            order_number: "ORD-2026-9042".to_string(),
            customer_name: "Bavaria Power Systems".to_string(),
            customer_email: "engineering@bavaria-power.de".to_string(),
            date: "2026-03-25".to_string(),
            amount: 38400.5,
            status: EntityStatus::Active,
            item_count: 24,
            payment_method: "Wire Transfer".to_string(),
            destination: "Munich, Germany".to_string(),
        },
        MockOrder {
            id: "ord-103".to_string(),
            order_number: "ORD-2026-9043".to_string(),
            customer_name: "Helvetia Precision AG".to_string(),
            customer_email: "orders@helvetia-prec.ch".to_string(),
            date: "2026-03-26".to_string(),
            amount: 7200.0,
            status: EntityStatus::Pending,
            item_count: 4,
            payment_method: "Card".to_string(),
            destination: "Zurich, Switzerland".to_string(),
        },
        MockOrder {
            id: "ord-104".to_string(),
            order_number: "ORD-2026-9044".to_string(),
            customer_name: "Rhône Energy SAS".to_string(),
            customer_email: "logistics@rhone-energy.fr".to_string(),
            date: "2026-03-27".to_string(),
            amount: 51200.0,
            status: EntityStatus::Warning,
            item_count: 16,
            payment_method: "Invoice".to_string(),
            destination: "Lyon, France".to_string(),
        },
        MockOrder {
            id: "ord-105".to_string(),
            order_number: "ORD-2026-9045".to_string(),
            customer_name: "Iberica Grid S.L.".to_string(),
            customer_email: "supplies@ibericagrid.es".to_string(),
            date: "2026-03-28".to_string(),
            amount: 9800.0,
            status: EntityStatus::Error,
            item_count: 3,
            payment_method: "SEPA".to_string(),
            destination: "Madrid, Spain".to_string(),
        },
    ]
}
