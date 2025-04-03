/**
 * Horus API types
 * This file contains TypeScript interfaces for the Horus API responses.
 */

// Authentication Types
export interface AuthResponse {
  access_token: string
  token_type: string
  expires_in: number
  role: string
}

export interface UserInfoResponse {
  user_id: number
  organization_id: number
  username: string
}

// User Types
export interface UserResponse {
  id: number
  username: string
  email: string
  first_name: string
  last_name: string
  created_at: string
  updated_at: string
}

export interface RoleResponse {
  id: number
  name: string
  description: string
  created_at: string
  updated_at: string
}

export interface UserWithRolesResponse extends UserResponse {
  roles: RoleResponse[]
}

// Organization Types
export interface OrganizationResponse {
  id: number
  name: string
  description?: string
  created_at: string
  updated_at: string
}

export interface OrganizationListResponse {
  organizations: OrganizationResponse[]
  total: number
  page?: number
  per_page?: number
  pages?: number
}

export interface OrganizationConfigResponse {
  id: number
  organization_id: number
  config_key: string
  config_value: string
  created_at: string
  updated_at: string
}

export interface OrganizationConfigListResponse {
  configs: OrganizationConfigResponse[]
  total: number
  page: number
  per_page: number
}

// Department Types
export interface DepartmentResponse {
  id: number
  name: string
  description: string
  created_at: string
  updated_at: string
  user_count?: number
  role_count?: number
}

export interface DepartmentListResponse {
  items: DepartmentResponse[]
  total: number
  page: number
  per_page: number
  pages: number
}

export interface DepartmentRoleResponse {
  id: number
  department_id: number
  role_id: number
  created_at: string
  updated_at: string
}

export interface DepartmentRoleListResponse {
  items: DepartmentRoleResponse[]
  total: number
  page: number
  per_page: number
  pages: number
}

// Developer Metrics Types
export interface Commit {
  total_count: number
  commit_frequency: number
  lines_added: number
  lines_removed: number
  files_changed: number
  commit_timestamps: string[]
}

export interface PullRequest {
  total_count: number
  open_count: number
  closed_count: number
  merged_count: number
  pr_frequency: number
  avg_time_to_merge: number
  avg_comments_per_pr: number
}

export interface Metrics {
  username: string
  time_period: {
    start_date: string
    end_date: string
  }
  commit_metrics: Commit
  pr_metrics: PullRequest
  active_days: number
  contribution_score: number
}

// Collaboration Quality Types
export interface CodeReview {
  team_name: string
  time_period: {
    start_date: string
    end_date: string
  }
  total_reviews: number
  avg_review_time: number
  avg_comments_per_review: number
  reviews_by_day: { [date: string]: number }
  top_reviewers: Array<{ username: string; review_count: number }>
}

export interface CollaborationMetrics {
  team_name: string
  time_period: {
    start_date: string
    end_date: string
  }
  pr_collaboration_score: number
  cross_functional_collaboration: number
  knowledge_sharing_index: number
  team_responsiveness: number
}

export interface TeamInteraction {
  team_name: string
  time_period: {
    start_date: string
    end_date: string
  }
  interactions_with_other_teams: Array<{
    team_name: string
    interaction_count: number
    collaboration_type: string
  }>
  interaction_trend: Array<{
    date: string
    interaction_count: number
  }>
}

// Delivery Insights Types
export interface DeliveryMetrics {
  repository: string
  time_period: {
    start_date: string
    end_date: string
    period: string
  }
  deployment_frequency: number
  lead_time: number
  change_failure_rate: number
  mean_time_to_restore: number
  dora_score: string
  metrics_by_period: Array<{
    period_start: string
    deployment_frequency: number
    lead_time: number
    change_failure_rate: number
    mean_time_to_restore: number
  }>
}

// API Response Types
export interface ApiResponseData {
  averageLatency: number
  successRate: number
  errorsByType: Record<string, number>
  responseTimeDistribution: Record<string, number>
}

// Common Types
export interface ErrorResponse {
  status: number
  message: string
  details?: string
}

// Request Types
export interface LoginRequest {
  username: string
  username_or_email: string
}

export interface CreateUserRequest {
  username: string
  email: string
  first_name: string
  last_name: string
  password: string
}

export interface UpdateUserRequest {
  email?: string
  first_name?: string
  last_name?: string
}

export interface ChangePasswordRequest {
  current_password: string
  new_password: string
}

export interface AssignRoleRequest {
  user_id: number
  role_id: number
}

export interface CreateOrganizationRequest {
  name: string
  description?: string
}

export interface UpdateOrganizationRequest {
  name?: string
  description?: string
}

export interface CreateOrganizationConfigRequest {
  config_key: string
  config_value: string
}

export interface UpdateOrganizationConfigRequest {
  config_value: string
}

export interface CreateDepartmentRequest {
  name: string
  description?: string
}

export interface UpdateDepartmentRequest {
  name?: string
  description?: string
}

export interface AssignUserToDepartmentRequest {
  user_id: number
  role: string
}

export interface BatchAssignUsersToDepartmentRequest {
  user_ids: number[]
  role: string
}

// Role Types
export interface RoleListResponse {
  items: RoleResponse[]
  total: number
  page: number
  per_page: number
  pages: number
}

export interface AssignDepartmentRoleRequest {
  role_id: number
}
