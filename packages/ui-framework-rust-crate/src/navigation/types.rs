use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationItem {
    pub id: String,
    pub label: String,
    pub label_key: Option<String>,
    pub href: Option<String>,
    pub children: Vec<NavigationItem>,
    pub badge: Option<String>,
    pub badge_variant: Option<String>,
    pub tag: Option<String>,
    pub required_roles: Vec<String>,
    pub required_permissions: Vec<String>,
    pub section: Option<String>,
    pub pinned: bool,
}

impl Default for NavigationItem {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            label_key: None,
            href: None,
            children: Vec::new(),
            badge: None,
            badge_variant: None,
            tag: None,
            required_roles: Vec::new(),
            required_permissions: Vec::new(),
            section: None,
            pinned: false,
        }
    }
}
