<template>
  <v-container fluid class="dora-dashboard">
    <v-row justify="center" align="center">
      <!-- Team Selector & Date Range Picker -->
      <v-col cols="12" md="12">
        <v-row align="center" justify="end">
          <!-- Repository Selector with Search -->
          <v-col cols="12" md="5">
            <v-select
              v-model="selectedRepo"
              :items="repositoriesNames"
              label="Select Repository"
              class="repo-select"
              :loading="isLoadingRepos"
              @change="fetchData"
              item-title="text"
              item-value="value"
              variant="outlined"
              color="primary"
              bg-color="white"
            >
              <template v-slot:append-item>
                <v-list-item
                  v-if="hasMoreRepos && !isLoadingRepos"
                  @click="loadMoreRepositories"
                  class="load-more-item"
                >
                  <v-list-item-title> Load more repositories... </v-list-item-title>
                </v-list-item>
                <v-list-item v-if="isLoadingRepos">
                  <v-list-item-title> Loading... </v-list-item-title>
                </v-list-item>
              </template>
            </v-select>
          </v-col>
          <v-col cols="12" md="3">
            <v-select
              v-model="selectedTimePeriod"
              :items="timePeriods"
              label="Select Time Period"
              class="time-period-select"
              variant="outlined"
              color="primary"
              bg-color="white"
            />
          </v-col>
        </v-row>
      </v-col>

      <!-- No repositories message -->
      <v-col v-if="repositories.length === 0 && !isLoadingRepos" cols="12">
        <v-alert type="info" class="metric-alert">
          No repositories found. Please check your connections and permissions.
        </v-alert>
      </v-col>

      <!-- Loading indicator -->
      <v-col v-if="isLoadingRepos && repositories.length === 0" cols="12">
        <div class="text-center">
          <v-progress-circular indeterminate color="primary" size="64" />
          <div class="mt-2">Loading repositories...</div>
        </div>
      </v-col>

      <!-- Metric Alerts (only show when repository selected) -->
      <v-col v-if="selectedRepo && repositories.length > 0" cols="12">
        <v-alert type="info" class="metric-alert">
          {{ metricAlert }}
        </v-alert>
      </v-col>

      <!-- Metric Cards (only show when repository selected) -->
      <template v-if="selectedRepo && repositories.length > 0">
        <v-col cols="12" md="12">
          <CycleTimeCard :pull-requests="pullRequests" :selected-repo="selectedRepo" />
        </v-col>
        <v-col cols="12" md="6">
          <DeploymentFrequencyCard
            :deployments="deployments"
            :deployment-frequency="deploymentFrequency"
          />
        </v-col>
        <v-col cols="12" md="6">
          <ChangeFailureRateCard :statuses="statuses" />
        </v-col>
        <v-col cols="12" md="6">
          <MTTRCard :statuses="statuses" />
        </v-col>
        <v-col cols="12" md="6">
          <LeadTimeCard :pull-requests="pullRequests" />
        </v-col>
      </template>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import { defineComponent, ref, computed, watch, onMounted } from 'vue'
import DeploymentFrequencyCard from './DeploymentFrequencyCard.vue'
import ChangeFailureRateCard from './ChangeFailureRateCard.vue'
import MTTRCard from './MTTRCard.vue'
import CycleTimeCard from './CycleTimeCard.vue'
import LeadTimeCard from './LeadTimeCard.vue'
import { useStore } from 'vuex'
import config from '@/config'

export default defineComponent({
  name: 'DoraDashboard',
  components: {
    CycleTimeCard,
    DeploymentFrequencyCard,
    ChangeFailureRateCard,
    MTTRCard,
    LeadTimeCard
  },
  setup() {
    const store = useStore()

    // Repositories state from Vuex
    const repositories = computed(() => store.state.DoraData.repositories || [])
    const isLoadingRepos = computed(() => store.state.DoraData.isLoadingRepos)
    const hasMoreRepos = computed(() => store.state.DoraData.hasMoreRepos)
    console.log('repositories', repositories.value)
    const repositoriesNames = computed(() => {
      return repositories.value.map((repo: any) => ({
        text: repo.name,
        value: repo.name
      }))
    })
    console.log('repositoriesNames', repositoriesNames.value)

    // UI state
    const selectedTimePeriod = ref<string>('Weekly')
    const timePeriods = ref<string[]>(['Weekly', 'Monthly', 'Yearly'])
    const selectedRepo = ref<string>('')
    const date = ref({ start: new Date(), end: new Date() })
    const startDate = computed(() => date.value.start)
    const endDate = computed(() => date.value.end)
    const metricAlert = ref(
      'Cycle Time needs focus. Deployment PRs are good. Work on reducing time to restore system functionality after incidents.'
    )

    // Data from store
    const deployments = computed(() => store.state.DoraData.deployments || [])
    const pullRequests = computed(() => store.state.DoraData.pullRequests || [])
    const statuses = computed(() => store.state.DoraData.statuses || [])

    // Computed metrics
    const deploymentFrequency = computed(() => {
      if (!deployments.value.length) return '0/week'
      const totalDeployments = deployments.value.length
      const weeks = Math.max(
        1,
        (endDate.value.getTime() - startDate.value.getTime()) / (1000 * 60 * 60 * 24 * 7)
      )
      return `${(totalDeployments / weeks).toFixed(2)}/week`
    })

    // Methods
    const updateDates = (newDate: any) => {
      date.value.start = newDate.start
      date.value.end = newDate.end
    }

    const fetchData = () => {
      if (!selectedRepo.value) {
        console.log('[DoraDashboard] No repository selected, skipping data fetch')
        return
      }

      console.log(`[DoraDashboard] Fetching data for repository: ${selectedRepo.value}`)
      const owner = config.github.org
      const repo = selectedRepo.value

      // Fetch data for the selected repository
      store.dispatch('DoraData/fetchDeploymentsAndStatuses', { owner, repo })
      store.dispatch('DoraData/fetchPullRequests', { owner, repo })
    }

    const loadMoreRepositories = () => {
      // Use the store action for lazy loading
      store.dispatch('DoraData/loadMoreRepositories')
    }

    // Watchers
    watch(selectedRepo, (newRepo, oldRepo) => {
      // Only fetch data if a repository is explicitly selected by the user
      if (newRepo && newRepo !== oldRepo) {
        fetchData()
      }
    })

    // Watch date changes separately
    watch([startDate, endDate], () => {
      if (selectedRepo.value) {
        fetchData()
      }
    })

    onMounted(() => {
      // Wait for repositories to load, but don't auto-select or fetch data
      let watcherSetup = false
      const repoWatcher = watch(repositories, newRepos => {
        if (!watcherSetup) {
          watcherSetup = true
          return // Skip first run to avoid immediate callback issue
        }

        // Just log that repos are loaded, but don't auto-select or fetch data
        if (newRepos.length > 0) {
          console.log(`[DoraDashboard] ${newRepos.length} repositories loaded`)
          repoWatcher() // Stop watching once we've seen repos are loaded
        }
      })

      // If repos are already loaded, just log it but don't auto-select
      if (repositories.value.length > 0) {
        console.log(`[DoraDashboard] ${repositories.value.length} repositories already loaded`)
        repoWatcher() // Stop watching if we already have repositories
      }
    })

    return {
      // State
      repositories,
      isLoadingRepos,
      hasMoreRepos,
      selectedTimePeriod,
      timePeriods,
      repositoriesNames,
      selectedRepo,
      date,
      startDate,
      endDate,
      metricAlert,

      // Data
      deployments,
      pullRequests,
      statuses,
      deploymentFrequency,

      // Methods
      updateDates,
      fetchData,
      loadMoreRepositories
    }
  }
})
</script>

<style scoped>
.dora-dashboard {
  margin-bottom: 20px;
}

.metric-alert {
  display: flex;
  align-items: center;
  margin-bottom: 20px;
}

.repo-select,
.time-period-select {
  width: 100%;
}

.repo-select :deep(.v-field__input),
.time-period-select :deep(.v-field__input) {
  color: #333333 !important;
  background-color: white;
}

.repo-select :deep(.v-field__outline),
.time-period-select :deep(.v-field__outline) {
  color: #4f46e5;
}

.repo-select :deep(.v-field__field),
.time-period-select :deep(.v-field__field) {
  background-color: white;
}

.repo-select :deep(.v-field--active),
.time-period-select :deep(.v-field--active) {
  background-color: white;
}

.load-more-item {
  color: #4f46e5;
  cursor: pointer;
}

.load-more-item:hover {
  background-color: #f5f5f5;
}
</style>
