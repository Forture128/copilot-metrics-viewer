import axios from 'axios'
import type { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios'
import router from '../router'
import ToastService from './ToastService'
import config from '../config'
import store from '../store'

// Extend AxiosRequestConfig to support our custom properties
interface ExtendedAxiosRequestConfig extends AxiosRequestConfig {
  mockData?: any
  emptyResponse?: any
}

/**
 * DoraService - Service to communicate with the GitHub API for DORA metrics
 * This service handles authentication, API requests and provides methods for all GitHub API endpoints
 * needed for DevOps Research and Assessment (DORA) metrics.
 */
export class DoraService {
  private axiosInstance: AxiosInstance
  private baseUrl: string

  constructor(baseUrl: string = config.github.baseUrl) {
    this.baseUrl = baseUrl
    this.axiosInstance = axios.create({
      baseURL: this.baseUrl,
      timeout: 30000 // 30 seconds timeout
    })

    // Add request interceptor to include auth token
    this.axiosInstance.interceptors.request.use(
      config => {
        // Get token from Vuex store
        const token = store.state.auth?.token
        if (token) {
          config.headers.Authorization = `Bearer ${token}`
        }
        config.headers.Accept = 'application/vnd.github+json'
        config.headers['X-GitHub-Api-Version'] = '2022-11-28'
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
          if (message.includes('Bad credentials') || message.includes('Token expired')) {
            ToastService.error('Your session has expired. Please login again.')
            // Clear authentication data
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
   * Generic request method to handle all API calls
   */
  private async request<T>(config: ExtendedAxiosRequestConfig): Promise<T> {
    // If mock data is enabled, check if we have a mock handler
    if (config.url && config.method && config.mockData) {
      return config.mockData as T
    }

    try {
      const response: AxiosResponse<T> = await this.axiosInstance.request(config)
      return response.data
    } catch (error) {
      console.error(`Error in ${config.method} request to ${config.url}:`, error)
      // Let the interceptor handle the error display, but return empty data for the caller
      return (config.emptyResponse || []) as T
    }
  }

  /**
   * Get deployments for a repository
   * @param owner Repository owner
   * @param repo Repository name
   */
  public async getDeployments(owner: string, repo: string): Promise<any[]> {
    return this.request({
      method: 'GET',
      url: `/repos/${owner}/${repo}/deployments`,
      emptyResponse: []
    })
  }

  /**
   * Get pull requests for a repository
   * @param owner Repository owner
   * @param repo Repository name
   * @param params Optional parameters (state, sort, direction, etc.)
   */
  public async getPullRequests(
    owner: string,
    repo: string,
    params: Record<string, any> = {}
  ): Promise<any[]> {
    return this.request({
      method: 'GET',
      url: `/repos/${owner}/${repo}/pulls`,
      params,
      emptyResponse: []
    })
  }

  /**
   * Get deployment statuses for a specific deployment
   * @param owner Repository owner
   * @param repo Repository name
   * @param deploymentId Deployment ID
   */
  public async getDeploymentStatuses(
    owner: string,
    repo: string,
    deploymentId: number
  ): Promise<any[]> {
    return this.request({
      method: 'GET',
      url: `/repos/${owner}/${repo}/deployments/${deploymentId}/statuses`,
      emptyResponse: []
    })
  }

  /**
   * Get commits for a repository
   * @param owner Repository owner
   * @param repo Repository name
   */
  public async getCommits(owner: string, repo: string): Promise<any[]> {
    return this.request({
      method: 'GET',
      url: `/repos/${owner}/${repo}/commits`,
      emptyResponse: []
    })
  }

  /**
   * Get workflow runs for a repository
   * @param owner Repository owner
   * @param repo Repository name
   */
  public async getWorkflowRuns(owner: string, repo: string): Promise<any> {
    return this.request({
      method: 'GET',
      url: `/repos/${owner}/${repo}/actions/runs`,
      emptyResponse: { workflow_runs: [] }
    })
  }

  /**
   * Get issues for a repository
   * @param owner Repository owner
   * @param repo Repository name
   */
  public async getIssues(owner: string, repo: string): Promise<any[]> {
    return this.request({
      method: 'GET',
      url: `/repos/${owner}/${repo}/issues`,
      emptyResponse: []
    })
  }

  /**
   * Get pull request reviews
   * @param owner Repository owner
   * @param repo Repository name
   * @param pull_number Pull request number
   */
  public async getTimePullRequestReviews(
    owner: string,
    repo: string,
    pull_number: number
  ): Promise<any> {
    try {
      const reviews = await this.request<any[]>({
        method: 'GET',
        url: `/repos/${owner}/${repo}/pulls/${pull_number}/reviews`,
        emptyResponse: []
      })

      if (reviews.length > 0) {
        const firstReview = reviews[0]
        const approvedReview = reviews.find(
          (review: { state: string }) => review.state === 'APPROVED'
        )
        const lastReview = reviews[reviews.length - 1]

        return {
          first_review_at: firstReview.submitted_at,
          approved_at: approvedReview ? approvedReview.submitted_at : null,
          review_completed_at: lastReview.submitted_at
        }
      }
      return null
    } catch (error) {
      console.error('Error processing PR reviews:', error)
      return null
    }
  }

  /**
   * Get repository information
   * @param owner Repository owner
   * @param repo Repository name
   */
  public async getRepository(owner: string, repo: string): Promise<any> {
    return this.request({
      method: 'GET',
      url: `/repos/${owner}/${repo}`,
      emptyResponse: {}
    })
  }

  /**
   * Get repositories for a specific owner
   * @param owner Username or organization name
   * @param page Page number for pagination
   * @param perPage Items per page (default: 50)
   * @param type Type of repositories to return (all, owner, member, etc)
   */
  public async getRepositoriesByOwner(
    owner: string,
    page = 1,
    perPage = 50,
    type = 'public'
  ): Promise<any[]> {
    // Determine if owner is user or org (simplified, you might need more logic)
    return this.request({
      method: 'GET',
      url: `/orgs/${owner}/repos`,
      params: {
        type,
        sort: 'updated',
        per_page: perPage,
        page
      },
      emptyResponse: []
    })
  }

  /**
   * Get all public repositories or repositories for the authenticated user
   * @param page Page number for pagination
   * @param perPage Items per page (default: 50)
   */
  public async getRepositories(page = 1, perPage = 50): Promise<any[]> {
    return this.request({
      method: 'GET',
      url: `/repositories`,
      params: {
        since: (page - 1) * perPage,
        per_page: perPage
      },
      emptyResponse: []
    })
  }
}

// Create and export a singleton instance
export const doraService = new DoraService()
export default doraService
