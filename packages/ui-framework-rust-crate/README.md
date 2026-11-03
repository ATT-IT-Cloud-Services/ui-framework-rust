# ui-framework-rust-crate

> Standalone, host-agnostic enterprise UI framework for Dioxus 0.7 & Rust.

## Features
- **100% Rust & Dioxus 0.7**: Zero JavaScript dependencies, built on Rust/UI principles.
- **Enterprise Layouts**: SinglePane, MasterDetail, ThreePane, SplitWithInfoPanel (Docked, Overlap 1/3, Overlap 1/2 with left blur), ObjectPage, SettingsLayout (equal-height cards).
- **Navigation Modes**: Full Sidebar, Icon Rail (with Pin/Dauerhaft dock), Hybrid (toggleable), Dual Rail, Inset Card, Topnav, Bottom Nav.
- **Realtime SCADA Primitives**: LiveValue with quality indicators, LiveBadge, AlarmList, AlarmBanner, ConfirmHold (safety breaker), ShiftModeToggle, Sparkline.
- **Internationalization**: 5 built-in locales (en, de, fr, es, ar) with 100% key parity and instant RTL mirroring.
- **Zero-Auth Architecture**: Pure consumer of generic `AuthAdapter` and permissions.

## Usage in Cargo.toml
```toml
[dependencies]
ui-framework-rust-crate = { git = "https://github.com/ATT-IT-Cloud-Services/ui-framework-rust-crate.git" }
dioxus = "0.7"
```

## License
AS-IS Software License Agreement © ATT IT / Philippe Simon Pflug. See `LICENSE.md`.
