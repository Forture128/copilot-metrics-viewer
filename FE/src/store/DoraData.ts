import GitHubService from '@/services/DoraService'

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
  isLoadingRepos: false,
  hasMoreRepos: true,
  owner: 'moneyforward',
  perPage: 100
}

const mutations = {
  SET_REPOSITORIES(state: { repositories: any }, repositories: any) {
    state.repositories = repositories
  },
  ADD_REPOSITORIES(state: { repositories: any }, repositories: any) {
    state.repositories = [...state.repositories, ...repositories]
  },
  SET_SELECTED_REPO(state: { selectedRepo: any }, selectedRepo: any) {
    state.selectedRepo = selectedRepo
  },
  SET_PULL_REQUESTS(state: { pullRequests: any }, pullRequests: any) {
    state.pullRequests = pullRequests
  },
  SET_COMMITS(state: { commits: any }, commits: any) {
    state.commits = commits
  },
  SET_WORKFLOW_RUNS(state: { workflowRuns: any }, workflowRuns: any) {
    state.workflowRuns = workflowRuns
  },
  SET_ISSUES(state: { issues: any }, issues: any) {
    state.issues = issues
  },
  SET_STATUSES(state: { statuses: any }, statuses: any) {
    state.statuses = statuses
  },
  SET_DEPLOYMENTS(state: { deployments: any }, deployments: any) {
    state.deployments = deployments
  },
  SET_CURRENT_PAGE(state: { currentPage: number }, page: number) {
    state.currentPage = page
  },
  SET_LOADING_REPOS(state: { isLoadingRepos: boolean }, isLoading: boolean) {
    state.isLoadingRepos = isLoading
  },
  SET_HAS_MORE_REPOS(state: { hasMoreRepos: boolean }, hasMore: boolean) {
    state.hasMoreRepos = hasMore
  },
  SET_OWNER(state: { owner: string | null }, owner: string | null) {
    state.owner = owner
  }
}

const actions = {
  /**
   * Fetch repositories for the current page
   */
  async fetchRepositories({
    commit,
    state
  }: {
    commit: (mutation: string, payload: any) => void
    state: any
  }) {
    commit('SET_LOADING_REPOS', true)
    try {
      const repositories = await GitHubService.getRepositories(state.currentPage, state.perPage)

      if (state.currentPage === 1) {
        commit('SET_REPOSITORIES', repositories)
      } else {
        commit('ADD_REPOSITORIES', repositories)
      }

      // Check if we've reached the end
      commit('SET_HAS_MORE_REPOS', repositories.length === state.perPage)
    } catch (error) {
      console.error('[DoraData] Error fetching repositories:', error)
    } finally {
      commit('SET_LOADING_REPOS', false)
    }
  },

  /**
   * Fetch repositories for a specific owner
   */
  async fetchRepositoriesByOwner(
    {
      commit,
      state
    }: {
      commit: (mutation: string, payload: any) => void
      state: any
    },
    owner: string = 'moneyforward'
  ) {
    if (!owner) return

    commit('SET_LOADING_REPOS', true)
    commit('SET_OWNER', owner)

    try {
      const repositories = await GitHubService.getRepositoriesByOwner(
        owner,
        state.currentPage,
        state.perPage
      )

      if (state.currentPage === 1) {
        commit('SET_REPOSITORIES', repositories)
      } else {
        commit('ADD_REPOSITORIES', repositories)
      }

      // Check if we've reached the end
      commit('SET_HAS_MORE_REPOS', repositories.length === state.perPage)
    } catch (error) {
      console.error(`[DoraData] Error fetching repositories for ${owner}:`, error)
    } finally {
      commit('SET_LOADING_REPOS', false)
    }
  },

  /**
   * Load more repositories (lazy loading)
   */
  async loadMoreRepositories({
    commit,
    state,
    dispatch
  }: {
    commit: (mutation: string, payload: any) => void
    state: any
    dispatch: (action: string, payload?: any) => Promise<any>
  }) {
    // Don't fetch if already loading or no more data
    if (state.isLoadingRepos || !state.hasMoreRepos) return

    // Increment page number
    commit('SET_CURRENT_PAGE', state.currentPage + 1)

    // Use the appropriate fetch method based on whether an owner is specified
    await dispatch('fetchRepositoriesByOwner', 'moneyforward')
  },

  /**
   * Reset repository fetching state and fetch first page
   */
  async resetAndFetchRepositories({
    commit,
    dispatch
  }: {
    commit: (mutation: string, payload: any) => void
    dispatch: (action: string, payload?: any) => Promise<any>
  }) {
    // Reset pagination and state
    commit('SET_CURRENT_PAGE', 1)
    commit('SET_REPOSITORIES', [])
    commit('SET_HAS_MORE_REPOS', true)

    // Always fetch repositories from moneyforward organization
    commit('SET_OWNER', 'moneyforward')
    await dispatch('fetchRepositoriesByOwner', 'moneyforward')
  },

  async fetchPullRequests(
    { commit }: { commit: (mutation: string, payload: any) => void },
    { owner, repo }: { owner: string; repo: string }
  ) {
    const params = {
      state: 'closed',
      base: 'main'
    }
    const pullRequests = await GitHubService.getPullRequests(owner, repo, params)
    commit('SET_PULL_REQUESTS', pullRequests)
  },
  async fetchCommits(
    { commit }: { commit: (mutation: string, payload: any) => void },
    { owner, repo }: { owner: string; repo: string }
  ) {
    const commits = await GitHubService.getCommits(owner, repo)
    commit('SET_COMMITS', commits)
  },
  async fetchWorkflowRuns(
    { commit }: { commit: (mutation: string, payload: any) => void },
    { owner, repo }: { owner: string; repo: string }
  ) {
    const workflowRuns = await GitHubService.getWorkflowRuns(owner, repo)
    commit('SET_WORKFLOW_RUNS', workflowRuns)
  },
  async fetchIssues(
    { commit }: { commit: (mutation: string, payload: any) => void },
    { owner, repo }: { owner: string; repo: string }
  ) {
    const issues = await GitHubService.getIssues(owner, repo)
    commit('SET_ISSUES', issues)
  },
  async fetchDeploymentsAndStatuses(
    { commit }: { commit: (mutation: string, payload: any) => void },
    { owner, repo }: { owner: string; repo: string }
  ) {
    const deployments = await GitHubService.getDeployments(owner, repo)
    commit('SET_DEPLOYMENTS', deployments)
    // Fetch statuses for each deployment
    const statuses = await Promise.all(
      deployments.map((deployment: any) =>
        GitHubService.getDeploymentStatuses(owner, repo, deployment.id)
      )
    )
    commit('SET_STATUSES', statuses.flat())
  }
}

export default {
  namespaced: true,
  state,
  mutations,
  actions
}
