#[derive(Debug, Clone, PartialEq)]
pub struct MockProduct {
    pub sku: String,
    pub name: String,
    pub unit_price: f64,
}

pub fn get_mock_products() -> Vec<MockProduct> {
    vec![
        MockProduct {
            sku: "VLV-IND-320".to_string(),
            name: "Industrial Cryo Valve 320mm".to_string(),
            unit_price: 1250.0,
        },
        MockProduct {
            sku: "SNS-VIB-90".to_string(),
            name: "Tri-Axial Vibration Sensor".to_string(),
            unit_price: 840.0,
        },
        MockProduct {
            sku: "PLC-MOD-X8".to_string(),
            name: "Safety PLC Controller X8".to_string(),
            unit_price: 4600.0,
        },
    ]
}
