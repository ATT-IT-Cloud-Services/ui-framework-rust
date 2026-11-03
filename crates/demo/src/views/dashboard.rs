use dioxus::prelude::*;
use ui_framework::shell::PageHeader;
use ui_framework::panels::stat_card::{StatCard, StatTrend};
use ui_framework::realtime::{LiveBadge, LiveValue, AlarmList, Sparkline};
use ui_framework::ui::card::{Card, CardHeader, CardTitle, CardContent};
use crate::mock_data::alarms::get_mock_alarms;

#[component]
pub fn DashboardView() -> Element {
    let mut alarms = use_signal(get_mock_alarms);
    let mut grid_output = use_signal(|| 482.6f64);
    let mut rpm = use_signal(|| 1498.4f64);
    let mut bearing_temp = use_signal(|| 98.4f64);

    rsx! {
        div { class: "space-y-6",
            PageHeader {
                title: "Plant Operations & Fleet Telemetry".to_string(),
                subtitle: Some("Real-time telemetry, mission-critical alarms, and automated dispatch pipeline.".to_string()),
                badge: rsx! { LiveBadge { is_live: true } },
            }

            // Stat Cards Grid
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4",
                StatCard {
                    title: "Active Dispatch Volume".to_string(),
                    value: "€142,850".to_string(),
                    subtitle: Some("Orders in fulfillment".to_string()),
                    trend: Some(StatTrend { value: "+12.4%".to_string(), is_up: true }),
                    Sparkline {
                        data: vec![45.0, 52.0, 48.0, 62.0, 59.0, 74.0, 82.0, 88.0],
                        color: "#16a34a",
                        width: 140,
                        height: 20,
                    }
                }

                StatCard {
                    title: "Turbine Grid Output".to_string(),
                    value: format!("{:.1} MW", grid_output()),
                    subtitle: Some("98.2% operating efficiency".to_string()),
                    trend: Some(StatTrend { value: "+3.1%".to_string(), is_up: true }),
                    Sparkline {
                        data: vec![420.0, 440.0, 460.0, 455.0, 470.0, grid_output()],
                        color: "#0064d9",
                        width: 140,
                        height: 20,
                    }
                }

                StatCard {
                    title: "Sensor Health".to_string(),
                    value: "99.94%".to_string(),
                    subtitle: Some("1,420 sensors online".to_string()),
                    trend: Some(StatTrend { value: "-0.02%".to_string(), is_up: false }),
                }

                StatCard {
                    title: "Active Alarms".to_string(),
                    value: format!("{}", alarms().iter().filter(|a| !a.acknowledged).count()),
                    subtitle: Some("Requiring review".to_string()),
                    trend: Some(StatTrend { value: "1 critical".to_string(), is_up: false }),
                }
            }

            // Live Diagnostics Card
            Card {
                CardHeader {
                    CardTitle { "Live Turbine Cluster Diagnostics" }
                }
                CardContent {
                    div { class: "grid grid-cols-2 sm:grid-cols-4 gap-6",
                        LiveValue { label: Some("Rotor Velocity".to_string()), value: format!("{:.1}", rpm()), unit: "RPM" }
                        LiveValue { label: Some("Bearing Temp".to_string()), value: format!("{:.1}", bearing_temp()), unit: "°C" }
                        LiveValue { label: Some("Lubrication".to_string()), value: "4.2".to_string(), unit: "bar" }
                        LiveValue { label: Some("Vibration Harmonic".to_string()), value: "3.8".to_string(), unit: "mm/s" }
                    }
                }
            }

            // Alarms List Card
            Card {
                CardHeader {
                    CardTitle { "Active SCADA Alarms" }
                }
                CardContent {
                    AlarmList {
                        alarms: alarms(),
                        onacknowledge: move |id: String| {
                            let mut list = alarms();
                            if let Some(entry) = list.iter_mut().find(|a| a.id == id) {
                                entry.acknowledged = true;
                            }
                            alarms.set(list);
                        }
                    }
                }
            }
        }
    }
}
