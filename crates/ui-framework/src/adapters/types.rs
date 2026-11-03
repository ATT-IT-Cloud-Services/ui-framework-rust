use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SessionStatus {
    Authenticated,
    Unauthenticated,
    Loading,
    #[default]
    Anonymous,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub tenant_id: Option<String>,
    pub tenant_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AuthAdapter {
    pub status: SessionStatus,
    pub user: Option<AuthUser>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DataQuery {
    pub search: Option<String>,
    pub page_index: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DataResult<T> {
    pub data: Vec<T>,
    pub total_count: usize,
    pub page_count: usize,
}

pub trait DataAdapter<T>: Send + Sync {
    fn load(&self, query: &DataQuery) -> DataResult<T>;
}

pub trait TelemetryAdapter: Send + Sync {
    fn track_page_view(&self, path: &str);
    fn track_event(&self, name: &str, properties: Option<&serde_json::Value>);
    fn track_error(&self, error: &str);
}

#[derive(Default)]
pub struct NoopTelemetryAdapter;

impl TelemetryAdapter for NoopTelemetryAdapter {
    fn track_page_view(&self, _path: &str) {}
    fn track_event(&self, _name: &str, _properties: Option<&serde_json::Value>) {}
    fn track_error(&self, _error: &str) {}
}
