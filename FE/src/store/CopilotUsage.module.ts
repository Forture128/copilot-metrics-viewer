import type { Module } from 'vuex'
import type { RootState } from './index'
// Import the API methods from GitHubApi
import { getMetricsApi, getTeamMembers, getTeamMetricsApi } from '@/api/GitHubApi'
import type { ApiResponseData } from '@/types/horus-api.types'
// Import horusService only if we'll use it for API calls
// import horusService from '@/services/HorusService'
import axios from 'axios'

// Define the state type
export interface CopilotUsageState {
  // General metrics
  metrics: any | null
  metricsLoading: boolean
  metricsError: string | null
  metricsReady: boolean

  // Chat data
  chatData: any | null
  chatDataLoading: boolean
  chatDataError: string | null

  // Seats data
  seatsData: any | null
  seatsDataLoading: boolean
  seatsDataError: string | null

  // API response data
  apiResponseData: ApiResponseData | null
  apiResponseLoading: boolean
  apiResponseError: string | null

  // Team data
  teamData: any | null
  teamDataLoading: boolean
  teamDataError: string | null

  // Department data
  departmentData: any | null
  departmentDataLoading: boolean
  departmentDataError: string | null

  // Legacy properties for backwards compatibility
  seats: any | null
  teamList: string[] | null
  teamMetrics: any | null
  teamMetricsReady: boolean
  apiError: string | null
  departments: { [key: string]: string[] } | null
  seatsReady: boolean
}

// Create the module
const copilotUsageModule: Module<CopilotUsageState, RootState> = {
  namespaced: true,

  state: {
    // General metrics
    metrics: null,
    metricsLoading: false,
    metricsError: null,
    metricsReady: false,

    // Chat data
    chatData: null,
    chatDataLoading: false,
    chatDataError: null,

    // Seats data
    seatsData: null,
    seatsDataLoading: false,
    seatsDataError: null,

    // API response data
    apiResponseData: null,
    apiResponseLoading: false,
    apiResponseError: null,

    // Team data
    teamData: null,
    teamDataLoading: false,
    teamDataError: null,

    // Department data
    departmentData: null,
    departmentDataLoading: false,
    departmentDataError: null,

    // Legacy properties for backwards compatibility
    seats: null,
    teamList: [
      'cloud_transfer_reviewers',
      'neo-ap',
      'rci-mfv',
      'attendance_dev_reviewers',
      'mf_connected_db_developers',
      'payroll_dev_reviewers',
      'payroll_kotlin_reviewers',
      'social_insurance_dev_reviewers',
      'tax_adjustment_dev_mfj_reviewers'
    ],
    teamMetrics: [],
    teamMetricsReady: false,
    apiError: null,
    departments: {
      'Department 1': [
        'cloud_transfer_reviewers',
        'neo-ap',
        'rci-mfv',
        'attendance_dev_reviewers',
        'mf_connected_db_developers',
        'payroll_dev_reviewers',
        'payroll_kotlin_reviewers',
        'social_insurance_dev_reviewers',
        'tax_adjustment_dev_mfj_reviewers'
      ],
      'Department 2': ['rci-mfv', 'attendance_dev_reviewers', 'mf_connected_db_developers'],
      'Department 3': ['payroll_dev_reviewers', 'payroll_kotlin_reviewers'],
      'Department 4': ['social_insurance_dev_reviewers', 'tax_adjustment_dev_mfj_reviewers']
    },
    seatsReady: false
  },

  getters: {
    // General metrics getters
    getMetrics: state => state.metrics,
    isMetricsLoading: state => state.metricsLoading,
    getMetricsError: state => state.metricsError,

    // Chat data getters
    getChatData: state => state.chatData,
    isChatDataLoading: state => state.chatDataLoading,
    getChatDataError: state => state.chatDataError,

    // Seats data getters
    getSeatsData: state => state.seatsData,
    isSeatsDataLoading: state => state.seatsDataLoading,
    getSeatsDataError: state => state.seatsDataError,

    // API response data getters
    getApiResponseData: state => state.apiResponseData,
    isApiResponseLoading: state => state.apiResponseLoading,
    getApiResponseError: state => state.apiResponseError,

    // Team data getters
    getTeamData: state => state.teamData,
    isTeamDataLoading: state => state.teamDataLoading,
    getTeamDataError: state => state.teamDataError,

    // Department data getters
    getDepartmentData: state => state.departmentData,
    isDepartmentDataLoading: state => state.departmentDataLoading,
    getDepartmentDataError: state => state.departmentDataError
  },

  mutations: {
    // General metrics mutations
    SET_METRICS(state, metrics) {
      console.log(
        '[CopilotUsage:Mutation] SET_METRICS:',
        metrics
          ? `Setting ${Array.isArray(metrics) ? metrics.length : 1} records`
          : 'Setting null metrics'
      )
      state.metrics = metrics
      state.metricsReady = true
      console.log('[CopilotUsage:Mutation] metricsReady set to true')
    },
    SET_METRICS_LOADING(state, isLoading) {
      console.log('[CopilotUsage:Mutation] SET_METRICS_LOADING:', isLoading)
      state.metricsLoading = isLoading
    },
    SET_METRICS_ERROR(state, error) {
      if (error) {
        console.log('[CopilotUsage:Mutation] SET_METRICS_ERROR:', error)
      }
      state.metricsError = error
      state.apiError = error // For legacy compatibility
    },
    SET_METRICS_READY(state, isReady) {
      console.log('[CopilotUsage:Mutation] SET_METRICS_READY:', isReady)
      state.metricsReady = isReady
    },

    // Chat data mutations
    SET_CHAT_DATA(state, data) {
      state.chatData = data
    },
    SET_CHAT_DATA_LOADING(state, isLoading) {
      state.chatDataLoading = isLoading
    },
    SET_CHAT_DATA_ERROR(state, error) {
      state.chatDataError = error
    },

    // Seats data mutations
    SET_SEATS_DATA(state, data) {
      state.seatsData = data
      state.seats = data // For legacy compatibility
    },
    SET_SEATS_DATA_LOADING(state, isLoading) {
      state.seatsDataLoading = isLoading
    },
    SET_SEATS_DATA_ERROR(state, error) {
      state.seatsDataError = error
    },
    SET_SEATS_READY(state, ready) {
      state.seatsReady = ready
    },

    // API response data mutations
    SET_API_RESPONSE_DATA(state, data) {
      state.apiResponseData = data
    },
    SET_API_RESPONSE_LOADING(state, isLoading) {
      state.apiResponseLoading = isLoading
    },
    SET_API_RESPONSE_ERROR(state, error) {
      state.apiResponseError = error
    },

    // Team data mutations
    SET_TEAM_DATA(state, data) {
      state.teamData = data
      // Don't overwrite the entire teamMetrics array for a single team
      // state.teamMetrics = data // For legacy compatibility
      state.teamMetricsReady = true // For legacy compatibility

      // Extract team list from data for legacy components
      if (data && data.teams) {
        state.teamList = data.teams.map((team: any) => team.id)
      }
    },
    SET_TEAM_DATA_LOADING(state, isLoading) {
      state.teamDataLoading = isLoading
    },
    SET_TEAM_DATA_ERROR(state, error) {
      state.teamDataError = error
    },

    // Department data mutations
    SET_DEPARTMENT_DATA(state, data) {
      state.departmentData = data

      // For legacy compatibility
      if (data && data.departments) {
        state.departments = data.departments
      }
    },
    SET_DEPARTMENT_DATA_LOADING(state, isLoading) {
      state.departmentDataLoading = isLoading
    },
    SET_DEPARTMENT_DATA_ERROR(state, error) {
      state.departmentDataError = error
    },

    // Team metrics mutations
    ADD_TEAM_METRICS(state, data) {
      console.log('[CopilotUsage:Mutation] ADD_TEAM_METRICS for team:', data?.team_tag || 'unknown')
      if (!Array.isArray(state.teamMetrics)) {
        console.log('[CopilotUsage:Mutation] Initializing teamMetrics as empty array')
        state.teamMetrics = []
      }

      // Check if this team is already in the array
      const existingIndex = state.teamMetrics.findIndex(
        (team: any) => team.team_tag === data.team_tag
      )
      if (existingIndex >= 0) {
        console.log('[CopilotUsage:Mutation] Updating existing team data for', data.team_tag)
        state.teamMetrics[existingIndex] = data
      } else {
        console.log('[CopilotUsage:Mutation] Adding new team data for', data.team_tag)
        state.teamMetrics.push(data)
      }

      console.log('[CopilotUsage:Mutation] teamMetrics now has', state.teamMetrics.length, 'items')
    },

    // Legacy compatibility mutations
    SET_TEAM_METRICS_READY(state, ready) {
      console.log('[CopilotUsage:Mutation] SET_TEAM_METRICS_READY:', ready)
      state.teamMetricsReady = ready
    }
  },

  actions: {
    // General metrics actions
    async fetchMetrics({ commit, dispatch }) {
      console.log('[CopilotUsage] Starting fetchMetrics action')
      commit('SET_METRICS_LOADING', true)
      commit('SET_METRICS_ERROR', null)

      try {
        console.log('[CopilotUsage] Calling getMetricsApi()')
        const metricsData = await getMetricsApi()

        console.log(
          '[CopilotUsage] Metrics API call successful:',
          metricsData
            ? `Received ${Array.isArray(metricsData) ? metricsData.length : 1} records`
            : 'Empty response'
        )

        if (Array.isArray(metricsData) && metricsData.length > 0) {
          console.log(
            '[CopilotUsage] Sample metrics data structure:',
            JSON.stringify(metricsData[0], null, 2).substring(0, 500) + '...'
          )
        }

        commit('SET_METRICS', metricsData)
        console.log('[CopilotUsage] Metrics data committed to store')
      } catch (error: unknown) {
        const errorMessage = error instanceof Error ? error.message : 'Failed to fetch metrics'
        console.error('[CopilotUsage] Error fetching metrics:', errorMessage, error)

        // Check for token expiration
        if (axios.isAxiosError(error) && error.response?.status === 401) {
          console.log('[CopilotUsage] Detected authentication error (401) in fetchMetrics')

          // Reset the loading and ready states
          commit('SET_METRICS', null)
          commit('SET_METRICS_READY', false)

          // Handle token expiration
          if (
            error.response.data?.message?.includes('ExpiredSignature') ||
            error.response.data?.message?.includes('Token validation error')
          ) {
            console.log('[CopilotUsage] Token expired, dispatching auth/handleTokenExpired')
            dispatch('auth/handleTokenExpired', null, { root: true })
            return
          }
        }

        commit('SET_METRICS_ERROR', errorMessage)
        console.log('[CopilotUsage] Error committed to store')

        // Log specific error information for better debugging
        if (axios.isAxiosError(error)) {
          console.error('[CopilotUsage] API Error Details:', {
            status: error.response?.status,
            statusText: error.response?.statusText,
            message: error.response?.data?.message || error.message,
            url: error.config?.url,
            headers: {
              // Show header names but not values for security
              request: error.config?.headers ? Object.keys(error.config.headers) : 'No headers',
              response: error.response?.headers ? Object.keys(error.response.headers) : 'No headers'
            }
          })

          // Check specifically for authorization issues with GitHub API
          if (error.response?.status === 401 || error.response?.status === 403) {
            console.error(
              '[CopilotUsage] GitHub API authentication error detected. Please check your token.'
            )

            // Add a more specific error message for users
            commit(
              'SET_METRICS_ERROR',
              'Authentication failed with GitHub. Please check your API token or login again.'
            )
          }
        }
      } finally {
        commit('SET_METRICS_LOADING', false)
        console.log('[CopilotUsage] fetchMetrics action completed')
      }
    },

    // Chat data actions - temporarily use empty data (implement API when ready)
    async fetchChatData({ commit }) {
      commit('SET_CHAT_DATA_LOADING', true)
      commit('SET_CHAT_DATA_ERROR', null)

      try {
        // In the future, replace with actual API call:
        // const chatData = await someApiCall()
        // For now, return an empty object to maintain structure
        const chatData = {}
        commit('SET_CHAT_DATA', chatData)
      } catch (error: unknown) {
        console.error('Error fetching chat data:', error)
        const errorMessage = error instanceof Error ? error.message : 'Failed to fetch chat data'
        commit('SET_CHAT_DATA_ERROR', errorMessage)
      } finally {
        commit('SET_CHAT_DATA_LOADING', false)
      }
    },

    // Seats data actions - currently disabled in legacy code
    async fetchSeats({ commit }) {
      commit('SET_SEATS_DATA_LOADING', true)
      commit('SET_SEATS_DATA_ERROR', null)

      try {
        // This was commented out in the legacy code:
        // const seatsData = await getSeatsApi()
        // For now, return an empty object to maintain structure
        const seatsData = {}
        commit('SET_SEATS_DATA', seatsData)
        // Also set the legacy state for compatibility
        commit('SET_SEATS_READY', true)
      } catch (error: unknown) {
        console.error('Error fetching seats data:', error)
        const errorMessage = error instanceof Error ? error.message : 'Failed to fetch seats data'
        commit('SET_SEATS_DATA_ERROR', errorMessage)
      } finally {
        commit('SET_SEATS_DATA_LOADING', false)
      }
    },

    // API response data actions
    async fetchApiResponseData({ commit }) {
      commit('SET_API_RESPONSE_LOADING', true)
      commit('SET_API_RESPONSE_ERROR', null)

      try {
        // Set null data to indicate no data is available
        commit('SET_API_RESPONSE_DATA', null)
        commit(
          'SET_API_RESPONSE_ERROR',
          'No API response data available. This feature is under development.'
        )
      } catch (error: unknown) {
        console.error('Error fetching API response data:', error)
        const errorMessage =
          error instanceof Error ? error.message : 'Failed to fetch API response data'
        commit('SET_API_RESPONSE_ERROR', errorMessage)
      } finally {
        commit('SET_API_RESPONSE_LOADING', false)
      }
    },

    // Fetch team metrics and members
    async fetchTeamMetrics({ commit }, team: string) {
      console.log(`[CopilotUsage] Starting fetchTeamMetrics action for team "${team}"`)

      // Initialize with empty arrays for data that might fail to load
      let metrics: any[] = []
      let members: any[] = []
      let hasMetricsError = false
      let hasMembersError = false

      // Try to fetch metrics
      try {
        console.log(`[CopilotUsage] Fetching metrics for team "${team}"`)
        metrics = await getTeamMetricsApi(team)
        console.log(
          `[CopilotUsage] Team metrics API call successful for "${team}":`,
          metrics
            ? `Received ${Array.isArray(metrics) ? metrics.length : 1} metrics records`
            : 'Empty metrics'
        )
      } catch (metricsError) {
        console.error(`[CopilotUsage] Error fetching metrics for team "${team}":`, metricsError)
        hasMetricsError = true
        // Don't return here, continue to fetch members
      }

      // Try to fetch members
      try {
        console.log(`[CopilotUsage] Fetching members for team "${team}"`)
        members = await getTeamMembers(team)
        console.log(
          `[CopilotUsage] Team members API call successful for "${team}":`,
          members
            ? `Received ${Array.isArray(members) ? members.length : 1} members records`
            : 'Empty members'
        )
      } catch (membersError) {
        console.error(`[CopilotUsage] Error fetching members for team "${team}":`, membersError)
        hasMembersError = true
        // Continue with available data
      }

      // Create team data with whatever we have
      const teamData = {
        team_tag: team,
        metrics,
        members
      }

      // Add to team data regardless of partial failures
      console.log(
        `[CopilotUsage] Committing team data for "${team}" (metrics: ${!hasMetricsError}, members: ${!hasMembersError})`
      )

      // For new components
      commit('SET_TEAM_DATA', teamData)

      // For legacy components (add to array of team metrics)
      commit('ADD_TEAM_METRICS', teamData)
      console.log(`[CopilotUsage] Team data for "${team}" added to teamMetrics array`)

      // Set appropriate error state if needed
      if (hasMetricsError && hasMembersError) {
        commit(
          'SET_TEAM_DATA_ERROR',
          `We couldn't load any data for this team. Please try again later.`
        )
      } else if (hasMetricsError) {
        commit(
          'SET_TEAM_DATA_ERROR',
          `Metrics data unavailable for ${team}, but team member information is displayed.`
        )
      } else if (hasMembersError) {
        commit(
          'SET_TEAM_DATA_ERROR',
          `Member data unavailable for ${team}, but metrics information is displayed.`
        )
      }

      console.log(`[CopilotUsage] fetchTeamMetrics action completed for team "${team}"`)
    },

    // Fetch metrics for all teams
    async fetchAllTeamMetrics({ commit, state, dispatch }) {
      console.log('[CopilotUsage] Starting fetchAllTeamMetrics action')
      commit('SET_TEAM_DATA_LOADING', true)
      commit('SET_TEAM_DATA_ERROR', null)

      // Always reset teamMetrics to an empty array when starting fetchAllTeamMetrics
      console.log('[CopilotUsage] Resetting teamMetrics to empty array')
      state.teamMetrics = []

      try {
        // Get teams list from state
        const teams = state.teamList || []
        console.log('[CopilotUsage] Teams to process:', teams.length, 'teams')

        if (teams.length === 0) {
          console.log('[CopilotUsage] No teams in state, using fallback team list')
          // Use fallback list of teams instead of exiting with an error
          const fallbackTeams = [
            'cloud_transfer_reviewers',
            'neo-ap',
            'rci-mfv',
            'attendance_dev_reviewers',
            'mf_connected_db_developers',
            'payroll_dev_reviewers',
            'payroll_kotlin_reviewers',
            'social_insurance_dev_reviewers',
            'tax_adjustment_dev_mfj_reviewers'
          ]

          // Use these teams instead
          const fetchTeams = fallbackTeams.map(team => dispatch('fetchTeamMetrics', team))
          console.log('[CopilotUsage] Waiting for all fallback team fetches to complete')

          await Promise.all(fetchTeams)
          console.log('[CopilotUsage] All fallback team fetches completed')

          // Set ready state for both new and legacy components
          commit('SET_TEAM_METRICS_READY', true)
          return
        }

        // Add all teams data first if metrics exist
        if (state.metrics) {
          console.log('[CopilotUsage] Adding "All Teams" entry with existing metrics')
          const allTeamsData = {
            team_tag: 'All Teams',
            metrics: state.metrics
          }

          // Add to teamMetrics array
          state.teamMetrics.push(allTeamsData)
          console.log('[CopilotUsage] "All Teams" entry added to teamMetrics array')
        } else {
          console.log('[CopilotUsage] No metrics data available for "All Teams" entry')
        }

        // Fetch each team's metrics asynchronously
        const teamsToFetch = teams.filter(team => team !== 'All Teams')
        console.log('[CopilotUsage] Starting fetch for', teamsToFetch.length, 'individual teams')

        const fetchTeams = teamsToFetch.map(team => dispatch('fetchTeamMetrics', team))
        console.log('[CopilotUsage] Waiting for all team fetches to complete')

        await Promise.all(fetchTeams)
        console.log('[CopilotUsage] All team fetches completed')

        // Set ready state for both new and legacy components
        commit('SET_TEAM_METRICS_READY', true)
        console.log('[CopilotUsage] Team metrics marked as ready')
      } catch (error) {
        console.error('[CopilotUsage] Error fetching all team metrics:', error)
        commit('SET_TEAM_DATA_ERROR', 'Failed to fetch metrics for all teams')
      } finally {
        commit('SET_TEAM_DATA_LOADING', false)
        console.log('[CopilotUsage] fetchAllTeamMetrics action completed')

        // Add detailed logging about the final state
        if (Array.isArray(state.teamMetrics)) {
          console.log(
            '[CopilotUsage] Final teamMetrics array contains',
            state.teamMetrics.length,
            'teams:',
            state.teamMetrics.map((team: any) => team.team_tag).join(', ')
          )
        } else {
          console.log('[CopilotUsage] Warning: teamMetrics is not an array in final state')
        }
      }
    },

    // Department data actions - temporarily use empty data (implement API when ready)
    async fetchDepartmentMetrics({ commit }) {
      commit('SET_DEPARTMENT_DATA_LOADING', true)
      commit('SET_DEPARTMENT_DATA_ERROR', null)

      try {
        // Mock department data
        const mockDepartments = {
          'Department 1': [
            'cloud_transfer_reviewers',
            'neo-ap',
            'rci-mfv',
            'attendance_dev_reviewers',
            'mf_connected_db_developers',
            'payroll_dev_reviewers',
            'payroll_kotlin_reviewers',
            'social_insurance_dev_reviewers',
            'tax_adjustment_dev_mfj_reviewers'
          ],
          'Department 2': ['rci-mfv', 'attendance_dev_reviewers', 'mf_connected_db_developers'],
          'Department 3': ['payroll_dev_reviewers', 'payroll_kotlin_reviewers'],
          'Department 4': ['social_insurance_dev_reviewers', 'tax_adjustment_dev_mfj_reviewers']
        }

        // Create department data object for the store
        const departmentData = {
          departments: mockDepartments
        }

        console.log(
          '[CopilotUsage] Setting mock department data:',
          Object.keys(mockDepartments).join(', ')
        )
        commit('SET_DEPARTMENT_DATA', departmentData)
      } catch (error: unknown) {
        console.error('Error fetching department metrics:', error)
        const errorMessage =
          error instanceof Error ? error.message : 'Failed to fetch department metrics'
        commit('SET_DEPARTMENT_DATA_ERROR', errorMessage)
      } finally {
        commit('SET_DEPARTMENT_DATA_LOADING', false)
      }
    }
  }
}

export default copilotUsageModule
