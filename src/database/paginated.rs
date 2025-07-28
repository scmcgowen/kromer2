#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaginatedResult<T> {
    pub count: i64,
    pub total: i64,
    pub rows: Vec<T>,
}

impl<T> PaginatedResult<T> {
    // Create a new paginated result
    pub fn new(rows: Vec<T>, total: i64) -> Self {
        Self {
            count: rows.len() as i64,
            total,
            rows,
        }
    }

    // Create an empty paginated result
    pub fn empty() -> Self {
        Self {
            count: 0,
            total: 0,
            rows: Vec::new(),
        }
    }

    // Check if this page has more results available
    pub fn has_more(&self, offset: i64) -> bool {
        offset + self.count < self.total
    }

    // Get the next offset for pagination
    pub fn next_offset(&self, current_offset: i64) -> Option<i64> {
        if self.has_more(current_offset) {
            Some(current_offset + self.count)
        } else {
            None
        }
    }

    // Get the previous offset for pagination
    pub fn prev_offset(&self, current_offset: i64, limit: i64) -> Option<i64> {
        if current_offset > 0 {
            Some((current_offset - limit).max(0))
        } else {
            None
        }
    }

    // Transform the data items using a closure while preserving pagination metadata
    pub fn map<U, F>(self, f: F) -> PaginatedResult<U>
    where
        F: FnMut(T) -> U,
    {
        PaginatedResult {
            count: self.count,
            total: self.total,
            rows: self.rows.into_iter().map(f).collect(),
        }
    }

    // Filter the data items, update count, but not total
    pub fn filter<F>(self, mut f: F) -> Self
    where
        F: FnMut(&T) -> bool,
    {
        let filtered_rows: Vec<T> = self.rows.into_iter().filter(|item| f(item)).collect();
        let new_count = filtered_rows.len() as i64;

        Self {
            count: new_count,
            total: self.total, // Keep original
            rows: filtered_rows,
        }
    }

    // Get pagination info
    pub fn pagination_info(&self, offset: i64, limit: i64) -> PaginationInfo {
        let current_page = (offset / limit) + 1;
        let total_pages = ((self.total as f64) / (limit as f64)).ceil() as i64;

        PaginationInfo {
            current_page,
            total_pages,
            has_next: self.has_more(offset),
            has_prev: offset > 0,
            offset,
            limit,
        }
    }
}

// Pagination metadata for API responses
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaginationInfo {
    pub current_page: i64,
    pub total_pages: i64,
    pub has_next: bool,
    pub has_prev: bool,
    pub offset: i64,
    pub limit: i64,
}
