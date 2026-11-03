use dioxus::prelude::*;
use ui_framework::shell::{AppShell, PageHeader, UserMenu};
use ui_framework::layouts::split_with_info_panel::{SplitDisplayMode, SplitRatio, SplitWithInfoPanel};
use ui_framework::panels::info_panel::{InfoPanel, KeyValuePair};
use ui_framework::overlays::notification_center::NotificationCenter;
use ui_framework::adapters::types::{AuthAdapter, AuthUser, SessionStatus};
use ui_framework::adapters::auth_adapter::use_session;
use crate::navigation::get_demo_navigation_items;
use super::orders::OrdersView;

#[component]
pub fn FlagshipShowcaseView() -> Element {
    let mut session = use_session();
    let mut current_role = use_signal(|| "admin".to_string());
    let mut authenticated = use_signal(|| true);

    let role_val = current_role();

    let user = AuthUser {
        id: "usr-1".to_string(),
        name: if role_val == "admin" { "Dr. Elena Rostova".to_string() } else if role_val == "editor" { "Marcus Vance".to_string() } else { "Aisha Al-Mansoor".to_string() },
        email: format!("{role_val}@enterprise.internal"),
        avatar_url: None,
        roles: vec![role_val.clone()],
        permissions: if role_val == "admin" { vec!["orders:read".to_string(), "orders:write".to_string(), "telemetry:audit".to_string()] } else if role_val == "editor" { vec!["orders:read".to_string(), "orders:write".to_string()] } else { vec!["orders:read".to_string()] },
        tenant_id: Some("t-central-grid".to_string()),
        tenant_name: Some("Central Grid Operator".to_string()),
    };

    use_effect(move || {
        let r = current_role();
        let u = AuthUser {
            id: "usr-1".to_string(),
            name: if r == "admin" { "Dr. Elena Rostova".to_string() } else if r == "editor" { "Marcus Vance".to_string() } else { "Aisha Al-Mansoor".to_string() },
            email: format!("{r}@enterprise.internal"),
            avatar_url: None,
            roles: vec![r.clone()],
            permissions: if r == "admin" { vec!["orders:read".to_string(), "orders:write".to_string(), "telemetry:audit".to_string()] } else if r == "editor" { vec!["orders:read".to_string(), "orders:write".to_string()] } else { vec!["orders:read".to_string()] },
            tenant_id: Some("t-central-grid".to_string()),
            tenant_name: Some("Central Grid Operator".to_string()),
        };
        session.set(AuthAdapter {
            status: SessionStatus::Authenticated,
            user: Some(u),
            roles: vec![r.clone()],
            permissions: if r == "admin" { vec!["orders:read".to_string(), "orders:write".to_string(), "telemetry:audit".to_string()] } else if r == "editor" { vec!["orders:read".to_string(), "orders:write".to_string()] } else { vec!["orders:read".to_string()] },
        });
    });

    if !authenticated() {
        return rsx! {
            div { class: "min-h-screen flex flex-col items-center justify-center p-6 bg-muted/20 text-xs",
                div { class: "w-full max-w-md p-6 rounded-xl border border-border bg-card shadow-lg space-y-4",
                    h2 { class: "text-lg font-bold", "Mock Authentication Gateway" }
                    p { class: "text-muted-foreground", "Select a role to sign in to the operations showcase." }
                    div { class: "grid grid-cols-3 gap-2",
                        for r in ["admin", "editor", "viewer"] {
                            button {
                                r#type: "button",
                                class: "py-2 px-3 rounded-lg border border-primary bg-primary text-primary-foreground font-semibold cursor-pointer uppercase text-xs",
                                onclick: move |_| {
                                    current_role.set(r.to_string());
                                    authenticated.set(true);
                                },
                                "{r}"
                            }
                        }
                    }
                }
            }
        };
    }

    let info_content = rsx! {
        InfoPanel {
            title: "Operator Security Context".to_string(),
            subtitle: Some(format!("{} ({})", user.name, role_val.to_uppercase())),
            key_values: vec![
                KeyValuePair { key: "Security Enclave".to_string(), value: "Level 4 (EU-Central)".to_string() },
                KeyValuePair { key: "Delegated Role".to_string(), value: role_val.to_uppercase() },
                KeyValuePair { key: "Tenant".to_string(), value: "Central Grid Operator".to_string() },
            ],
        }
    };

    rsx! {
        AppShell {
            navigation_items: get_demo_navigation_items(),
            header_slot: rsx! {
                div { class: "flex items-center gap-2",
                    NotificationCenter {}
                    // Live Role Switcher in Header
                    div { class: "flex items-center gap-1 bg-muted p-0.5 rounded text-2xs",
                        span { class: "px-1.5 text-muted-foreground font-semibold", "Role:" }
                        for r in ["admin", "editor", "viewer"] {
                            {
                                let is_active = current_role() == r;
                                rsx! {
                                    button {
                                        key: "{r}",
                                        r#type: "button",
                                        class: if is_active {
                                            "px-2 py-0.5 rounded bg-primary text-primary-foreground font-bold shadow-2xs cursor-pointer uppercase"
                                        } else {
                                            "px-2 py-0.5 rounded text-muted-foreground hover:text-foreground cursor-pointer uppercase"
                                        },
                                        onclick: move |_| current_role.set(r.to_string()),
                                        "{r}"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            user_menu_slot: rsx! {
                UserMenu {
                    onsignout: move |_| authenticated.set(false),
                }
            },

            div { class: "space-y-4",
                PageHeader {
                    title: "Enterprise Dispatch & SCADA Control Room".to_string(),
                    subtitle: Some(format!("Signed in as {}. Role privileges apply in real time without reload.", user.name)),
                }

                SplitWithInfoPanel {
                    info_panel: info_content,
                    default_ratio: SplitRatio::TwoThirds,
                    default_mode: SplitDisplayMode::Docked,
                    panel_title: "Operator Security & Context",
                    OrdersView {}
                }
            }
        }
    }
}
