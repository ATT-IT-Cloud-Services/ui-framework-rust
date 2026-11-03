use dioxus::prelude::*;
use ui_framework::settings::provider::LayoutSettingsProvider;
use ui_framework::i18n::provider::I18nProvider;
use ui_framework::adapters::auth_adapter::SessionProvider;
use ui_framework::overlays::toasts::OverlayProvider;
use ui_framework::shell::AppShell;
use ui_framework::layouts::split_with_info_panel::{SplitDisplayMode, SplitRatio, SplitWithInfoPanel};
use ui_framework::panels::info_panel::InfoPanel;

mod mock_data;
mod navigation;
mod views;

use navigation::get_demo_navigation_items;
use views::{DashboardView, FlagshipShowcaseView, GalleryView, OperatorConsoleView, OrderDetailView, OrdersView, SettingsView};

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    GalleryView {},

    #[route("/demo/combined/enterprise-showcase")]
    FlagshipShowcaseView {},

    #[route("/demo/realtime/operator-console")]
    OperatorConsoleDemo {},

    #[route("/demo/navigation/:variant")]
    NavigationDemo { variant: String },

    #[route("/demo/layouts/:template")]
    LayoutDemo { template: String },

    #[route("/demo/feedback/:topic")]
    FeedbackDemo { topic: String },

    #[route("/demo/services/:topic")]
    ServicesDemo { topic: String },
}

#[component]
fn OperatorConsoleDemo() -> Element {
    rsx! {
        AppShell {
            navigation_items: get_demo_navigation_items(),
            OperatorConsoleView {}
        }
    }
}

#[component]
fn NavigationDemo(variant: String) -> Element {
    rsx! {
        AppShell {
            navigation_items: get_demo_navigation_items(),
            DashboardView {}
        }
    }
}

#[component]
fn LayoutDemo(template: String) -> Element {
    let tpl = template.as_str();
    rsx! {
        AppShell {
            navigation_items: get_demo_navigation_items(),
            if tpl == "split-info-panel" {
                SplitWithInfoPanel {
                    info_panel: rsx! { InfoPanel { title: "Properties".to_string() } },
                    default_ratio: SplitRatio::TwoThirds,
                    default_mode: SplitDisplayMode::Docked,
                    OrdersView {}
                }
            } else if tpl == "object-page" {
                OrderDetailView {}
            } else if tpl == "settings-layout" {
                SettingsView {}
            } else {
                DashboardView {}
            }
        }
    }
}

#[component]
fn FeedbackDemo(topic: String) -> Element {
    rsx! {
        AppShell {
            navigation_items: get_demo_navigation_items(),
            DashboardView {}
        }
    }
}

#[component]
fn ServicesDemo(topic: String) -> Element {
    rsx! {
        AppShell {
            navigation_items: get_demo_navigation_items(),
            DashboardView {}
        }
    }
}

#[allow(non_snake_case)]
fn App() -> Element {
    rsx! {
        LayoutSettingsProvider {
            I18nProvider {
                SessionProvider {
                    OverlayProvider {
                        Router::<Route> {}
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
