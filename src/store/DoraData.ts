import { createStore } from 'vuex';
import GitHubService from '@/services/DoraService';

const state = {
  repositories: [],
  selectedRepo: null,
  pullRequests: [],
  commits: [],
  workflowRuns: [],
  issues: [],
  statuses: [],
  deployments: [],
  currentPage: 1,
};

const mutations = {
  SET_REPOSITORIES(state: { repositories: any; }, repositories: any) {
    state.repositories = repositories;
  },
  ADD_REPOSITORIES(state: { repositories: any; }, repositories: any) {
    state.repositories = [...state.repositories, ...repositories];
  },
  SET_SELECTED_REPO(state: { selectedRepo: any; }, selectedRepo: any) {
    state.selectedRepo = selectedRepo;
  },
  SET_PULL_REQUESTS(state: { pullRequests: any; }, pullRequests: any) {
    state.pullRequests = pullRequests;
  },
  SET_COMMITS(state: { commits: any; }, commits: any) {
    state.commits = commits;
  },
  SET_WORKFLOW_RUNS(state: { workflowRuns: any; }, workflowRuns: any) {
    state.workflowRuns = workflowRuns;
  },
  SET_ISSUES(state: { issues: any; }, issues: any) {
    state.issues = issues;
  },
  SET_STATUSES(state: { statuses: any; }, statuses: any) {
    state.statuses = statuses;
  },
  SET_DEPLOYMENTS(state: { deployments: any; }, deployments: any) {
    state.deployments = deployments;
  },
  SET_CURRENT_PAGE(state: { currentPage: number; }, page: number) {
    state.currentPage = page;
  },
};

const actions = {
  async fetchRepositories({ commit, state }: { commit: (mutation: string, payload: any) => void, state: any }) {
    const repositories = await GitHubService.getRepositories(state.currentPage);
    if (state.currentPage === 1) {
      commit('SET_REPOSITORIES', repositories);
    } else {
      commit('ADD_REPOSITORIES', repositories);
    }
  },
  async fetchPullRequests({ commit }: { commit: (mutation: string, payload: any) => void }, { owner, repo }: { owner: string, repo: string }) {
    const params = {
      state: 'closed',
      base: 'main',
    };
    const pullRequests = await GitHubService.getPullRequests(owner, repo, params);
    commit('SET_PULL_REQUESTS', pullRequests);
  },
  async fetchCommits({ commit }: { commit: (mutation: string, payload: any) => void }, { owner, repo }: { owner: string, repo: string }) {
    const commits = await GitHubService.getCommits(owner, repo);
    commit('SET_COMMITS', commits);
  },
  async fetchWorkflowRuns({ commit }: { commit: (mutation: string, payload: any) => void }, { owner, repo }: { owner: string, repo: string }) {
    const workflowRuns = await GitHubService.getWorkflowRuns(owner, repo);
    commit('SET_WORKFLOW_RUNS', workflowRuns);
  },
  async fetchIssues({ commit }: { commit: (mutation: string, payload: any) => void }, { owner, repo }: { owner: string, repo: string }) {
    const issues = await GitHubService.getIssues(owner, repo);
    commit('SET_ISSUES', issues);
  },
  async fetchDeploymentsAndStatuses({ commit }: { commit: (mutation: string, payload: any) => void }, { owner, repo }: { owner: string, repo: string }) {
    const deployments = await GitHubService.getDeployments(owner, repo);
    commit('SET_DEPLOYMENTS', deployments);
    // Fetch statuses for each deployment
    const statuses = await Promise.all(deployments.map((deployment: any) => GitHubService.getDeploymentStatuses(owner, repo, deployment.id)));
    commit('SET_STATUSES', statuses.flat());
  },
  async loadMoreRepositories({ commit, state }: { commit: (mutation: string, payload: any) => void, state: any }) {
    commit('SET_CURRENT_PAGE', state.currentPage + 1);
    const repositories = await GitHubService.getRepositories(state.currentPage);
    commit('ADD_REPOSITORIES', repositories);
  },
};

export default createStore({
  state,
  mutations,
  actions,
});