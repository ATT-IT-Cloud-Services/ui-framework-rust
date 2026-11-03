use dioxus::prelude::*;
use crate::ui::dialog::{Dialog, DialogDescription, DialogFooter, DialogHeader, DialogTitle};
use crate::ui::button::{Button, ButtonVariant};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ConfirmOptions {
    pub title: String,
    pub description: String,
    pub confirm_text: String,
    pub cancel_text: String,
    pub is_destructive: bool,
}

#[derive(Clone, Copy)]
pub struct ConfirmContext {
    options: Signal<Option<ConfirmOptions>>,
    callback: Signal<Option<Callback<bool>>>,
}

pub fn use_confirm() -> impl Fn(ConfirmOptions, Callback<bool>) {
    let ctx = use_context::<ConfirmContext>();
    move |opts: ConfirmOptions, cb: Callback<bool>| {
        let mut options_signal = ctx.options;
        let mut callback_signal = ctx.callback;
        options_signal.set(Some(opts));
        callback_signal.set(Some(cb));
    }
}

#[component]
pub fn ConfirmProvider(children: Element) -> Element {
    let mut options = use_signal(|| None::<ConfirmOptions>);
    let callback = use_signal(|| None::<Callback<bool>>);

    use_context_provider(|| ConfirmContext { options, callback });

    rsx! {
        {children}
        if let Some(opts) = options() {
            Dialog {
                open: true,
                onclose: move |_| {
                    if let Some(cb) = callback() {
                        cb.call(false);
                    }
                    options.set(None);
                },
                DialogHeader {
                    DialogTitle { "{opts.title}" }
                    DialogDescription { "{opts.description}" }
                }
                DialogFooter {
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| {
                            if let Some(cb) = callback() {
                                cb.call(false);
                            }
                            options.set(None);
                        },
                        if opts.cancel_text.is_empty() { "Cancel" } else { "{opts.cancel_text}" }
                    }
                    Button {
                        variant: if opts.is_destructive { ButtonVariant::Destructive } else { ButtonVariant::Default },
                        onclick: move |_| {
                            if let Some(cb) = callback() {
                                cb.call(true);
                            }
                            options.set(None);
                        },
                        if opts.confirm_text.is_empty() { "Confirm" } else { "{opts.confirm_text}" }
                    }
                }
            }
        }
    }
}
