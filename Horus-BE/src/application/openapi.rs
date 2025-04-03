use crate::application::controllers::{
    auth_controller::*, collaboration_controller::*, delivery_controller::*,
    department_controller::*, developer_controller::*, organization_controller::*,
    role_controller::*, team_controller::*, user_controller::*,
};
use crate::common::errors::ErrorResponse;
use crate::domain::auth::{AuthResponse, LoginRequest, UserInfoResponse};
use crate::domain::organization::{Organization, OrganizationConfig};
use crate::domain::{
    collaboration_quality::{entities::*, value_objects::*},
    delivery_insights::{entities::*, value_objects::*},
    department::{
        AssignUserToDepartmentRequest, BatchAssignUsersToDepartmentRequest,
        CreateDepartmentRequest, DepartmentListResponse, DepartmentResponse,
        UpdateDepartmentRequest,
    },
    developer_metrics::{entities::*, value_objects::*},
    organization::{
        CreateOrganizationConfigRequest, CreateOrganizationRequest, OrganizationConfigListResponse,
        OrganizationConfigResponse, OrganizationListResponse, OrganizationResponse,
        UpdateOrganizationConfigRequest, UpdateOrganizationRequest,
    },
    roles::{
        AssignDepartmentRoleRequest, CreateRoleRequest, DepartmentRole, Role, RoleListResponse,
        RoleResponse, UpdateRoleRequest,
    },
    teams::{
        AddTeamMemberRequest, CreateTeamRequest, Team, TeamListResponse, TeamMember, TeamResponse,
        TeamWithMembersResponse, UpdateTeamRequest,
    },
    user::{
        AssignRoleRequest, ChangePasswordRequest, CreateUserRequest, UpdateUserRequest,
        UserResponse, UserWithRolesResponse,
    },
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Organization endpoints
        create_organization,
        update_organization,
        delete_organization,
        get_organization,
        list_organizations,
        create_config,
        update_config,
        delete_config,
        get_config,
        list_configs,
        batch_create_organizations,
        batch_update_organizations,
        batch_delete_organizations,
        find_organizations_by_name,
        get_organizations_by_ids,
        organization_exists,
        batch_create_configs,
        batch_update_configs,
        batch_delete_configs,
        find_configs_by_key,
        get_configs_by_ids,

        // Department endpoints
        create_department,
        update_department,
        delete_department,
        get_department,
        list_departments,
        add_user_to_department,
        remove_user_from_department,
        get_department_users,
        batch_add_users_to_department,

        // Developer metrics endpoints
        get_developer_metrics,
        get_developer_commits,
        get_developer_pull_requests,

        // Collaboration quality endpoints
        get_collaboration_metrics,
        get_code_review_stats,
        get_team_interactions,

        // Delivery insights endpoints
        get_delivery_metrics,

        // Team endpoints
        create_team,
        update_team,
        delete_team,
        get_team,
        list_teams,
        add_team_member,
        remove_team_member,
        get_team_with_members,
        list_teams_by_department,

        // Role endpoints
        create_role,
        update_role,
        delete_role,
        get_role,
        list_roles,
        assign_department_role,

        // User controllers
        register_user,
        user_login,
        get_user,
        get_user_with_roles,
        update_user,
        change_password,
        list_users,
        assign_role,
        remove_role,

        // // Auth endpoints
        auth_login,
        auth_me,
    ),
    components(
        schemas(
            // Common
            ErrorResponse,
            // Organization
            Organization,
            OrganizationResponse,
            OrganizationListResponse,
            CreateOrganizationRequest,
            UpdateOrganizationRequest,
            OrganizationConfig,
            OrganizationConfigResponse,
            OrganizationConfigListResponse,
            CreateOrganizationConfigRequest,
            UpdateOrganizationConfigRequest,

            // Department
            DepartmentResponse,
            DepartmentListResponse,
            CreateDepartmentRequest,
            UpdateDepartmentRequest,
            AssignUserToDepartmentRequest,
            BatchAssignUsersToDepartmentRequest,
            DepartmentRole,
            DefaultRoleQuery,
            AssignDepartmentRoleRequest,

            // Developer metrics
            Developer,
            Commit,
            PullRequest,
            CodeReview,
            CommitFrequency,
            ReviewMetrics,
            CodeChangeMetrics,
            Metrics,
            Metrics,
            CommitFrequency,
            ReviewMetrics,
            CodeChangeMetrics,

            // Collaboration quality //
            CollaborationMetrics,
            TeamCollaborationMetrics,
            // Entities
            CodeReview,
            ReviewComment,
            ReviewStatus,
            CommentType,
            TeamInteraction,
            InteractionType,
            // Objects Value
            ReviewQualityMetrics,
            TeamCollaborationMetrics,
            ReviewPattern,
            CollaborationNetwork,
            CollaborationEdge,

            // Delivery insights //

            // Entities
            Deployment,
            DeploymentChange,
            Release,
            Incident,
            DeploymentStatus,
            ChangeType,

            // Objects Value
            DeliveryMetrics,
            DeploymentFrequency,
            LeadTime,
            ChangeFailureRate,
            PipelineStage,
            DeploymentPipeline,
            LeadTimePercentiles,
            RecoveryTimePoint,
            EnvironmentFrequency,
            FailureCategory,
            MeanTimeToRecover,
            RecoveryTimePoint,

            // Team
            Team,
            TeamResponse,
            TeamListResponse,
            CreateTeamRequest,
            UpdateTeamRequest,
            TeamWithMembersResponse,
            AddTeamMemberRequest,
            TeamMember,
            UserResponse,

            // Role
            Role,
            RoleResponse,
            RoleListResponse,
            CreateRoleRequest,
            UpdateRoleRequest,

            // User schemas
            CreateUserRequest,
            UpdateUserRequest,
            ChangePasswordRequest,
            AssignRoleRequest,
            UserWithRolesResponse,

            // Auth
            AuthResponse,
            UserInfoResponse,
            LoginRequest,
        )
    ),
    tags(
        (name = "Organizations", description = "Organization management endpoints"),
        (name = "Departments", description = "Department management endpoints"),
        (name = "Developer Metrics", description = "Developer productivity metrics endpoints"),
        (name = "Collaboration Metrics", description = "Team collaboration and quality metrics endpoints"),
        (name = "Delivery Metrics", description = "Project delivery and DORA metrics endpoints"),
        (name = "Teams", description = "Team management endpoints"),
        (name = "Roles", description = "Role management endpoints"),
        (name = "Users", description = "User management endpoints"),
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Organization Configs", description = "Organization configuration endpoints"),
    ),
    info(
        title = "Horus API",
        version = "1.0.0",
        description = r#"
## Authentication Guide

To authenticate API requests:

1. Use the `/api/v1/auth/login` endpoint with username/password to get a token
2. For each subsequent request, add the header: `Authorization: Bearer your_token_here`

For a helper tool that formats your token automatically:
- Visit [Token Helper](/api/v1/auth/token-helper) after logging in
        "#
    ),
    security(
        ("bearerAuth" = [])
    )
)]
pub struct ApiDoc;
