use dioxus::prelude::*;
use crate::ui::dialog::{Dialog, DialogDescription, DialogFooter, DialogHeader, DialogTitle};
use crate::ui::button::{Button, ButtonVariant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WizardStepInfo {
    pub id: String,
    pub title: String,
}

#[component]
pub fn WizardDialog(
    open: bool,
    title: String,
    #[props(default = None)] subtitle: Option<String>,
    steps: Vec<WizardStepInfo>,
    current_step: usize,
    #[props(default = "Finish".to_string())] finish_label: String,
    onstepchange: EventHandler<usize>,
    onfinish: EventHandler<()>,
    onclose: EventHandler<()>,
    children: Element,
) -> Element {
    let total_steps = steps.len();
    let is_first = current_step == 0;
    let is_last = current_step + 1 >= total_steps;

    rsx! {
        Dialog {
            open: open,
            onclose: move |_| onclose.call(()),
            DialogHeader {
                DialogTitle { "{title}" }
                if let Some(sub) = subtitle {
                    DialogDescription { "{sub}" }
                }

                // Step Pills
                div { class: "grid grid-cols-3 gap-2 pt-3",
                    for (idx, step) in steps.iter().enumerate() {
                        {
                            let is_current = idx == current_step;
                            let is_passed = idx < current_step;
                            rsx! {
                                div {
                                    key: "{step.id}",
                                    class: if is_current {
                                        "p-2 rounded-md border border-primary bg-primary/10 text-primary font-semibold text-xs flex items-center gap-2"
                                    } else if is_passed {
                                        "p-2 rounded-md border border-border bg-background text-foreground text-xs flex items-center gap-2 cursor-pointer"
                                    } else {
                                        "p-2 rounded-md border border-transparent text-muted-foreground/60 text-xs flex items-center gap-2"
                                    },
                                    onclick: move |_| {
                                        if is_passed {
                                            onstepchange.call(idx);
                                        }
                                    },
                                    span { class: "flex h-5 w-5 items-center justify-center rounded-full text-2xs font-bold bg-muted",
                                        if is_passed { "✓" } else { "{idx + 1}" }
                                    }
                                    span { class: "truncate text-2xs", "{step.title}" }
                                }
                            }
                        }
                    }
                }
            }

            // Step Content
            div { class: "py-4 max-h-[60vh] overflow-y-auto",
                {children}
            }

            // Footer
            DialogFooter {
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| onclose.call(()),
                    "Cancel"
                }

                if !is_first {
                    Button {
                        variant: ButtonVariant::Outline,
                        onclick: move |_| onstepchange.call(current_step - 1),
                        "Back"
                    }
                }

                if is_last {
                    Button {
                        variant: ButtonVariant::Default,
                        onclick: move |_| onfinish.call(()),
                        "{finish_label}"
                    }
                } else {
                    Button {
                        variant: ButtonVariant::Default,
                        onclick: move |_| onstepchange.call(current_step + 1),
                        "Next ▶"
                    }
                }
            }
        }
    }
}
