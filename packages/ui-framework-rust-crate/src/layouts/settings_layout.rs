use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsTabItem {
    pub id: String,
    pub label: String,
    pub badge: Option<String>,
}

#[component]
pub fn SettingsLayout(
    title: String,
    #[props(default = None)] subtitle: Option<String>,
    tabs: Vec<SettingsTabItem>,
    active_tab_id: String,
    #[props(default = "")] class: &'static str,
    on_select_tab: EventHandler<String>,
    aside_footer: Option<Element>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "w-full max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-6 space-y-6 {class}",
            // Header Bar
            div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between pb-4 border-b border-border gap-3",
                div { class: "space-y-1",
                    h1 { class: "text-xl font-bold tracking-tight text-foreground", "{title}" }
                    if let Some(sub) = subtitle {
                        p { class: "text-xs text-muted-foreground", "{sub}" }
                    }
                }
            }

            // Main Grid: Equal Height Cards
            div { class: "flex flex-col md:flex-row gap-6 items-stretch min-h-[580px]",
                // Sub-navigation Card
                aside { class: "w-full md:w-64 shrink-0 rounded-xl border border-border bg-card p-4 shadow-2xs flex flex-col justify-between",
                    div { class: "space-y-1",
                        span { class: "text-[10px] font-semibold uppercase tracking-wider text-muted-foreground px-2 pb-2 block border-b border-border/50 mb-1.5", "Categories" }
                        div { class: "space-y-1",
                            for tab in tabs {
                                {
                                    let is_active = active_tab_id == tab.id;
                                    let id_clone = tab.id.clone();
                                    rsx! {
                                        button {
                                            key: "{tab.id}",
                                            r#type: "button",
                                            class: if is_active {
                                                "w-full flex items-center justify-between rounded-lg px-3 py-2.5 text-xs font-semibold bg-primary text-primary-foreground shadow-xs cursor-pointer"
                                            } else {
                                                "w-full flex items-center justify-between rounded-lg px-3 py-2.5 text-xs font-medium text-muted-foreground hover:bg-muted hover:text-foreground cursor-pointer"
                                            },
                                            onclick: move |_| on_select_tab.call(id_clone.clone()),
                                            span { "{tab.label}" }
                                            if let Some(b) = tab.badge {
                                                span { class: "text-[10px] font-mono", "{b}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Aside Footer Status Box
                    div { class: "pt-4 mt-6 border-t border-border/60",
                        if let Some(f) = aside_footer {
                            {f}
                        } else {
                            div { class: "p-3 rounded-lg bg-muted/30 border border-border/40 text-2xs space-y-1.5",
                                div { class: "flex items-center justify-between text-muted-foreground",
                                    span { "🛡 Security" }
                                    span { class: "font-mono font-semibold text-emerald-600", "ENCLAVE 4" }
                                }
                                div { class: "flex items-center justify-between text-muted-foreground",
                                    span { "💾 Storage" }
                                    span { class: "font-mono", "Synced" }
                                }
                            }
                        }
                    }
                }

                // Settings Form Content
                main { class: "flex-1 w-full rounded-xl border border-border bg-card p-6 sm:p-8 shadow-2xs flex flex-col justify-between",
                    div { class: "flex-1", {children} }
                }
            }
        }
    }
}
