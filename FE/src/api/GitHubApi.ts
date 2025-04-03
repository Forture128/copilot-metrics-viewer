//Make a call to the GitHub API to get Copilot Metrics, the API is https://api.github.com/orgs/toussaintt/copilot/usage
//Add the header Accept: application/vnd.github+json to the request
//Add also the Authorization: Bearer <token> header where <token> is hardcoded for now
//Also add X-GitHub-Api-Version: 2022-11-28 header
//Return the response from the API

import axios from 'axios'

import { Metrics } from '../model/Metrics'
import organizationMockedResponse from '../assets/organization_response_sample.json'
import enterpriseMockedResponse from '../assets/enterprise_response_sample.json'
import teamMockedResponse from '../assets/teams_response.json'
import config from '../config'
import { Team } from '@/model/Teams'
import { Members } from '@/model/Members'
import store from '@/store' // Import the store to get auth token
import ToastService from '@/services/ToastService'
import router from '@/router'

// Helper to get the current authentication token
const getAuthToken = (): string => {
  // Try to get token from Vuex store first
  const storeToken = store.getters['auth/token']
  if (storeToken) {
    return storeToken
  }

  // Fall back to config token if available
  return config.github.token || ''
}

// Helper to handle authentication errors
const handleAuthError = async (error: any) => {
  if (axios.isAxiosError(error) && error.response?.status === 401) {
    console.error(
      'Authentication error:',
      error.response.data?.message || 'Token validation failed'
    )

    // Dispatch logout action
    await store.dispatch('auth/handleTokenExpired')

    // Redirect to login if not already there
    if (router.currentRoute.value.path !== '/login') {
      router.push('/login')
    }

    // Use ToastService instead of direct useToast call
    ToastService.error('Your session has expired. Please login again.')
  }
  throw error
}

export const getMetricsApi = async (): Promise<Metrics[]> => {
  let response
  let metricsData
  console.log('Config ', config)

  if (config.mockedData) {
    console.log('Using mock data. Check VUE_APP_MOCKED_DATA variable.')
    response =
      config.scope.type === 'organization' ? organizationMockedResponse : enterpriseMockedResponse
    metricsData = response.map((item: any) => new Metrics(item))
  } else {
    try {
      const token = getAuthToken()
      console.log('Using authentication token:', token ? 'Token available' : 'No token available')

      response = await axios.get(`${config.github.apiUrl}/copilot/usage`, {
        headers: {
          Accept: 'application/vnd.github+json',
          Authorization: `Bearer ${token}`,
          'X-GitHub-Api-Version': '2022-11-28'
        }
      })
      console.log('Response ', response)
      metricsData = response.data.map((item: any) => new Metrics(item))
    } catch (error) {
      await handleAuthError(error)
      throw error
    }
  }
  return metricsData
}

export const getTeams = async (): Promise<Team[]> => {
  let response
  let teamData
  const token = getAuthToken()

  // If config mockdata is enabled, return a mocked response
  if (config.mockedData) {
    response = teamMockedResponse
    // map with Team object
    teamData = response.map((item: any) => new Team(item))
  } else {
    try {
      response = await axios.get(`${config.github.apiUrl}/teams`, {
        headers: {
          Accept: 'application/vnd.github+json',
          Authorization: `Bearer ${token}`,
          'X-GitHub-Api-Version': '2022-11-28'
        }
      })
      teamData = response.data.map((item: any) => new Team(item))
    } catch (error) {
      await handleAuthError(error)
      throw error
    }
  }
  return teamData
}

export const getTeamMetricsApi = async (team_tag: string): Promise<Metrics[]> => {
  let response
  let metricsData
  const token = getAuthToken()

  if (config.mockedData) {
    response =
      config.scope.type === 'organization' ? organizationMockedResponse : enterpriseMockedResponse
    metricsData = response.map((item: any) => new Metrics(item))
  } else {
    try {
      response = await axios.get(`${config.github.apiUrl}/team/${team_tag}/copilot/metrics`, {
        headers: {
          Accept: 'application/vnd.github+json',
          Authorization: `Bearer ${token}`,
          'X-GitHub-Api-Version': '2022-11-28'
        }
      })

      if (response.status === 200) {
        metricsData = response.data.map((item: any) => new Metrics(item))
      } else {
        console.error(`Error: Received status code ${response.status}`)
        metricsData = []
      }
    } catch (error) {
      await handleAuthError(error)
      console.error('Error fetching team metrics:', error)
      metricsData = []
    }
  }
  return metricsData
}

// Get the team members from the GitHub API
export const getTeamMembers = async (team_tag: string): Promise<Members[]> => {
  let response
  let membersData: Members[] = []
  const token = getAuthToken()

  if (config.mockedData) {
    response = organizationMockedResponse
    membersData = response.map((item: any) => new Members(item))
  } else {
    try {
      response = await axios.get(`${config.github.apiUrl}/teams/${team_tag}/members`, {
        headers: {
          Accept: 'application/vnd.github+json',
          Authorization: `Bearer ${token}`,
          'X-GitHub-Api-Version': '2022-11-28'
        }
      })

      if (response.status === 200) {
        if (Array.isArray(response.data)) {
          membersData = response.data.map((item: any) => new Members(item))
        } else {
          console.error('Error: Response data is not an array', response.data)
          membersData = []
        }
      } else {
        console.error(`Error: Received status code ${response.status}`)
        membersData = []
      }
    } catch (error) {
      await handleAuthError(error)
      console.error('Error fetching team members:', error)
      membersData = []
    }
  }
  return membersData
}
