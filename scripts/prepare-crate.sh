#!/usr/bin/env bash
set -e

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET_DIR="${ROOT_DIR}/packages/ui-framework-rust-crate"

echo "Preparing standalone crate repository in: ${TARGET_DIR}"

rm -rf "${TARGET_DIR}"
mkdir -p "${TARGET_DIR}/src"

# Copy clean framework code (NO demo files)
cp -r "${ROOT_DIR}/crates/ui-framework/src/"* "${TARGET_DIR}/src/"
cp "${ROOT_DIR}/LICENSE.md" "${TARGET_DIR}/LICENSE.md"

# Generate standalone Cargo.toml
cat << 'EOF' > "${TARGET_DIR}/Cargo.toml"
[package]
name = "ui-framework-rust-crate"
version = "0.1.0"
edition = "2021"
authors = ["ATT IT / Philippe Simon Pflug <service@att-it.com>"]
description = "Host-agnostic enterprise UI framework for Dioxus 0.7 & Rust"
license-file = "LICENSE.md"
repository = "https://github.com/ATT-IT-Cloud-Services/ui-framework-rust"
homepage = "https://github.com/ATT-IT-Cloud-Services/ui-framework-rust#readme"
readme = "README.md"
keywords = ["dioxus", "ui", "enterprise", "scada", "components"]
categories = ["gui", "web-programming"]

[dependencies]
dioxus = { version = "0.7.0" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
EOF

# Copy README
cat << 'EOF' > "${TARGET_DIR}/README.md"
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
EOF

mkdir -p "${TARGET_DIR}/.github/workflows"
cat << 'EOF' > "${TARGET_DIR}/.github/workflows/ci.yml"
name: Rust Crate CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  build-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo check --verbose
      - run: cargo test --verbose
EOF

echo "Standalone crate prepared successfully."
