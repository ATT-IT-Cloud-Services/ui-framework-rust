use std::sync::Arc;
use dioxus::prelude::*;
use super::types::{NoopTelemetryAdapter, TelemetryAdapter};

#[derive(Clone)]
pub struct SharedTelemetry(pub Arc<dyn TelemetryAdapter>);

impl PartialEq for SharedTelemetry {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

pub fn use_telemetry() -> SharedTelemetry {
    if let Some(telemetry) = try_consume_context::<SharedTelemetry>() {
        telemetry
    } else {
        SharedTelemetry(Arc::new(NoopTelemetryAdapter))
    }
}

#[component]
pub fn TelemetryProvider(
    adapter: SharedTelemetry,
    children: Element,
) -> Element {
    use_context_provider(|| adapter);
    rsx! { {children} }
}
