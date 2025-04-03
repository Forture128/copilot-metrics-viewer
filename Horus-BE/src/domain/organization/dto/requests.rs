use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateOrganizationRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateOrganizationRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateOrganizationConfigRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Key must be between 1 and 255 characters"
    ))]
    pub config_key: String,

    #[validate(length(min = 1, message = "Value cannot be empty"))]
    pub config_value: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateOrganizationConfigRequest {
    #[validate(length(min = 1, message = "Value cannot be empty"))]
    pub config_value: String,
}
