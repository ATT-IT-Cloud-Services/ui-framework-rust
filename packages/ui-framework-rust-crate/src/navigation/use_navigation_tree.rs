use dioxus::prelude::*;
use super::types::NavigationItem;
use crate::adapters::auth_adapter::use_session;
use crate::i18n::provider::use_i18n;

pub fn filter_by_permissions(
    items: &[NavigationItem],
    roles: &[String],
    permissions: &[String],
) -> Vec<NavigationItem> {
    items
        .iter()
        .filter(|item| {
            if !item.required_roles.is_empty() {
                let has_role = roles.iter().any(|r| r == "admin" || item.required_roles.contains(r));
                if !has_role {
                    return false;
                }
            }
            if !item.required_permissions.is_empty() {
                let has_perm = permissions.iter().any(|p| item.required_permissions.contains(p));
                if !has_perm && !roles.iter().any(|r| r == "admin") {
                    return false;
                }
            }
            true
        })
        .map(|item| {
            let mut cloned = item.clone();
            cloned.children = filter_by_permissions(&item.children, roles, permissions);
            cloned
        })
        .collect()
}

pub fn use_navigation_tree(items: Vec<NavigationItem>) -> Memo<Vec<NavigationItem>> {
    let session = use_session();
    let i18n = use_i18n();

    use_memo(move || {
        let current_roles = session.read().roles.clone();
        let current_perms = session.read().permissions.clone();
        let filtered = filter_by_permissions(&items, &current_roles, &current_perms);

        // Translate labels if label_key is present
        filtered
            .into_iter()
            .map(|mut item| {
                if let Some(key) = &item.label_key {
                    item.label = i18n.t(key);
                }
                item.children = item
                    .children
                    .into_iter()
                    .map(|mut child| {
                        if let Some(key) = &child.label_key {
                            child.label = i18n.t(key);
                        }
                        child
                    })
                    .collect();
                item
            })
            .collect()
    })
}
