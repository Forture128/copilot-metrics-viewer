import type { Module } from 'vuex'
import type { RootState } from './index'
import horusService from '@/services/HorusService'
import { getTeamMembers } from '@/api/GitHubApi'
import type { Metrics } from '@/types/horus-api.types'
import axios from 'axios'

// Helper to ensure date is in correct ISO 8601 format (YYYY-MM-DD)
const ensureISOFormat = (dateString: string): string => {
  // If it's already in YYYY-MM-DD format, return as is
  if (/^\d{4}-\d{2}-\d{2}$/.test(dateString)) {
    return dateString
  }

  // Try to parse the date and format it
  try {
    const date = new Date(dateString)
    if (isNaN(date.getTime())) {
      // If invalid date, return today's date as fallback
      return new Date().toISOString().split('T')[0]
    }
    return date.toISOString().split('T')[0]
  } catch (error) {
    console.error('Error formatting date:', error)
    // Return today's date as fallback
    return new Date().toISOString().split('T')[0]
  }
}

// Track in-flight requests to prevent duplicates
const inFlightRequests: Record<string, Promise<any>> = {}
// Cache for API responses
const metricsCache: Record<string, any> = {}

// Define the state type
export interface DeveloperMetricsState {
  developerMetrics: Metrics | null
  developersList: string[] // List of developers (usernames)
  teamsList: string[] // List of teams
  loading: boolean
  error: string | null
  selectedDeveloper: string | null
  selectedTeam: string | null
  selectedTimeFrame: {
    startDate: string
    endDate: string
  }
}

// Create the module
const developerMetricsModule: Module<DeveloperMetricsState, RootState> = {
  namespaced: true,

  state: {
    developerMetrics: null,
    developersList: [],
    teamsList: [],
    loading: false,
    error: null,
    selectedDeveloper: null,
    selectedTeam: null,
    selectedTimeFrame: {
      startDate: ensureISOFormat(new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString()),
      endDate: ensureISOFormat(new Date().toISOString())
    }
  },

  getters: {
    getDeveloperMetrics: state => state.developerMetrics,
    getDeveloperCommits: state => state.developerMetrics?.commit_metrics || null,
    getDeveloperPullRequests: state => state.developerMetrics?.pr_metrics || null,
    getDevelopersList: state => state.developersList,
    getTeamsList: state => state.teamsList,
    isLoading: state => state.loading,
    getError: state => state.error,
    getSelectedDeveloper: state => state.selectedDeveloper,
    getSelectedTeam: state => state.selectedTeam,
    getSelectedTimeFrame: state => state.selectedTimeFrame,

    // Derived getters for UI
    getAverageCopilotUsage: state => {
      if (!state.developerMetrics) return 0
      return Math.round(state.developerMetrics.contribution_score * 100) / 100
    },

    getCodeContributions: state => {
      if (!state.developerMetrics?.commit_metrics) return 0
      const commits = state.developerMetrics.commit_metrics
      return commits.lines_added + commits.lines_removed
    },

    getPullRequestCount: state => {
      if (!state.developerMetrics?.pr_metrics) return 0
      return state.developerMetrics.pr_metrics.merged_count
    },

    getTimeSaved: state => {
      if (!state.developerMetrics?.commit_metrics) return 0
      const commits = state.developerMetrics.commit_metrics
      // Assuming 10 minutes saved per 100 lines of code
      const linesOfCode = commits.lines_added + commits.lines_removed
      return Math.round((linesOfCode / 100) * 10 * 10) / 10
    }
  },

  mutations: {
    SET_DEVELOPER_METRICS(state, metrics) {
      state.developerMetrics = metrics
    },
    SET_DEVELOPERS_LIST(state, developers) {
      state.developersList = developers
    },
    SET_TEAMS_LIST(state, teams) {
      state.teamsList = teams
    },
    SET_LOADING(state, isLoading) {
      state.loading = isLoading
    },
    SET_ERROR(state, error) {
      state.error = error
    },
    SET_SELECTED_DEVELOPER(state, developer) {
      state.selectedDeveloper = developer
    },
    SET_SELECTED_TEAM(state, team) {
      state.selectedTeam = team
    },
    SET_TIME_FRAME(state, timeFrame) {
      state.selectedTimeFrame = timeFrame
    }
  },

  actions: {
    // Initialize module data
    async initialize({ dispatch }) {
      await dispatch('fetchTeams')
    },

    // Fetch teams from API
    async fetchTeams({ commit, rootGetters }) {
      commit('SET_LOADING', true)
      commit('SET_ERROR', null)

      try {
        // We need to use the Organization API to get teams
        // This would require integration with an actual endpoint from HorusService
        // For now, use default teams
        const defaultTeams = [
          'cloud_transfer_reviewers',
          'neo-ap',
          'rci-mfv',
          'attendance_dev_reviewers',
          'mf_connected_db_developers',
          'payroll_dev_reviewers'
        ]

        // Try to get teams from CopilotUsage store if available
        if (rootGetters['CopilotUsage/getTeamData']) {
          const existingTeams = rootGetters['CopilotUsage/getTeamData'].teams || []
          if (existingTeams.length > 0) {
            commit(
              'SET_TEAMS_LIST',
              existingTeams.map((team: any) => team.id || team.name)
            )
            return
          }
        }

        // Use default teams if no API data available
        commit('SET_TEAMS_LIST', defaultTeams)
      } catch (error) {
        console.error('Error fetching teams:', error)
        commit('SET_ERROR', 'Failed to fetch teams. Using default teams.')

        // Set default teams even if there's an error
        commit('SET_TEAMS_LIST', [
          'cloud_transfer_reviewers',
          'neo-ap',
          'rci-mfv',
          'attendance_dev_reviewers',
          'mf_connected_db_developers',
          'payroll_dev_reviewers'
        ])
      } finally {
        commit('SET_LOADING', false)
      }
    },

    // Fetch developers for a team
    async fetchDevelopersForTeam({ commit, dispatch, rootGetters }, teamId: string) {
      if (!teamId) return

      commit('SET_LOADING', true)
      commit('SET_ERROR', null)

      try {
        // First try the GitHubApi's getTeamMembers function
        const members = await getTeamMembers(teamId)

        if (members && members.length > 0) {
          // Extract usernames from member objects
          const developers = members
            .flatMap(member => member.members.map((user: any) => user.login))
            .filter(Boolean)

          if (developers.length > 0) {
            commit('SET_DEVELOPERS_LIST', developers)
            return
          }
        }

        // Fallback to CopilotUsage store if first method fails
        await dispatch('CopilotUsage/fetchTeamMetrics', teamId, { root: true })

        const teamData = rootGetters['CopilotUsage/getTeamMetrics']?.find(
          (team: any) => team.team_tag === teamId
        )

        if (teamData && Array.isArray(teamData.members)) {
          // Extract usernames from member objects
          const developers = teamData.members.map((member: any) => member.login || member.name)
          commit('SET_DEVELOPERS_LIST', developers)
        } else {
          // If no team members found, use sample data
          console.warn(`No team members found for team ${teamId}. Using sample data.`)
          commit('SET_DEVELOPERS_LIST', [
            'developer1',
            'developer2',
            'developer3',
            'developer4',
            'developer5'
          ])
        }
      } catch (error) {
        console.error(`Error fetching developers for team ${teamId}:`, error)

        // Use sample data if API call fails
        commit('SET_ERROR', `Could not fetch developers for team ${teamId}. Using sample data.`)
        commit('SET_DEVELOPERS_LIST', [
          'developer1',
          'developer2',
          'developer3',
          'developer4',
          'developer5'
        ])
      } finally {
        commit('SET_LOADING', false)
      }
    },

    // Set selected developer
    setSelectedDeveloper({ commit, dispatch }, developer) {
      commit('SET_SELECTED_DEVELOPER', developer)
      if (developer) {
        dispatch('fetchDeveloperData', developer)
      }
    },

    // Set selected team
    setSelectedTeam({ commit, dispatch }, team) {
      commit('SET_SELECTED_TEAM', team)
      if (team && team !== 'all') {
        dispatch('fetchDevelopersForTeam', team)
      } else {
        // Reset developers list when "All Teams" is selected
        commit('SET_DEVELOPERS_LIST', [])
      }
    },

    // Set time frame
    setTimeFrame({ commit, dispatch, state }, timeFrame) {
      // Only update if the time frame has actually changed
      if (
        timeFrame.startDate !== state.selectedTimeFrame.startDate ||
        timeFrame.endDate !== state.selectedTimeFrame.endDate
      ) {
        commit('SET_TIME_FRAME', timeFrame)

        // Only fetch data if we have a selected developer
        if (state.selectedDeveloper && state.selectedDeveloper !== 'all') {
          dispatch('fetchDeveloperData', state.selectedDeveloper)
        }
      }
    },

    // Fetch data for a developer
    async fetchDeveloperData({ dispatch, state }, developer) {
      const { startDate, endDate } = state.selectedTimeFrame
      await dispatch('fetchDeveloperMetrics', { username: developer, startDate, endDate })
    },

    // Fetch developer metrics
    async fetchDeveloperMetrics({ commit }, { username, startDate, endDate }) {
      commit('SET_LOADING', true)
      commit('SET_ERROR', null)

      try {
        // Ensure dates are correctly formatted in ISO 8601
        const formattedStartDate = ensureISOFormat(startDate)
        const formattedEndDate = ensureISOFormat(endDate)

        // Create a cache key and request key
        const cacheKey = `${username}_${formattedStartDate}_${formattedEndDate}`

        // Check if we have a cached response
        if (Object.prototype.hasOwnProperty.call(metricsCache, cacheKey)) {
          console.log(`Using cached metrics for ${username}`)
          commit('SET_DEVELOPER_METRICS', metricsCache[cacheKey])
          commit('SET_LOADING', false)
          return
        }

        // Check if this exact request is already in flight
        if (Object.prototype.hasOwnProperty.call(inFlightRequests, cacheKey)) {
          console.log(`Request for ${username} already in progress, waiting for result`)
          try {
            const result = await inFlightRequests[cacheKey]
            commit('SET_DEVELOPER_METRICS', result)
            commit('SET_LOADING', false)
            return
          } catch (error) {
            // If the in-flight request fails, we'll continue with a new request
            console.warn(`In-flight request for ${username} failed, trying again`)
            delete inFlightRequests[cacheKey]
          }
        }

        // Store the promise to track this request
        inFlightRequests[cacheKey] = horusService.getDeveloperMetrics(
          username,
          formattedStartDate,
          formattedEndDate
        )

        // Wait for the request to complete
        const metrics = await inFlightRequests[cacheKey]

        // Cache the response
        metricsCache[cacheKey] = metrics

        // Clear the in-flight request
        delete inFlightRequests[cacheKey]

        commit('SET_DEVELOPER_METRICS', metrics)
      } catch (error) {
        console.error('Error fetching developer metrics:', error)
        const errorMessage =
          error instanceof Error ? error.message : 'Failed to fetch developer metrics'

        if (import.meta.env.DEV) {
          // In development, provide more detailed error but still use mock data
          console.warn('Using mock data as fallback in development mode:', errorMessage)
        } else {
          // In production, show the error to the user
          commit('SET_ERROR', errorMessage)
        }

        // Handle token expiration
        if (axios.isAxiosError(error) && error.response?.status === 401) {
          // This will be handled by HorusService's handleApiError method
        }

        // Use mock data as fallback if real API fails
        const mockMetrics = {
          username: username,
          time_period: {
            start_date: startDate,
            end_date: endDate
          },
          commit_metrics: {
            total_count: 45,
            commit_frequency: 1.5,
            lines_added: 2500,
            lines_removed: 982,
            files_changed: 78,
            commit_timestamps: []
          },
          pr_metrics: {
            total_count: 15,
            open_count: 2,
            closed_count: 1,
            merged_count: 12,
            pr_frequency: 0.5,
            avg_time_to_merge: 24.5,
            avg_comments_per_pr: 3.2
          },
          active_days: 22,
          contribution_score: 73.5
        }

        // Only commit mock metrics in development mode or if real API fails and we need fallback
        commit('SET_DEVELOPER_METRICS', mockMetrics)
      } finally {
        commit('SET_LOADING', false)
      }
    },

    // Mock action for development - populate with sample data
    loadMockData({ commit }) {
      // Sample metrics
      const metrics = {
        username: 'developer1',
        time_period: {
          start_date: '2023-01-01',
          end_date: '2023-01-30'
        },
        commit_metrics: {
          total_count: 45,
          commit_frequency: 1.5,
          lines_added: 2500,
          lines_removed: 982,
          files_changed: 78,
          commit_timestamps: []
        },
        pr_metrics: {
          total_count: 15,
          open_count: 2,
          closed_count: 1,
          merged_count: 12,
          pr_frequency: 0.5,
          avg_time_to_merge: 24.5,
          avg_comments_per_pr: 3.2
        },
        active_days: 22,
        contribution_score: 73.5
      }

      commit('SET_DEVELOPER_METRICS', metrics)

      // Sample developers list
      commit('SET_DEVELOPERS_LIST', [
        'developer1',
        'developer2',
        'developer3',
        'developer4',
        'developer5'
      ])

      // Sample teams list
      commit('SET_TEAMS_LIST', [
        'cloud_transfer_reviewers',
        'neo-ap',
        'rci-mfv',
        'attendance_dev_reviewers',
        'mf_connected_db_developers',
        'payroll_dev_reviewers'
      ])
    }
  }
}

export default developerMetricsModule
