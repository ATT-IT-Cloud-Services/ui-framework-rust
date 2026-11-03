#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MockCustomer {
    pub id: String,
    pub name: String,
    pub country: String,
    pub industry: String,
}

pub fn get_mock_customers() -> Vec<MockCustomer> {
    vec![
        MockCustomer {
            id: "c-1".to_string(),
            name: "Nordic Automation AS".to_string(),
            country: "Norway".to_string(),
            industry: "Wind Power".to_string(),
        },
        MockCustomer {
            id: "c-2".to_string(),
            name: "Bavaria Power Systems".to_string(),
            country: "Germany".to_string(),
            industry: "Turbomachinery".to_string(),
        },
        MockCustomer {
            id: "c-3".to_string(),
            name: "Helvetia Precision AG".to_string(),
            country: "Switzerland".to_string(),
            industry: "Precision Cryo".to_string(),
        },
    ]
}
