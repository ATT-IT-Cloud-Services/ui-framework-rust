use dioxus::prelude::*;
use ui_framework::shell::PageHeader;
use ui_framework::realtime::{
    AlarmBanner, AlarmList, ConfirmHold, LiveBadge, LiveValue, ShiftModeToggle, Sparkline,
    StatusIndicator, SystemStatus, TimeRangePicker,
};
use ui_framework::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::mock_data::alarms::get_mock_alarms;

#[component]
pub fn OperatorConsoleView() -> Element {
    let mut alarms = use_signal(get_mock_alarms);
    let mut critical_banner_open = use_signal(|| true);
    let rpm = 1498.4;
    let temp = 98.4;

    rsx! {
        div { class: "space-y-4",
            if critical_banner_open() {
                AlarmBanner {
                    source: "Turbine-04".to_string(),
                    message: "Rotor bearing temperature critical (98.4°C).".to_string(),
                    onacknowledge: move |_| critical_banner_open.set(false),
                }
            }

            PageHeader {
                title: "Turbine Cluster 04 — Mission Operations Console".to_string(),
                subtitle: Some("Live SCADA telemetry gateway streaming at 50 Hz from Siemens ST-60 units.".to_string()),
                badge: rsx! { LiveBadge { is_live: true } },
                actions: rsx! {
                    div { class: "flex items-center gap-2 flex-wrap",
                        TimeRangePicker {}
                        ShiftModeToggle {}
                    }
                }
            }

            // Quick Status
            div { class: "grid grid-cols-2 sm:grid-cols-4 gap-3 p-3 rounded-xl border border-border bg-card text-xs",
                StatusIndicator { status: SystemStatus::Ok, label: "Core: OK" }
                StatusIndicator { status: SystemStatus::Warn, label: "Bearing: Warn" }
                StatusIndicator { status: SystemStatus::Ok, label: "Inlet: OK" }
                StatusIndicator { status: SystemStatus::Ok, label: "Grid: Synced" }
            }

            // Instrument Cards
            div { class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4",
                Card {
                    CardHeader { CardTitle { "Rotor Velocity" } }
                    CardContent {
                        LiveValue { value: format!("{:.1}", rpm), unit: "RPM" }
                    }
                }
                Card {
                    CardHeader { CardTitle { "Bearing Thermal" } }
                    CardContent {
                        LiveValue { value: format!("{:.1}", temp), unit: "°C" }
                        div { class: "pt-2",
                            Sparkline {
                                data: vec![96.0, 96.5, 97.0, 97.8, 98.4],
                                color: "#ea580c",
                                width: 140,
                                height: 20,
                            }
                        }
                    }
                }
                Card {
                    CardHeader { CardTitle { "Hydraulic Pressure" } }
                    CardContent {
                        LiveValue { value: "4.2".to_string(), unit: "bar" }
                    }
                }
                Card {
                    CardHeader { CardTitle { "Safety Interlock" } }
                    CardContent {
                        ConfirmHold {
                            label: "Hold to Trip Breaker".to_string(),
                            onconfirm: move |_| {},
                        }
                    }
                }
            }

            // Alarms
            Card {
                CardHeader { CardTitle { "Active Alarms" } }
                CardContent {
                    AlarmList {
                        alarms: alarms(),
                        onacknowledge: move |id: String| {
                            let mut list = alarms();
                            if let Some(a) = list.iter_mut().find(|entry| entry.id == id) {
                                a.acknowledged = true;
                            }
                            alarms.set(list);
                        }
                    }
                }
            }
        }
    }
}
