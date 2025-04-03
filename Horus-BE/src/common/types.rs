use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    10
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: u64, params: &PaginationParams) -> Self {
        let total_pages = ((total as f64) / (params.per_page as f64)).ceil() as i64;
        Self {
            items,
            total,
            page: params.page,
            per_page: params.per_page,
            total_pages,
        }
    }
}

/// JWT Claims structure used for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // User ID
    pub org: i32,     // Organization ID
    pub role: String, // Role (user, admin, super_admin)
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at time
}

impl Claims {
    /// Creates a new Claims instance
    pub fn new(user_id: i32, org_id: i32, role: &str, expires_in_seconds: u64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        Self {
            sub: user_id.to_string(),
            org: org_id,
            role: role.to_string(),
            iat: now,
            exp: now + expires_in_seconds as usize,
        }
    }

    /// Checks if the claims have expired
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        self.exp < now
    }
}
