use ui_framework::realtime::alarm_list::{AlarmItem, AlarmSeverity};

pub fn get_mock_alarms() -> Vec<AlarmItem> {
    vec![
        AlarmItem {
            id: "alm-1".to_string(),
            source: "Turbine-04 (Rotor Bearing)".to_string(),
            message: "High temperature alert: 98.4°C exceeding safety envelope.".to_string(),
            severity: AlarmSeverity::Critical,
            timestamp: "12m ago".to_string(),
            acknowledged: false,
        },
        AlarmItem {
            id: "alm-2".to_string(),
            source: "Cooling Circuit B".to_string(),
            message: "Inlet pressure drop to 2.1 bar (nominal 3.5 bar).".to_string(),
            severity: AlarmSeverity::High,
            timestamp: "24m ago".to_string(),
            acknowledged: false,
        },
        AlarmItem {
            id: "alm-3".to_string(),
            source: "Generator Substation 2".to_string(),
            message: "Harmonic distortion detected on Phase L3 (4.2% THD).".to_string(),
            severity: AlarmSeverity::Medium,
            timestamp: "1h ago".to_string(),
            acknowledged: true,
        },
    ]
}
