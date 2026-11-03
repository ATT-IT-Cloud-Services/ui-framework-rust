use std::collections::HashMap;
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CustomComponentStyle {
    pub radius: Option<String>,
    pub density: Option<String>,
    pub variant: Option<String>,
    pub class_name: Option<String>,
    pub style: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ComponentRegistry {
    pub global: HashMap<String, CustomComponentStyle>,
    pub by_custom_id: HashMap<String, HashMap<String, CustomComponentStyle>>,
}

impl ComponentRegistry {
    pub fn get_style(&self, component_type: &str, custom_id: Option<&str>) -> CustomComponentStyle {
        let global = self.global.get(component_type).cloned().unwrap_or_default();
        let id_style = if let Some(id) = custom_id {
            self.by_custom_id
                .get(component_type)
                .and_then(|map| map.get(id))
                .cloned()
                .unwrap_or_default()
        } else {
            CustomComponentStyle::default()
        };

        CustomComponentStyle {
            radius: id_style.radius.or(global.radius),
            density: id_style.density.or(global.density),
            variant: id_style.variant.or(global.variant),
            class_name: match (global.class_name, id_style.class_name) {
                (Some(g), Some(i)) => Some(format!("{} {}", g, i)),
                (Some(g), None) => Some(g),
                (None, Some(i)) => Some(i),
                (None, None) => None,
            },
            style: match (global.style, id_style.style) {
                (Some(g), Some(i)) => Some(format!("{} {}", g, i)),
                (Some(g), None) => Some(g),
                (None, Some(i)) => Some(i),
                (None, None) => None,
            },
        }
    }
}

pub fn use_component_style(component_type: &str, custom_id: Option<&str>) -> CustomComponentStyle {
    let registry = try_consume_context::<Signal<ComponentRegistry>>();
    if let Some(reg) = registry {
        reg.read().get_style(component_type, custom_id)
    } else {
        CustomComponentStyle::default()
    }
}
