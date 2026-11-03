# Configurable Enterprise UI Framework in Rust (`ui-framework-rust`)

> High-performance, host-agnostic enterprise UI framework built with Rust, Dioxus 0.7, and Tailwind CSS. Direct 1:1 Rust implementation of the TypeScript enterprise framework.

---

## 1. Design & Architecture

- **100% Rust & Dioxus 0.7**: Zero JavaScript dependencies, cross-platform (Web WASM, Desktop WebView, Mobile).
- **Modernized Enterprise Aesthetics**: Calm, professional, dense-but-airy layout with subtle 1px borders instead of heavy drop shadows.
- **Micro-radii**: 4px (`sm`), 6px (`md`, default), and 8px (`lg`) corner radii for structured industrial ergonomics.
- **Configurable Brand Palettes**: Blue, Teal, Violet, Green, Orange, plus custom hex overrides.
- **Strict Framework Separation**:
  - `crates/ui-framework`: Pure framework crate without any demo or mock data.
  - `crates/demo`: Full interactive demo gallery.
  - `packages/ui-framework-rust-crate`: Standalone package crate for distribution.
- **Zero-Auth Boundary**: The framework consumes only generic `AuthAdapter` types and permissions. No authentication libraries or hardcoded user assumptions in the framework.

---

## 2. Feature Parity Matrix with TypeScript

| Capability | Rust / Dioxus 0.7 Implementation |
| --- | --- |
| **Navigation Modes** | 7 Presentation Variants: `SidebarFull`, `SidebarRail` (with Pin/Dock), `SidebarHybrid`, `SidebarDual`, `SidebarInset`, `Topnav`, `BottomNav` (full text, no truncation). |
| **Split Layouts** | `SplitWithInfoPanel` with **Docked**, **Overlap 1/3**, and **Overlap 1/2** with left backdrop blur and display-height binding! Plus `ThreePane`, `FullSplit`, `MasterDetail`. |
| **Entity Overview** | `ObjectPage` with header, key attributes bar, sticky tabs, and sticky action bar. |
| **Settings Layout** | `SettingsLayout` with equal-height cards (`items-stretch min-h-[580px]`) and static sub-nav. |
| **Form Dialogs & Wizards** | `WizardDialog` (3-step wizard with line items, sliders, live price calculation). `DestructiveConfirmDialog` with typed confirmation. |
| **Realtime Primitives** | `LiveValue` with quality indicators, `LiveBadge`, `AlarmList`, `AlarmBanner`, `ConfirmHold` (1500ms safety breaker), `ShiftModeToggle`, `Sparkline`. |
| **Internationalization** | 5 Locales (`en`, `de`, `fr`, `es`, `ar`) with 100% key parity and instant RTL text direction mirroring. |
| **Component Customizer** | `use_component_style` supporting global and per-`customId` overrides. |

---

## 3. Quick Start

### Build & Test Workspace
```bash
cargo check --workspace
cargo test --workspace
```

### Run Demo Gallery
```bash
cargo run -p demo
```

---

## 4. Usage as Crate in another Rust / Dioxus Project

Add to your `Cargo.toml`:
```toml
[dependencies]
ui-framework-rust-crate = { git = "https://github.com/ATT-IT-Cloud-Services/ui-framework-rust-crate.git" }
dioxus = { version = "0.7", features = ["router"] }
```

### Application Root Setup
```rust
use dioxus::prelude::*;
use ui_framework_rust_crate::{AppShell, LayoutSettingsProvider, I18nProvider, SessionProvider, OverlayProvider};

fn App() -> Element {
    rsx! {
        LayoutSettingsProvider {
            I18nProvider {
                SessionProvider {
                    OverlayProvider {
                        AppShell {
                            navigation_items: vec![
                                // your navigation items
                            ],
                            // your pages
                        }
                    }
                }
            }
        }
    }
}
```

---

## 5. Repositories

- **Monorepo (Framework + Demo Gallery)**: [https://github.com/ATT-IT-Cloud-Services/ui-framework-rust](https://github.com/ATT-IT-Cloud-Services/ui-framework-rust)
- **Standalone Crate Repository**: [https://github.com/ATT-IT-Cloud-Services/ui-framework-rust-crate](https://github.com/ATT-IT-Cloud-Services/ui-framework-rust-crate)

---

## 6. License

AS-IS Software License Agreement © ATT IT / Philippe Simon Pflug. See `LICENSE.md`.
