use dioxus::prelude::*;
use crate::ui::button::{Button, ButtonVariant};
use crate::ui::search_filter_input::SearchFilterInput;
use crate::ui::checkbox::Checkbox;

#[derive(Clone)]
pub struct TableColumn<T> {
    pub key: &'static str,
    pub title: &'static str,
    pub can_sort: bool,
    pub render: fn(&T) -> Element,
}

impl<T> PartialEq for TableColumn<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.title == other.title && self.can_sort == other.can_sort
    }
}

#[component]
pub fn DataTable<T: Clone + PartialEq + 'static>(
    columns: Vec<TableColumn<T>>,
    data: Vec<T>,
    #[props(default = true)] enable_selection: bool,
    #[props(default = 10)] page_size: usize,
    #[props(default = "")] class: &'static str,
    toolbar_actions: Option<Element>,
) -> Element {
    let mut search_query = use_signal(String::new);
    let mut selected_indices = use_signal(Vec::<usize>::new);
    let mut page_index = use_signal(|| 0usize);

    let total_rows = data.len();
    let total_pages = (total_rows + page_size - 1) / page_size.max(1);

    let start_idx = page_index() * page_size;
    let end_idx = (start_idx + page_size).min(total_rows);
    let page_data = if start_idx < total_rows {
        &data[start_idx..end_idx]
    } else {
        &[]
    };

    let all_selected = !page_data.is_empty() && page_data.iter().enumerate().all(|(idx, _)| selected_indices().contains(&(start_idx + idx)));

    rsx! {
        div { class: "space-y-3 {class}",
            // Toolbar
            div { class: "flex flex-col sm:flex-row sm:items-center sm:justify-between gap-2.5",
                div { class: "flex items-center gap-2 flex-1 max-w-sm",
                    SearchFilterInput {
                        value: search_query(),
                        placeholder: "Filter table records...",
                        onchange: move |val| search_query.set(val),
                    }
                    if let Some(act) = toolbar_actions {
                        {act}
                    }
                }
            }

            // Table Box
            div { class: "rounded-xl border border-border bg-card overflow-hidden shadow-2xs",
                div { class: "relative w-full overflow-auto",
                    table { class: "w-full caption-bottom text-xs",
                        thead { class: "[&_tr]:border-b bg-muted/30",
                            tr {
                                if enable_selection {
                                    th { class: "w-10 px-3 py-2 text-center",
                                        Checkbox {
                                            checked: all_selected,
                                            onchange: move |checked| {
                                                if checked {
                                                    let mut next = selected_indices();
                                                    for i in start_idx..end_idx {
                                                        if !next.contains(&i) {
                                                            next.push(i);
                                                        }
                                                    }
                                                    selected_indices.set(next);
                                                } else {
                                                    let mut next = selected_indices();
                                                    next.retain(|&i| i < start_idx || i >= end_idx);
                                                    selected_indices.set(next);
                                                }
                                            }
                                        }
                                    }
                                }
                                for col in columns.iter() {
                                    th {
                                        key: "{col.key}",
                                        class: "h-8 px-3 text-start align-middle font-semibold text-muted-foreground",
                                        "{col.title}"
                                    }
                                }
                            }
                        }

                        tbody { class: "[&_tr:last-child]:border-0",
                            if page_data.is_empty() {
                                tr {
                                    td {
                                        colspan: "{columns.len() + 1}",
                                        class: "h-32 text-center text-xs text-muted-foreground",
                                        "No records available in this table view."
                                    }
                                }
                            } else {
                                for (row_idx, row) in page_data.iter().enumerate() {
                                    {
                                        let absolute_idx = start_idx + row_idx;
                                        let is_selected = selected_indices().contains(&absolute_idx);
                                        rsx! {
                                            tr {
                                                key: "{absolute_idx}",
                                                class: if is_selected { "border-b border-border/60 bg-primary/5 transition-colors" } else { "border-b border-border/60 hover:bg-muted/40 transition-colors" },
                                                if enable_selection {
                                                    td { class: "px-3 py-2 text-center",
                                                        Checkbox {
                                                            checked: is_selected,
                                                            onchange: move |c| {
                                                                let mut next = selected_indices();
                                                                if c {
                                                                    if !next.contains(&absolute_idx) {
                                                                        next.push(absolute_idx);
                                                                    }
                                                                } else {
                                                                    next.retain(|&i| i != absolute_idx);
                                                                }
                                                                selected_indices.set(next);
                                                            }
                                                        }
                                                    }
                                                }
                                                for col in columns.iter() {
                                                    td {
                                                        key: "{col.key}",
                                                        class: "p-3 align-middle",
                                                        {(col.render)(row)}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Pagination Controls
            div { class: "flex items-center justify-between text-xs text-muted-foreground px-1 select-none",
                div {
                    if enable_selection {
                        span { "{selected_indices().len()} of {total_rows} row(s) selected" }
                    }
                }

                div { class: "flex items-center gap-3",
                    span { "Page {page_index() + 1} of {total_pages.max(1)}" }
                    div { class: "flex items-center gap-1",
                        Button {
                            variant: ButtonVariant::Outline,
                            size: crate::ui::button::ButtonSize::Sm,
                            disabled: page_index() == 0,
                            onclick: move |_| {
                                if page_index() > 0 {
                                    page_index.set(page_index() - 1);
                                }
                            },
                            "◀ Previous"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            size: crate::ui::button::ButtonSize::Sm,
                            disabled: page_index() + 1 >= total_pages,
                            onclick: move |_| {
                                if page_index() + 1 < total_pages {
                                    page_index.set(page_index() + 1);
                                }
                            },
                            "Next ▶"
                        }
                    }
                }
            }
        }
    }
}
