use dioxus::prelude::*;

#[component]
pub fn ConfirmHold(
    label: String,
    #[props(default = 1500)] hold_duration_ms: u64,
    #[props(default = "")] class: &'static str,
    onconfirm: EventHandler<()>,
) -> Element {
    let mut holding = use_signal(|| false);
    let mut progress = use_signal(|| 0.0f64);
    let mut confirmed = use_signal(|| false);

    rsx! {
        button {
            r#type: "button",
            class: "relative overflow-hidden rounded-lg font-semibold select-none shadow-2xs h-11 px-5 text-xs bg-destructive text-destructive-foreground hover:bg-destructive/90 flex items-center justify-center gap-2 cursor-pointer {class}",
            onmousedown: move |_| {
                if !confirmed() {
                    holding.set(true);
                    progress.set(100.0);
                }
            },
            onmouseup: move |_| {
                if !confirmed() && holding() {
                    holding.set(false);
                    confirmed.set(true);
                    onconfirm.call(());
                }
            },
            onmouseleave: move |_| {
                holding.set(false);
                progress.set(0.0);
            },
            span {
                class: "absolute inset-y-0 start-0 bg-black/25 pointer-events-none transition-all duration-[1500ms]",
                style: "width: {progress()}%;"
            }
            span { class: "relative z-10",
                if confirmed() {
                    "✓ Executed"
                } else if holding() {
                    "Hold to confirm..."
                } else {
                    "{label}"
                }
            }
        }
    }
}
