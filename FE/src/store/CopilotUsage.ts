import { getSeatsApi } from '@/api/ExtractSeats';
import { getMetricsApi, getTeamMembers, getTeamMetricsApi } from '@/api/GitHubApi';
import { Commit, Dispatch } from 'vuex';

interface State {
  metrics: any[];
  seats: any[];
  teamList: string[];
  teamMetrics: any[];
  teamMetricsReady: boolean;
  metricsReady: boolean;
  seatsReady: boolean;
}

// Initial state
const state: State = {
  metrics: [],
  seats: [],
  teamList: ["cloud_transfer_reviewers", "neo-ap", "rci-mfv", "attendance_dev_reviewers", "mf_connected_db_developers", "payroll_dev_reviewers", "payroll_kotlin_reviewers", "social_insurance_dev_reviewers", "tax_adjustment_dev_mfj_reviewers"],
  teamMetrics: [],
  teamMetricsReady: false,
  metricsReady: false,
  seatsReady: false,
};

// Mutations
const mutations = {
  SET_METRICS(state: State, metrics: any) {
    state.metrics = metrics;
  },
  SET_SEATS(state: State, seats: any) {
    state.seats = seats;
  },
  SET_TEAM_LIST(state: State, teamList: string[]) {
    state.teamList = teamList;
  },
  ADD_TEAM_METRICS(state: State, teamMetrics: any) {
    state.teamMetrics.push(teamMetrics);
  },
  SET_TEAM_METRICS_READY(state: State, ready: boolean) {
    state.teamMetricsReady = ready;
  },
  SET_METRICS_READY(state: State, ready: boolean) {
    state.metricsReady = ready;
  },
  SET_SEATS_READY(state: State, ready: boolean) {
    state.seatsReady = ready;
  },
};

// Actions
const actions = {
  // Centralized data fetch for metrics or seats
  async fetchData(
    { commit }: { commit: Commit },
    { apiCall, mutation, readyMutation }: { apiCall: () => Promise<any>, mutation: string, readyMutation: string }
  ) {
    commit(readyMutation, false);
    try {
      const data = await apiCall();
      commit(mutation, data);
    } finally {
      commit(readyMutation, true);
    }
  },

  // Fetch metrics from GitHub API
  async fetchMetrics({ commit }: { commit: Commit }) {
    await actions.fetchData({ commit }, {
      apiCall: getMetricsApi,
      mutation: 'SET_METRICS',
      readyMutation: 'SET_METRICS_READY'
    });
  },

  // Fetch seat data
  async fetchSeats({ commit }: { commit: Commit }) {
    await actions.fetchData({ commit }, {
      apiCall: getSeatsApi,
      mutation: 'SET_SEATS',
      readyMutation: 'SET_SEATS_READY'
    });
  },

  // Fetch team metrics and members
  async fetchTeamMetrics({ commit }: { commit: Commit }, team: string) {
    try {
      const [metrics, members] = await Promise.all([getTeamMetricsApi(team), getTeamMembers(team)]);
      commit('ADD_TEAM_METRICS', { team_tag: team, metrics, members });
    } catch (error) {
      console.error(`Error fetching metrics for team ${team}: ${error}`);
    }
  },

  // Fetch metrics for all teams
  async fetchAllTeamMetrics({ commit, state, dispatch }: { commit: Commit, state: State, dispatch: Dispatch }) {
    try {
      // Add all teams data
      commit('ADD_TEAM_METRICS', { team_tag: "All Teams", metrics: state.metrics });

      // Fetch each team's metrics asynchronously
      const fetchTeams = state.teamList.filter(team => team !== "All Teams").map(team =>
        dispatch('fetchTeamMetrics', team)
      );
      await Promise.all(fetchTeams);

      commit('SET_TEAM_METRICS_READY', true);
    } catch (error) {
      console.error(`Error fetching team metrics: ${error}`);
    }
  },
};

// Getters
const getters = {
  metrics: (state: State) => state.metrics,
  seats: (state: State) => state.seats,
  teamList: (state: State) => state.teamList,
  teamMetrics: (state: State) => state.teamMetrics,
  teamMetricsReady: (state: State) => state.teamMetricsReady,
  metricsReady: (state: State) => state.metricsReady,
  seatsReady: (state: State) => state.seatsReady,
};

// Create and export the Vuex module
export default {
  namespaced: true,
  state,
  mutations,
  actions,
  getters,
};