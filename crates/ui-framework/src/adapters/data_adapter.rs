use super::types::{DataAdapter, DataQuery, DataResult};

pub struct InMemoryDataAdapter<T> {
    pub items: Vec<T>,
}

impl<T: Clone + Send + Sync> DataAdapter<T> for InMemoryDataAdapter<T> {
    fn load(&self, query: &DataQuery) -> DataResult<T> {
        let total = self.items.len();
        let start = query.page_index * query.page_size;
        let end = (start + query.page_size).min(total);

        let data = if start < total {
            self.items[start..end].to_vec()
        } else {
            Vec::new()
        };

        DataResult {
            data,
            total_count: total,
            page_count: (total + query.page_size - 1) / query.page_size.max(1),
        }
    }
}
