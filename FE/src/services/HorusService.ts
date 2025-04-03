import axios from 'axios'
import type { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios'
import router from '../router'
import store from '../store'
import ToastService from './ToastService'
import type {
  AuthResponse,
  UserInfoResponse,
  UserResponse,
  UserWithRolesResponse,
  OrganizationResponse,
  OrganizationListResponse,
  OrganizationConfigResponse,
  DepartmentResponse,
  Metrics,
  Commit,
  PullRequest,
  CodeReview,
  CollaborationMetrics,
  TeamInteraction,
  DeliveryMetrics,
  DepartmentListResponse,
  DepartmentRoleListResponse,
  DepartmentRoleResponse,
  RoleListResponse,
  AssignDepartmentRoleRequest,
  OrganizationConfigListResponse,
  UpdateOrganizationConfigRequest,
  CreateOrganizationConfigRequest
} from '../types/horus-api.types'

/**
 * HorusService - Main service to communicate with the Horus backend APIs
 * This service handles authentication, API requests and provides methods for all API endpoints.
 */
export class HorusService {
  private axiosInstance: AxiosInstance
  private baseUrl: string
  private authToken: string | null = null
  private orgId: number | null = null

  constructor(baseUrl: string = import.meta.env.VITE_HORUS_API_URL || 'http://localhost:3000') {
    this.baseUrl = baseUrl
    this.axiosInstance = axios.create({
      baseURL: this.baseUrl,
      timeout: 30000 // 30 seconds timeout
    })

    // Add request interceptor to include auth token
    this.axiosInstance.interceptors.request.use(
      config => {
        if (this.authToken) {
          config.headers.Authorization = `Bearer ${this.authToken}`
        }
        return config
      },
      error => {
        return Promise.reject(error)
      }
    )

    // Add response interceptor for error handling
    this.axiosInstance.interceptors.response.use(
      response => response,
      error => {
        this.handleApiError(error)
        return Promise.reject(error)
      }
    )
  }

  /**
   * Handle API errors in a centralized way
   */
  private handleApiError(error: any): void {
    if (error.response) {
      // The request was made and the server responded with a status code outside 2xx range
      const status = error.response.status
      const message = error.response.data?.message || 'An error occurred'

      switch (status) {
        case 401:
          // Check for token expiration specifically
          if (message.includes('ExpiredSignature') || message.includes('Token validation error')) {
            ToastService.error('Your session has expired. Please login again.')
            // Clear authentication data
            this.clearAuth()
            // Dispatch logout action to update store state
            store.dispatch('auth/logout')
            // Redirect to login page
            router.push('/login')
          } else {
            ToastService.error('Authentication failed. Please login again.')
          }
          break
        case 403:
          ToastService.error('You do not have permission to perform this action.')
          break
        case 404:
          ToastService.error('The requested resource was not found.')
          break
        case 422:
          ToastService.error('Validation error. Please check your input.')
          break
        default:
          ToastService.error(`Error: ${message}`)
      }
    } else if (error.request) {
      // The request was made but no response was received
      ToastService.error('No response from server. Please check your connection.')
    } else {
      // Something happened in setting up the request
      ToastService.error(`Request error: ${error.message}`)
    }
  }

  /**
   * Set the authentication token and organization ID
   */
  public setAuth(token: string, orgId: number): void {
    this.authToken = token
    this.orgId = orgId
    // Store in localStorage for persistence across page reloads
    localStorage.setItem('authToken', token)
    localStorage.setItem('orgId', orgId.toString())
  }

  /**
   * Clear authentication data (for logout)
   */
  public clearAuth(): void {
    this.authToken = null
    this.orgId = null
    localStorage.removeItem('authToken')
    localStorage.removeItem('orgId')
  }

  /**
   * Load authentication from localStorage if available
   */
  public loadAuthFromStorage(): boolean {
    const token = localStorage.getItem('authToken')
    const orgId = localStorage.getItem('orgId')

    if (token && orgId) {
      this.authToken = token
      this.orgId = parseInt(orgId, 10)
      return true
    }
    return false
  }

  /**
   * Check if user is authenticated
   */
  public isAuthenticated(): boolean {
    return this.authToken !== null
  }

  /**
   * Get the current organization ID
   */
  public getOrgId(): number | null {
    return this.orgId
  }

  // AUTHENTICATION APIS

  /**
   * Login user and get authentication token
   * @param username_or_email User's username or email
   * @param password User's password
   */
  public async login(username_or_email: string, password: string): Promise<AuthResponse> {
    const response = await this.axiosInstance.post<AuthResponse>('/api/v1/auth/login', {
      username_or_email,
      password
    })

    if (response.data && response.data.access_token) {
      this.authToken = response.data.access_token

      // Get user info to retrieve organization ID
      const userInfo = await this.getCurrentUser()
      if (userInfo && userInfo.organization_id) {
        this.orgId = userInfo.organization_id
        if (this.authToken && this.orgId !== null) {
          this.setAuth(this.authToken, this.orgId)
        }
      }
    }

    return response.data
  }

  /**
   * Get current authenticated user information
   */
  public async getCurrentUser(): Promise<UserInfoResponse> {
    const response = await this.axiosInstance.get<UserInfoResponse>('/api/v1/auth/me')
    return response.data
  }

  /**
   * Generic request method to handle all API calls
   */
  private async request<T>(config: AxiosRequestConfig): Promise<T> {
    const response: AxiosResponse<T> = await this.axiosInstance.request(config)
    return response.data
  }

  // USER MANAGEMENT APIS

  /**
   * List users with pagination
   * @param organizationId Organization ID
   * @param page Page number (optional)
   * @param limit Items per page (optional)
   */
  public async listUsers(organizationId: number, page?: number, limit?: number): Promise<any> {
    const params: Record<string, any> = {}
    if (page !== undefined) params.page = page
    if (limit !== undefined) params.limit = limit

    return this.request({
      method: 'GET',
      url: `/api/users-orgs/${organizationId}/users`,
      params
    })
  }

  /**
   * Register a new user
   * @param userData User data
   */
  public async registerUser(userData: any): Promise<any> {
    return this.request({
      method: 'POST',
      url: '/api/users/register',
      data: userData
    })
  }

  /**
   * Get user by ID
   * @param userId User ID
   */
  public async getUser(userId: number): Promise<UserResponse> {
    return this.request<UserResponse>({
      method: 'GET',
      url: `/api/users/${userId}`
    })
  }

  /**
   * Update user by ID
   * @param userId User ID
   * @param userData Updated user data
   */
  public async updateUser(userId: number, userData: any): Promise<any> {
    return this.request({
      method: 'PUT',
      url: `/api/users/${userId}`,
      data: userData
    })
  }

  /**
   * Change user password
   * @param userId User ID
   * @param passwordData Password change data
   */
  public async changePassword(userId: number, passwordData: any): Promise<any> {
    return this.request({
      method: 'PUT',
      url: `/api/users/${userId}/change-password`,
      data: passwordData
    })
  }

  /**
   * Get user with roles by ID
   * @param userId User ID
   */
  public async getUserWithRoles(userId: number): Promise<UserWithRolesResponse> {
    return this.request<UserWithRolesResponse>({
      method: 'GET',
      url: `/api/users/${userId}/roles`
    })
  }

  /**
   * Assign role to user
   * @param roleData Role assignment data
   */
  public async assignRole(roleData: any): Promise<any> {
    return this.request({
      method: 'POST',
      url: '/api/users/roles',
      data: roleData
    })
  }

  /**
   * Remove role from user
   * @param userId User ID
   * @param roleId Role ID
   */
  public async removeRole(userId: number, roleId: number): Promise<any> {
    return this.request({
      method: 'DELETE',
      url: `/api/users/${userId}/roles/${roleId}`
    })
  }

  // ORGANIZATION APIS

  /**
   * List organizations with pagination
   * @param page Page number (optional)
   * @param perPage Items per page (optional)
   */
  public async listOrganizations(
    page?: number,
    perPage?: number
  ): Promise<OrganizationListResponse> {
    const params: Record<string, any> = {}
    if (page !== undefined) params.page = page
    if (perPage !== undefined) params.per_page = perPage

    return this.request<OrganizationListResponse>({
      method: 'GET',
      url: '/api/v1/organizations',
      params
    })
  }

  /**
   * Create a new organization
   * @param orgData Organization data
   */
  public async createOrganization(orgData: any): Promise<any> {
    return this.request({
      method: 'POST',
      url: '/api/v1/organizations',
      data: orgData
    })
  }

  /**
   * Batch create organizations
   * @param orgsData Array of organization data
   */
  public async batchCreateOrganizations(orgsData: any[]): Promise<any> {
    return this.request({
      method: 'POST',
      url: '/api/v1/organizations/batch',
      data: orgsData
    })
  }

  /**
   * Batch update organizations
   * @param updateData Array of [organizationId, updateData] pairs
   */
  public async batchUpdateOrganizations(updateData: any[]): Promise<any> {
    return this.request({
      method: 'PUT',
      url: '/api/v1/organizations/batch',
      data: updateData
    })
  }

  /**
   * Batch delete organizations
   * @param orgIds Array of organization IDs
   */
  public async batchDeleteOrganizations(orgIds: number[]): Promise<any> {
    return this.request({
      method: 'DELETE',
      url: '/api/v1/organizations/batch',
      data: orgIds
    })
  }

  /**
   * Get organizations by IDs
   * @param orgIds Array of organization IDs
   */
  public async getOrganizationsByIds(orgIds: number[]): Promise<any> {
    return this.request({
      method: 'POST',
      url: '/api/v1/organizations/get-by-ids',
      data: orgIds
    })
  }

  /**
   * Search organizations by name
   * @param name Name pattern to search for
   * @param page Page number (optional)
   * @param perPage Items per page (optional)
   */
  public async searchOrganizationsByName(
    name: string,
    page?: number,
    perPage?: number
  ): Promise<any> {
    const params: Record<string, any> = { name }
    if (page !== undefined) params.page = page
    if (perPage !== undefined) params.per_page = perPage

    return this.request({
      method: 'GET',
      url: '/api/v1/organizations/search',
      params
    })
  }

  /**
   * Get organization by ID
   * @param orgId Organization ID
   */
  public async getOrganization(orgId: number): Promise<OrganizationResponse> {
    return this.request<OrganizationResponse>({
      method: 'GET',
      url: `/organizations/${orgId}`
    })
  }

  /**
   * Update organization
   * @param orgId Organization ID
   * @param orgData Updated organization data
   */
  public async updateOrganization(orgId: number, orgData: any): Promise<any> {
    return this.request({
      method: 'PUT',
      url: `/api/v1/organizations/${orgId}`,
      data: orgData
    })
  }

  /**
   * Delete organization
   * @param orgId Organization ID
   */
  public async deleteOrganization(orgId: number): Promise<any> {
    return this.request({
      method: 'DELETE',
      url: `/api/v1/organizations/${orgId}`
    })
  }

  /**
   * Check if organization exists
   * @param orgId Organization ID
   */
  public async organizationExists(orgId: number): Promise<boolean> {
    return this.request({
      method: 'GET',
      url: `/api/v1/organizations/${orgId}/exists`
    })
  }

  // ORGANIZATION CONFIG APIS

  /**
   * List organization configurations
   * @param orgId Organization ID
   */
  public async listOrganizationConfigs(orgId: number): Promise<OrganizationConfigListResponse> {
    return this.request<OrganizationConfigListResponse>({
      method: 'GET',
      url: `/api/v1/organizations/${orgId}/configs`
    })
  }

  /**
   * Create organization configuration
   * @param orgId Organization ID
   * @param data Configuration data
   */
  public async createOrganizationConfig(
    orgId: number,
    data: CreateOrganizationConfigRequest
  ): Promise<OrganizationConfigResponse> {
    return this.request<OrganizationConfigResponse>({
      method: 'POST',
      url: `/api/v1/organizations/${orgId}/configs`,
      data
    })
  }

  /**
   * Update organization configuration
   */
  public async updateOrganizationConfig(
    orgId: number,
    configId: number,
    data: UpdateOrganizationConfigRequest
  ): Promise<OrganizationConfigResponse> {
    return this.request<OrganizationConfigResponse>({
      method: 'PUT',
      url: `/organizations/${orgId}/configs/${configId}`,
      data
    })
  }

  // DEPARTMENT APIS

  /**
   * List departments with pagination
   * @param page Page number (optional)
   * @param perPage Items per page (optional)
   */
  public async listDepartments(page?: number, perPage?: number): Promise<DepartmentListResponse> {
    const params: Record<string, any> = {}
    if (page !== undefined) params.page = page
    if (perPage !== undefined) params.per_page = perPage

    return this.request<DepartmentListResponse>({
      method: 'GET',
      url: '/api/v1/departments',
      params
    })
  }

  /**
   * Get department roles
   * @param departmentId Department ID
   */
  public async getDepartmentRoles(departmentId: number): Promise<DepartmentRoleListResponse> {
    return this.request<DepartmentRoleListResponse>({
      method: 'GET',
      url: `/api/v1/departments/${departmentId}/roles`
    })
  }

  /**
   * Assign role to department
   * @param departmentId Department ID
   * @param roleData Role assignment data
   */
  public async assignDepartmentRole(
    departmentId: number,
    roleData: AssignDepartmentRoleRequest
  ): Promise<DepartmentRoleResponse> {
    return this.request<DepartmentRoleResponse>({
      method: 'POST',
      url: `/api/v1/departments/${departmentId}/roles`,
      data: roleData
    })
  }

  /**
   * Remove role from department
   * @param departmentId Department ID
   * @param roleId Role ID
   */
  public async removeDepartmentRole(departmentId: number, roleId: number): Promise<boolean> {
    return this.request<boolean>({
      method: 'DELETE',
      url: `/api/v1/departments/${departmentId}/roles/${roleId}`
    })
  }

  /**
   * List roles with pagination
   * @param page Page number (optional)
   * @param perPage Items per page (optional)
   */
  public async listRoles(page?: number, perPage?: number): Promise<RoleListResponse> {
    const params: Record<string, any> = {}
    if (page !== undefined) params.page = page
    if (perPage !== undefined) params.per_page = perPage

    return this.request<RoleListResponse>({
      method: 'GET',
      url: '/api/v1/roles',
      params
    })
  }

  /**
   * Create a new department
   * @param departmentData Department data
   */
  public async createDepartment(departmentData: any): Promise<any> {
    return this.request({
      method: 'POST',
      url: '/api/v1/departments',
      data: departmentData
    })
  }

  /**
   * Get department by ID
   * @param departmentId Department ID
   */
  public async getDepartment(departmentId: number): Promise<DepartmentResponse> {
    return this.request<DepartmentResponse>({
      method: 'GET',
      url: `/api/v1/departments/${departmentId}`
    })
  }

  /**
   * Update department
   * @param departmentId Department ID
   * @param departmentData Updated department data
   */
  public async updateDepartment(departmentId: number, departmentData: any): Promise<any> {
    return this.request({
      method: 'PUT',
      url: `/api/v1/departments/${departmentId}`,
      data: departmentData
    })
  }

  /**
   * Delete department
   * @param departmentId Department ID
   */
  public async deleteDepartment(departmentId: number): Promise<any> {
    return this.request({
      method: 'DELETE',
      url: `/api/v1/departments/${departmentId}`
    })
  }

  /**
   * Get users in a department
   * @param departmentId Department ID
   */
  public async getDepartmentUsers(departmentId: number): Promise<any> {
    return this.request({
      method: 'GET',
      url: `/api/v1/departments/${departmentId}/users`
    })
  }

  /**
   * Add a user to a department
   * @param departmentId Department ID
   * @param assignData Assignment data
   */
  public async addUserToDepartment(departmentId: number, assignData: any): Promise<any> {
    return this.request({
      method: 'POST',
      url: `/api/v1/departments/${departmentId}/users`,
      data: assignData
    })
  }

  /**
   * Batch add users to a department
   * @param departmentId Department ID
   * @param batchAssignData Batch assignment data
   */
  public async batchAddUsersToDepartment(departmentId: number, batchAssignData: any): Promise<any> {
    return this.request({
      method: 'POST',
      url: `/api/v1/departments/${departmentId}/users/batch`,
      data: batchAssignData
    })
  }

  /**
   * Remove a user from a department
   * @param departmentId Department ID
   * @param userId User ID
   */
  public async removeUserFromDepartment(departmentId: number, userId: number): Promise<any> {
    return this.request({
      method: 'DELETE',
      url: `/api/v1/departments/${departmentId}/users/${userId}`
    })
  }

  // DEVELOPER METRICS APIS

  /**
   * Get metrics for a developer
   * @param username GitHub username of the developer
   * @param startDate Start date in ISO 8601 format
   * @param endDate End date in ISO 8601 format
   */
  public async getDeveloperMetrics(
    username: string,
    startDate: string,
    endDate: string
  ): Promise<Metrics> {
    return this.request<Metrics>({
      method: 'GET',
      url: '/api/v1/developer/metrics',
      params: {
        username,
        start_date: startDate,
        end_date: endDate
      }
    })
  }

  /**
   * Get commit statistics for a developer
   * @param username GitHub username of the developer
   * @param startDate Start date in ISO 8601 format
   * @param endDate End date in ISO 8601 format
   */
  public async getDeveloperCommits(
    username: string,
    startDate: string,
    endDate: string
  ): Promise<Commit> {
    return this.request<Commit>({
      method: 'GET',
      url: '/api/v1/developer/commits',
      params: {
        username,
        start_date: startDate,
        end_date: endDate
      }
    })
  }

  /**
   * Get pull request statistics for a developer
   * @param username GitHub username of the developer
   * @param startDate Start date in ISO 8601 format
   * @param endDate End date in ISO 8601 format
   */
  public async getDeveloperPullRequests(
    username: string,
    startDate: string,
    endDate: string
  ): Promise<PullRequest> {
    return this.request<PullRequest>({
      method: 'GET',
      url: '/api/v1/developer/pull-requests',
      params: {
        username,
        start_date: startDate,
        end_date: endDate
      }
    })
  }

  // COLLABORATION QUALITY APIS

  /**
   * Get collaboration metrics for a team
   * @param teamName Name of the team
   * @param startDate Start date in ISO 8601 format
   * @param endDate End date in ISO 8601 format
   */
  public async getCollaborationMetrics(
    teamName: string,
    startDate: string,
    endDate: string
  ): Promise<CollaborationMetrics> {
    return this.request<CollaborationMetrics>({
      method: 'GET',
      url: '/api/v1/collaboration/metrics',
      params: {
        team_name: teamName,
        start_date: startDate,
        end_date: endDate
      }
    })
  }

  /**
   * Get code review statistics for a team
   * @param teamName Name of the team
   * @param startDate Start date in ISO 8601 format
   * @param endDate End date in ISO 8601 format
   */
  public async getCodeReviewStats(
    teamName: string,
    startDate: string,
    endDate: string
  ): Promise<CodeReview> {
    return this.request<CodeReview>({
      method: 'GET',
      url: '/api/v1/collaboration/code-review-stats',
      params: {
        team_name: teamName,
        start_date: startDate,
        end_date: endDate
      }
    })
  }

  /**
   * Get team interaction metrics
   * @param teamName Name of the team
   * @param startDate Start date in ISO 8601 format
   * @param endDate End date in ISO 8601 format
   */
  public async getTeamInteractions(
    teamName: string,
    startDate: string,
    endDate: string
  ): Promise<TeamInteraction> {
    return this.request<TeamInteraction>({
      method: 'GET',
      url: '/api/v1/collaboration/team-interactions',
      params: {
        team_name: teamName,
        start_date: startDate,
        end_date: endDate
      }
    })
  }

  // DELIVERY INSIGHTS APIS

  /**
   * Get delivery metrics for a repository
   * @param identifier Repository name
   * @param startDate Start date in ISO 8601 format (optional)
   * @param endDate End date in ISO 8601 format (optional)
   * @param period Period - daily, weekly, monthly, yearly (optional)
   */
  public async getDeliveryMetrics(
    identifier: string,
    startDate?: string,
    endDate?: string,
    period?: string
  ): Promise<DeliveryMetrics> {
    const params: Record<string, any> = { identifier }
    if (startDate) params.start_date = startDate
    if (endDate) params.end_date = endDate
    if (period) params.period = period

    return this.request<DeliveryMetrics>({
      method: 'GET',
      url: '/api/v1/delivery/metrics',
      params
    })
  }
}

// Create and export a singleton instance
export const horusService = new HorusService()
export default horusService
