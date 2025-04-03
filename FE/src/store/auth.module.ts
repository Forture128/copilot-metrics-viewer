import type { Module } from 'vuex'
import type { RootState } from './index'
import horusService from '../services/HorusService'
import router from '../router'

export interface AuthState {
  isAuthenticated: boolean
  user: {
    userId: number
    organizationId: number
    role: string
  } | null
  token: string | null
  orgId: number | null
}

const authModule: Module<AuthState, RootState> = {
  namespaced: true,

  state: {
    isAuthenticated: false,
    user: null,
    token: null,
    orgId: null
  },

  getters: {
    isAuthenticated: state => state.isAuthenticated,
    user: state => state.user,
    token: state => state.token,
    orgId: state => state.orgId,
    userRoles: state => state.user?.role || null
  },

  mutations: {
    SET_AUTH(state, { token, orgId }) {
      state.isAuthenticated = true
      state.token = token
      state.orgId = orgId
    },

    SET_USER(state, user) {
      state.user = {
        userId: user.user_id,
        organizationId: user.organization_id,
        role: user.role
      }
    },

    CLEAR_AUTH(state) {
      state.isAuthenticated = false
      state.user = null
      state.token = null
      state.orgId = null
    }
  },

  actions: {
    async login({ commit, dispatch }, { username, password }) {
      const response = await horusService.login(username, password)

      if (response && response.access_token) {
        // The HorusService already stores the token and orgId
        commit('SET_AUTH', {
          token: response.access_token,
          orgId: horusService.getOrgId()
        })

        // Fetch user info
        await dispatch('fetchUserInfo')

        return true
      }

      return false
    },

    async fetchUserInfo({ commit, dispatch }) {
      try {
        if (horusService.isAuthenticated()) {
          const userInfo = await horusService.getCurrentUser()
          commit('SET_USER', userInfo)
        }
      } catch (error: any) {
        // If we get an error fetching user info, it could be due to an expired token
        if (error.response?.status === 401) {
          // Token might be expired, handle accordingly
          dispatch('handleTokenExpired')
        }
        throw error
      }
    },

    logout({ commit }) {
      horusService.clearAuth()
      commit('CLEAR_AUTH')
    },

    checkAuth({ commit, dispatch }) {
      if (horusService.loadAuthFromStorage()) {
        commit('SET_AUTH', {
          token: localStorage.getItem('authToken'),
          orgId: horusService.getOrgId()
        })

        // Attempt to fetch user info, which will validate the token
        dispatch('fetchUserInfo').catch(error => {
          console.error('Error fetching user info during auth check:', error)
          // If token validation fails, handle it
          if (error.response?.status === 401) {
            dispatch('handleTokenExpired')
            return false
          }
        })
        return true
      }

      return false
    },

    // New action to handle token expiration
    handleTokenExpired({ commit }) {
      // Clear auth data
      horusService.clearAuth()
      commit('CLEAR_AUTH')

      // Redirect to login page
      if (router.currentRoute.value.path !== '/login') {
        router.push('/login')
      }
    }
  }
}

export default authModule
