import { getSeatsApi } from '@/api/ExtractSeats';
import { getMetricsApi, getTeamMembers, getTeamMetricsApi } from '@/api/GitHubApi';
import { createStore } from 'vuex';

const state = {
  metrics: [],
  seats: [],
  teamList: ["All Teams", "cloud_transfer_reviewers", "neo-ap", "rci-mfv"],
  teamMetrics: [],
  teamMetricsReady: false,
  apiError: null,
  metricsReady: false,
  seatsReady: false,
};

const mutations = {
  SET_METRICS(state: { metrics: any; }, metrics: any) {
    state.metrics = metrics;
  },
  SET_SEATS(state: { seats: any; }, seats: any) {
    state.seats = seats;
  },
  SET_TEAM_LIST(state: { teamList: any; }, teamList: any) {
    state.teamList = teamList;
  },
  ADD_TEAM_METRICS(state: { teamMetrics: any[]; }, teamMetrics: any) {
    state.teamMetrics.push(teamMetrics);
  },
  SET_TEAM_METRICS_READY(state: { teamMetricsReady: any; }, ready: any) {
    state.teamMetricsReady = ready;
  },
  SET_API_ERROR(state: { apiError: any; }, error: any) {
    state.apiError = error;
  },
  SET_METRICS_READY(state: { metricsReady: any; }, ready: any) {
    state.metricsReady = ready;
  },
  SET_SEATS_READY(state: { seatsReady: any; }, ready: any) {
    state.seatsReady = ready;
  },
};

const actions = {
  async fetchMetrics({ commit }: { commit: (mutation: string, payload?: any) => void }) {
    commit('SET_METRICS_READY', false);
    try {
      const metrics = await getMetricsApi();
      commit('SET_METRICS', metrics);
      commit('SET_API_ERROR', null);
    } catch (error) {
      commit('SET_API_ERROR', error);
    } finally {
      commit('SET_METRICS_READY', true);
    }
  },
  async fetchSeats({ commit }: { commit: (mutation: string, payload?: any) => void }) {
    commit('SET_SEATS_READY', false);
    try {
      const seats = await getSeatsApi();
      commit('SET_SEATS', seats);
      commit('SET_API_ERROR', null);
    } catch (error) {
      commit('SET_API_ERROR', error);
    } finally {
      commit('SET_SEATS_READY', true);
    }
  },
  async fetchTeamMetrics({ commit }: any, team: string) {
    try {
      const metrics = await getTeamMetricsApi(team);
      const members = await getTeamMembers(team);
      const teamData = {
        team_tag: team,
        metrics: metrics,
        members: members,
      };
      commit('ADD_TEAM_METRICS', teamData);
    } catch (error) {
      commit('SET_API_ERROR', error);
    }
  },
  async fetchAllTeamMetrics({ commit, state, dispatch }: { commit: (mutation: string, payload?: any) => void, state: any, dispatch: (action: string, payload?: any) => Promise<any> }) {
    try {
      const allTeamsData = {
        team_tag: "All Teams",
        metrics: state.metrics,
      };
      commit('ADD_TEAM_METRICS', allTeamsData);

      for (const team of state.teamList) {
        if (team !== "All Teams") {
          await dispatch('fetchTeamMetrics', team);
        }
      }
      commit('SET_TEAM_METRICS_READY', true);
    } catch (error) {
      commit('SET_API_ERROR', error);
    }
  },
};

const getters = {
  metrics: (state: { metrics: any; }) => state.metrics,
  seats: (state: { seats: any; }) => state.seats,
  teamList: (state: { teamList: any; }) => state.teamList,
  teamMetrics: (state: { teamMetrics: any; }) => state.teamMetrics,
  teamMetricsReady: (state: { teamMetricsReady: any; }) => state.teamMetricsReady,
  apiError: (state: { apiError: any; }) => state.apiError,
  metricsReady: (state: { metricsReady: any; }) => state.metricsReady,
  seatsReady: (state: { seatsReady: any; }) => state.seatsReady,
};

export default createStore({
  state,
  mutations,
  actions,
  getters,
});