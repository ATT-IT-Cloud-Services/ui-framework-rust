use dioxus::prelude::*;
use crate::adapters::auth_adapter::use_session;

#[derive(Clone, Default)]
pub struct PermissionContext {
    pub roles: Signal<Vec<String>>,
    pub permissions: Signal<Vec<String>>,
}

impl PermissionContext {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.read().iter().any(|r| r == "admin" || r == role)
    }

    pub fn has_permission(&self, perm: &str) -> bool {
        self.roles.read().iter().any(|r| r == "admin")
            || self.permissions.read().iter().any(|p| p == perm)
    }
}

pub fn use_permission() -> PermissionContext {
    if let Some(ctx) = try_consume_context::<PermissionContext>() {
        ctx
    } else {
        let session = use_session();
        let roles = session.read().roles.clone();
        let perms = session.read().permissions.clone();
        PermissionContext {
            roles: Signal::new(roles),
            permissions: Signal::new(perms),
        }
    }
}

#[component]
pub fn Can(
    #[props(default = None)] permission: Option<String>,
    #[props(default = None)] role: Option<String>,
    fallback: Option<Element>,
    children: Element,
) -> Element {
    let perm_ctx = use_permission();

    let allowed = match (permission, role) {
        (Some(p), Some(r)) => perm_ctx.has_permission(&p) && perm_ctx.has_role(&r),
        (Some(p), None) => perm_ctx.has_permission(&p),
        (None, Some(r)) => perm_ctx.has_role(&r),
        (None, None) => true,
    };

    if allowed {
        rsx! { {children} }
    } else if let Some(f) = fallback {
        rsx! { {f} }
    } else {
        rsx! {}
    }
}
