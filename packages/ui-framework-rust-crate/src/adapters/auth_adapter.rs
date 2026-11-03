use dioxus::prelude::*;
use super::types::AuthAdapter;

pub fn use_session() -> Signal<AuthAdapter> {
    if let Some(session) = try_consume_context::<Signal<AuthAdapter>>() {
        session
    } else {
        use_signal(AuthAdapter::default)
    }
}

#[component]
pub fn SessionProvider(
    #[props(default)] adapter: AuthAdapter,
    children: Element,
) -> Element {
    let session = use_signal(|| adapter);
    use_context_provider(|| session);

    rsx! {
        {children}
    }
}
