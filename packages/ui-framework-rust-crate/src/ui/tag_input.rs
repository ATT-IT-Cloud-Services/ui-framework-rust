use dioxus::prelude::*;

#[component]
pub fn TagInput(
    tags: Vec<String>,
    #[props(default = 8)] max_tags: usize,
    #[props(default = "Type and press enter...")] placeholder: &'static str,
    #[props(default = "")] class: &'static str,
    onchange: EventHandler<Vec<String>>,
) -> Element {
    let mut input_text = use_signal(String::new);

    rsx! {
        div {
            class: "flex flex-wrap items-center gap-1.5 p-1.5 min-h-[36px] w-full rounded-md border border-input bg-background text-xs shadow-2xs {class}",
            for (idx, tag) in tags.iter().enumerate() {
                {
                    let tag_clone = tag.clone();
                    let current_tags = tags.clone();
                    rsx! {
                        div {
                            key: "{idx}",
                            class: "inline-flex items-center gap-1 px-2 py-0.5 rounded bg-secondary text-secondary-foreground text-2xs font-medium",
                            span { "{tag_clone}" }
                            button {
                                r#type: "button",
                                class: "text-muted-foreground hover:text-foreground cursor-pointer",
                                onclick: move |_| {
                                    let mut next = current_tags.clone();
                                    next.retain(|t| t != &tag_clone);
                                    onchange.call(next);
                                },
                                "✕"
                            }
                        }
                    }
                }
            }

            if tags.len() < max_tags {
                input {
                    r#type: "text",
                    value: "{input_text()}",
                    placeholder: if tags.is_empty() { placeholder } else { "" },
                    class: "flex-1 bg-transparent px-1 text-xs outline-none min-w-[120px]",
                    oninput: move |e| input_text.set(e.value()),
                    onkeydown: move |e: KeyboardEvent| {
                        if e.key() == Key::Enter {
                            let trimmed = input_text().trim().to_string();
                            if !trimmed.is_empty() && !tags.contains(&trimmed) && tags.len() < max_tags {
                                let mut next = tags.clone();
                                next.push(trimmed);
                                onchange.call(next);
                                input_text.set("".to_string());
                            }
                        }
                    }
                }
            }
        }
    }
}
