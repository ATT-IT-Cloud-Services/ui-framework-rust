use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::ui::badge::{Badge, BadgeVariant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EntityStatus {
    #[default]
    Active,
    Inactive,
    Pending,
    Completed,
    Error,
    Warning,
    Draft,
    Offline,
}

impl EntityStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Inactive => "Inactive",
            Self::Pending => "Pending",
            Self::Completed => "Completed",
            Self::Error => "Error",
            Self::Warning => "Warning",
            Self::Draft => "Draft",
            Self::Offline => "Offline",
        }
    }

    pub fn variant(&self) -> BadgeVariant {
        match self {
            Self::Active | Self::Completed => BadgeVariant::Success,
            Self::Pending | Self::Warning => BadgeVariant::Warning,
            Self::Error => BadgeVariant::Destructive,
            Self::Draft => BadgeVariant::Outline,
            Self::Inactive | Self::Offline => BadgeVariant::Muted,
        }
    }
}

#[component]
pub fn StatusBadge(
    status: EntityStatus,
    #[props(default = "")] class: &'static str,
) -> Element {
    rsx! {
        Badge {
            variant: status.variant(),
            class: class,
            "{status.as_str()}"
        }
    }
}
