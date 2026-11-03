use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WhiteLabelThemeConfig {
    pub app_name: String,
    pub app_subtitle: Option<String>,
    pub primary_hex: String,
    pub base_radius: String,
}

impl Default for WhiteLabelThemeConfig {
    fn default() -> Self {
        Self {
            app_name: "EnterpriseUI".to_string(),
            app_subtitle: Some("Operations Platform".to_string()),
            primary_hex: "#0064d9".to_string(),
            base_radius: "md".to_string(),
        }
    }
}
