use dioxus::prelude::*;

pub fn use_query_state<T: Clone + ToString + std::str::FromStr + 'static>(
    _key: &'static str,
    default_val: T,
) -> Signal<T> {
    // Reactive signal that holds the state
    use_signal(|| default_val)
}
