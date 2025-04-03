// Common domain-level types used across modules

/// Pagination parameters for repository queries
#[derive(Debug, Clone)]
pub struct PaginationParams {
    /// The page number, starting from 1
    pub page: i64,
    /// The number of items per page
    pub page_size: i64,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
        }
    }
}

/// Common status enum for various entities
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Active,
    Inactive,
    Pending,
    Archived,
}

/// Common sort direction for queries
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Date range filter
#[derive(Debug, Clone)]
pub struct DateRange {
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
}
