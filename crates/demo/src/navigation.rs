use ui_framework::navigation::types::NavigationItem;

pub fn get_demo_navigation_items() -> Vec<NavigationItem> {
    vec![
        NavigationItem {
            id: "dashboard".to_string(),
            label: "Operations Dashboard".to_string(),
            label_key: Some("nav.dashboard".to_string()),
            href: Some("/demo/navigation/sidebar-full".to_string()),
            section: Some("Operations".to_string()),
            pinned: true,
            ..Default::default()
        },
        NavigationItem {
            id: "dispatch".to_string(),
            label: "Fulfillment & Orders".to_string(),
            label_key: Some("nav.orders".to_string()),
            href: Some("/demo/layouts/master-detail".to_string()),
            badge: Some("5".to_string()),
            section: Some("Operations".to_string()),
            children: vec![
                NavigationItem {
                    id: "orders-all".to_string(),
                    label: "All Orders".to_string(),
                    label_key: Some("nav.ordersAll".to_string()),
                    href: Some("/demo/layouts/master-detail".to_string()),
                    ..Default::default()
                },
                NavigationItem {
                    id: "orders-pending".to_string(),
                    label: "Pending Approvals".to_string(),
                    label_key: Some("nav.ordersPending".to_string()),
                    badge: Some("1".to_string()),
                    badge_variant: Some("warning".to_string()),
                    href: Some("/demo/layouts/master-detail".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        NavigationItem {
            id: "telemetry".to_string(),
            label: "Telemetry & SCADA".to_string(),
            label_key: Some("nav.telemetry".to_string()),
            href: Some("/demo/realtime/operator-console".to_string()),
            section: Some("Cyber-Physical".to_string()),
            tag: Some("LIVE".to_string()),
            required_roles: vec!["admin".to_string(), "editor".to_string()],
            children: vec![
                NavigationItem {
                    id: "telemetry-turbines".to_string(),
                    label: "Turbine Clusters".to_string(),
                    label_key: Some("nav.turbines".to_string()),
                    href: Some("/demo/realtime/operator-console".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        NavigationItem {
            id: "alarms".to_string(),
            label: "System Alarms".to_string(),
            label_key: Some("nav.alarms".to_string()),
            href: Some("/demo/feedback/notifications".to_string()),
            badge: Some("2 Critical".to_string()),
            badge_variant: Some("destructive".to_string()),
            section: Some("Cyber-Physical".to_string()),
            required_roles: vec!["admin".to_string(), "editor".to_string()],
            ..Default::default()
        },
        NavigationItem {
            id: "reports".to_string(),
            label: "Compliance & Audit".to_string(),
            label_key: Some("nav.reports".to_string()),
            href: Some("/demo/layouts/object-page".to_string()),
            section: Some("Management".to_string()),
            required_roles: vec!["admin".to_string()],
            ..Default::default()
        },
        NavigationItem {
            id: "split-layouts".to_string(),
            label: "Split Layouts".to_string(),
            label_key: Some("nav.splitLayouts".to_string()),
            href: Some("/demo/layouts/split-info-panel".to_string()),
            section: Some("Framework".to_string()),
            tag: Some("GALLERY".to_string()),
            ..Default::default()
        },
        NavigationItem {
            id: "settings".to_string(),
            label: "Workspace Settings".to_string(),
            label_key: Some("nav.settings".to_string()),
            href: Some("/demo/layouts/settings-layout".to_string()),
            section: Some("Configuration".to_string()),
            required_roles: vec!["admin".to_string()],
            ..Default::default()
        },
    ]
}
