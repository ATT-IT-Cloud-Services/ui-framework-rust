use dioxus::prelude::*;
use super::types::LayoutSettings;
use crate::config::types::ThemeMode;

#[derive(Clone, Copy)]
pub struct LayoutSettingsSignal(pub Signal<LayoutSettings>);

pub fn use_layout_settings() -> Signal<LayoutSettings> {
    use_context::<Signal<LayoutSettings>>()
}

pub fn use_is_dark() -> bool {
    let settings = use_layout_settings();
    let system_dark = try_use_context::<Signal<bool>>().map(|s| s()).unwrap_or(false);
    let theme = settings.read().theme;
    match theme {
        ThemeMode::Dark => true,
        ThemeMode::Light => false,
        ThemeMode::System => system_dark,
    }
}

#[component]
pub fn LayoutSettingsProvider(
    #[props(default)] initial: LayoutSettings,
    children: Element,
) -> Element {
    let settings = use_signal(|| initial);
    use_context_provider(|| settings);

    let mut system_dark = use_signal(|| false);
    use_context_provider(|| system_dark);

    // Active theme manager effect (tracks Dark, Light, and live OS prefers-color-scheme changes)
    use_effect(move || {
        let theme = settings.read().theme;
        let eval = document::eval(r#"
            const mql = window.matchMedia ? window.matchMedia('(prefers-color-scheme: dark)') : null;
            const isSystemDark = mql ? mql.matches : false;
            
            // Send initial system dark state to Rust
            dioxus.send(isSystemDark);

            // Listen for OS scheme changes if not already attached
            if (mql && !window.__ui_theme_mql_listening) {
                window.__ui_theme_mql_listening = true;
                mql.addEventListener('change', (e) => {
                    if (window.__ui_current_theme_mode === 'system') {
                        if (e.matches) {
                            document.documentElement.classList.add('dark');
                            document.documentElement.style.colorScheme = 'dark';
                        } else {
                            document.documentElement.classList.remove('dark');
                            document.documentElement.style.colorScheme = 'light';
                        }
                        let m = document.querySelector('meta[name=\"color-scheme\"]');
                        if (m) { m.content = e.matches ? 'dark' : 'light'; }
                    }
                    dioxus.send(e.matches);
                });
            }

            // Sync the current theme mode sent from Rust
            const mode = await dioxus.recv();
            window.__ui_current_theme_mode = mode;
            let dark;
            if (mode === 'dark') {
                dark = true;
            } else if (mode === 'light') {
                dark = false;
            } else {
                dark = isSystemDark;
            }

            if (dark) {
                document.documentElement.classList.add('dark');
                document.documentElement.style.colorScheme = 'dark';
            } else {
                document.documentElement.classList.remove('dark');
                document.documentElement.style.colorScheme = 'light';
            }

            let meta = document.querySelector('meta[name=\"color-scheme\"]');
            if (!meta) {
                meta = document.createElement('meta');
                meta.name = 'color-scheme';
                document.head.appendChild(meta);
            }
            meta.content = dark ? 'dark' : 'light';
        "#);

        let mode_str = match theme {
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
            ThemeMode::System => "system",
        };
        let _ = eval.send(mode_str.to_string());

        spawn(async move {
            let mut eval = eval;
            while let Ok(val) = eval.recv::<bool>().await {
                system_dark.set(val);
            }
        });
    });

    rsx! {
        {children}
    }
}
