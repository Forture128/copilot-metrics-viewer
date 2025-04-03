// API version prefix
pub const API_V1_PREFIX: &str = "/api/v1";

// Organization routes
pub mod organizations {
    use super::API_V1_PREFIX;
    pub const BASE: &str = "/organizations";

    pub fn path() -> String {
        format!("{}{}", API_V1_PREFIX, BASE)
    }

    pub fn with_id(id: i32) -> String {
        format!("{}{}/{}", API_V1_PREFIX, BASE, id)
    }

    // Nested resources
    pub mod roles {
        use super::*;
        pub const BASE: &str = "/roles";

        pub fn path(org_id: i32) -> String {
            format!("{}{}/{}{}", API_V1_PREFIX, super::BASE, org_id, BASE)
        }
    }
}

// Team routes
pub mod teams {
    use super::API_V1_PREFIX;
    pub const BASE: &str = "/teams";

    pub fn path() -> String {
        format!("{}{}", API_V1_PREFIX, BASE)
    }

    pub fn with_id(id: i32) -> String {
        format!("{}{}/{}", API_V1_PREFIX, BASE, id)
    }
}

// Department routes
pub mod departments {
    use super::API_V1_PREFIX;
    pub const BASE: &str = "/departments";

    pub fn path() -> String {
        format!("{}{}", API_V1_PREFIX, BASE)
    }

    pub fn with_id(id: i32) -> String {
        format!("{}{}/{}", API_V1_PREFIX, BASE, id)
    }
}

// Role routes
pub mod roles {
    use super::API_V1_PREFIX;
    pub const BASE: &str = "/roles";

    pub fn path() -> String {
        format!("{}{}", API_V1_PREFIX, BASE)
    }

    pub fn with_id(id: i32) -> String {
        format!("{}{}/{}", API_V1_PREFIX, BASE, id)
    }
}

// Add more domains as needed
