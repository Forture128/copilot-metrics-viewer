<template>
  <AppLayout>
    <div class="developer-metrics-view">
      <div class="view-header">
        <h1 class="view-title">Developer Metrics</h1>
        <p class="view-description">
          View and analyze individual developer performance metrics, including productivity, code
          quality, and Copilot usage patterns.
        </p>
      </div>

      <!-- Filters -->
      <div class="filters-container">
        <div class="filter-group">
          <label for="time-period">Time Period</label>
          <select
            id="time-period"
            v-model="timePeriod"
            class="filter-select"
            @change="updateTimeFrame"
          >
            <option value="7d">Last 7 Days</option>
            <option value="30d">Last 30 Days</option>
            <option value="90d">Last 90 Days</option>
            <option value="ytd">Year to Date</option>
            <option value="custom">Custom Range</option>
          </select>
        </div>

        <!-- Custom Date Range Selection (only shown when custom is selected) -->
        <div v-if="timePeriod === 'custom'" class="date-range-filters">
          <div class="filter-group">
            <label for="start-date">From</label>
            <input
              type="date"
              id="start-date"
              v-model="customStartDate"
              class="date-input"
              @change="updateCustomDateRange"
            />
          </div>
          <div class="filter-group">
            <label for="end-date">To</label>
            <input
              type="date"
              id="end-date"
              v-model="customEndDate"
              class="date-input"
              @change="updateCustomDateRange"
              :max="today"
            />
          </div>
        </div>

        <div class="filter-group">
          <label for="team-filter">Team</label>
          <select
            id="team-filter"
            v-model="selectedTeam"
            class="filter-select"
            @change="onTeamChange"
          >
            <option value="all">All Teams</option>
            <option v-for="team in teams" :key="team" :value="team">{{ team }}</option>
          </select>
        </div>

        <div class="filter-group">
          <label for="developer-filter">Developer</label>
          <select
            id="developer-filter"
            v-model="selectedDeveloper"
            class="filter-select"
            @change="onDeveloperChange"
          >
            <option value="all">All Developers</option>
            <option v-for="dev in developersInStore" :key="dev" :value="dev">{{ dev }}</option>
          </select>
        </div>
      </div>

      <!-- Loading State -->
      <div v-if="isLoading" class="loading-container">
        <div class="loading-spinner" />
        <p>Loading developer metrics...</p>
      </div>

      <!-- Error State -->
      <div v-else-if="error" class="error-container">
        <div class="notification-banner warning">
          <div class="notification-icon">⚠️</div>
          <div class="notification-message">{{ error }}</div>
        </div>
      </div>

      <!-- Content when data is available -->
      <div v-else>
        <!-- Metrics Cards -->
        <div class="metrics-cards">
          <!-- Copilot Usage Card -->
          <div class="metric-card">
            <h3 class="metric-title">Copilot Usage</h3>
            <div class="metric-content">
              <div class="metric-value">{{ averageCopilotUsage }}%</div>
              <div class="metric-description">Average acceptance rate of Copilot suggestions</div>
            </div>
            <div class="metric-trend positive">
              <span class="trend-arrow">↑</span>
              <span class="trend-value">5.3%</span>
              <span class="trend-period">vs previous period</span>
            </div>
          </div>

          <!-- Code Contributions Card -->
          <div class="metric-card">
            <h3 class="metric-title">Code Contributions</h3>
            <div class="metric-content">
              <div class="metric-value">{{ codeContributions }}</div>
              <div class="metric-description">Lines of code contributed</div>
            </div>
            <div class="metric-trend positive">
              <span class="trend-arrow">↑</span>
              <span class="trend-value">12.7%</span>
              <span class="trend-period">vs previous period</span>
            </div>
          </div>

          <!-- Pull Requests Card -->
          <div class="metric-card">
            <h3 class="metric-title">Pull Requests</h3>
            <div class="metric-content">
              <div class="metric-value">{{ pullRequestCount }}</div>
              <div class="metric-description">PRs submitted and merged</div>
            </div>
            <div class="metric-trend negative">
              <span class="trend-arrow">↓</span>
              <span class="trend-value">3.2%</span>
              <span class="trend-period">vs previous period</span>
            </div>
          </div>

          <!-- Time Saved Card -->
          <div class="metric-card">
            <h3 class="metric-title">Time Saved</h3>
            <div class="metric-content">
              <div class="metric-value">{{ timeSaved }}</div>
              <div class="metric-description">Estimated hours saved with Copilot</div>
            </div>
            <div class="metric-trend positive">
              <span class="trend-arrow">↑</span>
              <span class="trend-value">8.5%</span>
              <span class="trend-period">vs previous period</span>
            </div>
          </div>
        </div>

        <!-- Detailed Charts -->
        <div class="charts-container">
          <div class="chart-card">
            <h3 class="chart-title">Developer Productivity Trends</h3>
            <div class="chart-placeholder">Chart visualization will be implemented here</div>
          </div>

          <div class="chart-card">
            <h3 class="chart-title">Copilot Suggestions by Language</h3>
            <div class="chart-placeholder">Chart visualization will be implemented here</div>
          </div>
        </div>

        <!-- Developer Comparison Table -->
        <div class="comparison-table-container">
          <h3 class="section-title">Developer Comparison</h3>

          <table class="comparison-table">
            <thead>
              <tr>
                <th>Developer</th>
                <th>Copilot Usage %</th>
                <th>PRs Merged</th>
                <th>Lines Added</th>
                <th>Lines Removed</th>
                <th>Est. Hours Saved</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="dev in developerData" :key="dev.name">
                <td>{{ dev.name }}</td>
                <td>{{ dev.copilotUsage }}%</td>
                <td>{{ dev.prsMerged }}</td>
                <td>{{ dev.linesAdded }}</td>
                <td>{{ dev.linesRemoved }}</td>
                <td>{{ dev.hoursSaved }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useStore } from 'vuex'
import AppLayout from '@/layouts/AppLayout.vue'

// Store
const store = useStore()

// State
const timePeriod = ref('30d')
const teams = computed(() => store.getters['DeveloperMetrics/getTeamsList'] || [])

// Date handling for filters
const today = computed(() => new Date().toISOString().split('T')[0])
const customStartDate = ref(
  new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString().split('T')[0]
)
const customEndDate = ref(today.value)

// Computed properties from store
const selectedDeveloper = computed({
  get: () => store.getters['DeveloperMetrics/getSelectedDeveloper'] || 'all',
  set: value => {
    if (value !== 'all') {
      store.dispatch('DeveloperMetrics/setSelectedDeveloper', value)
    } else {
      store.commit('DeveloperMetrics/SET_SELECTED_DEVELOPER', null)
    }
  }
})

const selectedTeam = computed({
  get: () => store.getters['DeveloperMetrics/getSelectedTeam'] || 'all',
  set: value => {
    if (value !== 'all') {
      store.dispatch('DeveloperMetrics/setSelectedTeam', value)
    } else {
      store.commit('DeveloperMetrics/SET_SELECTED_TEAM', null)
    }
  }
})

const developersInStore = computed(() => {
  return store.getters['DeveloperMetrics/getDevelopersList'] || []
})

const isLoading = computed(() => store.getters['DeveloperMetrics/isLoading'])
const error = computed(() => store.getters['DeveloperMetrics/getError'])

// Metric values from store
const averageCopilotUsage = computed(() => store.getters['DeveloperMetrics/getAverageCopilotUsage'])
const codeContributions = computed(() => store.getters['DeveloperMetrics/getCodeContributions'])
const pullRequestCount = computed(() => store.getters['DeveloperMetrics/getPullRequestCount'])
const timeSaved = computed(() => store.getters['DeveloperMetrics/getTimeSaved'])

// Mock developer data for comparison table (will be updated with real data)
const developerData = computed(() => {
  const metrics = store.getters['DeveloperMetrics/getDeveloperMetrics']

  if (metrics && metrics.commit_metrics && metrics.pr_metrics) {
    // If we have real data for the selected developer, include it
    const realDeveloperData = {
      name: metrics.username,
      copilotUsage: Math.round(metrics.contribution_score * 100) / 100,
      prsMerged: metrics.pr_metrics.merged_count,
      linesAdded: metrics.commit_metrics.lines_added,
      linesRemoved: metrics.commit_metrics.lines_removed,
      hoursSaved:
        Math.round(
          ((metrics.commit_metrics.lines_added + metrics.commit_metrics.lines_removed) / 100) *
            10 *
            10
        ) / 10
    }

    // Return with additional mock data
    return [
      realDeveloperData,
      {
        name: 'Developer 2',
        copilotUsage: 72.8,
        prsMerged: 12,
        linesAdded: 987,
        linesRemoved: 654,
        hoursSaved: 6.2
      },
      {
        name: 'Developer 3',
        copilotUsage: 65.3,
        prsMerged: 9,
        linesAdded: 754,
        linesRemoved: 421,
        hoursSaved: 5.1
      },
      {
        name: 'Developer 4',
        copilotUsage: 78.9,
        prsMerged: 8,
        linesAdded: 632,
        linesRemoved: 345,
        hoursSaved: 4.8
      },
      {
        name: 'Developer 5',
        copilotUsage: 69.1,
        prsMerged: 4,
        linesAdded: 321,
        linesRemoved: 213,
        hoursSaved: 2.7
      }
    ]
  }

  // Return mock data if no real data is available
  return [
    {
      name: 'Developer 1',
      copilotUsage: 85.2,
      prsMerged: 15,
      linesAdded: 1245,
      linesRemoved: 843,
      hoursSaved: 8.5
    },
    {
      name: 'Developer 2',
      copilotUsage: 72.8,
      prsMerged: 12,
      linesAdded: 987,
      linesRemoved: 654,
      hoursSaved: 6.2
    },
    {
      name: 'Developer 3',
      copilotUsage: 65.3,
      prsMerged: 9,
      linesAdded: 754,
      linesRemoved: 421,
      hoursSaved: 5.1
    },
    {
      name: 'Developer 4',
      copilotUsage: 78.9,
      prsMerged: 8,
      linesAdded: 632,
      linesRemoved: 345,
      hoursSaved: 4.8
    },
    {
      name: 'Developer 5',
      copilotUsage: 69.1,
      prsMerged: 4,
      linesAdded: 321,
      linesRemoved: 213,
      hoursSaved: 2.7
    }
  ]
})

// Methods
const updateTimeFrame = () => {
  let startDate: string, endDate: string
  const today = new Date()

  switch (timePeriod.value) {
    case '7d':
      startDate = formatDateISO(new Date(today.getTime() - 7 * 24 * 60 * 60 * 1000))
      endDate = formatDateISO(today)
      break
    case '30d':
      startDate = formatDateISO(new Date(today.getTime() - 30 * 24 * 60 * 60 * 1000))
      endDate = formatDateISO(today)
      break
    case '90d':
      startDate = formatDateISO(new Date(today.getTime() - 90 * 24 * 60 * 60 * 1000))
      endDate = formatDateISO(today)
      break
    case 'ytd':
      startDate = `${today.getFullYear()}-01-01`
      endDate = formatDateISO(today)
      break
    case 'custom':
      startDate = customStartDate.value
      endDate = customEndDate.value
      break
    default:
      startDate = formatDateISO(new Date(today.getTime() - 30 * 24 * 60 * 60 * 1000))
      endDate = formatDateISO(today)
  }

  // Only dispatch if we have a selected developer
  store.dispatch('DeveloperMetrics/setTimeFrame', { startDate, endDate })
}

// Update time frame when custom date range changes
const updateCustomDateRange = () => {
  if (timePeriod.value === 'custom') {
    updateTimeFrame()
  }
}

// Format date to ISO 8601 (YYYY-MM-DD)
const formatDateISO = (date: Date): string => {
  return date.toISOString().split('T')[0]
}

const onDeveloperChange = () => {
  // No need to do anything here - the computed property handles this
}

const onTeamChange = () => {
  // No need for additional actions here
}

// Fetch data on mount
onMounted(async () => {
  try {
    // Initialize the module and fetch teams
    await store.dispatch('DeveloperMetrics/initialize')

    // In development mode, load mock data for testing
    if (import.meta.env.DEV) {
      await store.dispatch('DeveloperMetrics/loadMockData')
    }

    // Set initial time frame without triggering developer data fetch yet
    const today = new Date()
    const startDate = formatDateISO(new Date(today.getTime() - 30 * 24 * 60 * 60 * 1000))
    const endDate = formatDateISO(today)
    store.dispatch('DeveloperMetrics/setTimeFrame', { startDate, endDate })
  } catch (error) {
    console.error('Error initializing developer metrics view:', error)
  }
})

// Watch for changes in developer selection - fetch data only when developer changes
watch(selectedDeveloper, (newDeveloper, oldDeveloper) => {
  if (newDeveloper && newDeveloper !== 'all' && newDeveloper !== oldDeveloper) {
    // Fetch data for the selected developer
    store.dispatch('DeveloperMetrics/fetchDeveloperData', newDeveloper)
  }
})

// Watch for time frame changes to fetch data when needed
watch(
  () => store.getters['DeveloperMetrics/getSelectedTimeFrame'],
  (newTimeFrame, oldTimeFrame) => {
    const currDeveloper = store.getters['DeveloperMetrics/getSelectedDeveloper']
    // Only fetch if we have a developer selected and time frame has changed
    if (
      currDeveloper &&
      currDeveloper !== 'all' &&
      (newTimeFrame.startDate !== oldTimeFrame?.startDate ||
        newTimeFrame.endDate !== oldTimeFrame?.endDate)
    ) {
      store.dispatch('DeveloperMetrics/fetchDeveloperData', currDeveloper)
    }
  },
  { deep: true }
)
</script>

<style scoped>
.developer-metrics-view {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.view-header {
  margin-bottom: 1rem;
}

.view-title {
  font-size: 1.75rem;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.view-description {
  color: #6b7280;
  margin: 0.5rem 0 0;
  max-width: 800px;
  line-height: 1.5;
}

/* Loading State */
.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  margin: 1rem 0;
}

.loading-spinner {
  border: 4px solid rgba(0, 0, 0, 0.1);
  border-radius: 50%;
  border-top: 4px solid #3b82f6;
  width: 40px;
  height: 40px;
  animation: spin 1s linear infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}

/* Error State */
.error-container {
  padding: 1rem;
  margin: 1rem 0;
}

.notification-banner {
  display: flex;
  align-items: center;
  padding: 1rem;
  border-radius: 0.5rem;
  background-color: #fef2f2;
  border-left: 4px solid #f87171;
}

.notification-banner.warning {
  background-color: #fef2f2;
  border-left-color: #f87171;
}

.notification-icon {
  margin-right: 0.75rem;
  font-size: 1.25rem;
}

.notification-message {
  color: #b91c1c;
  font-size: 0.875rem;
}

/* Filters */
.filters-container {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  padding: 1rem;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.filter-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.filter-group label {
  font-size: 0.875rem;
  font-weight: 500;
  color: #6b7280;
}

.filter-select {
  padding: 0.5rem;
  border: 1px solid #e5e7eb;
  border-radius: 4px;
  min-width: 150px;
}

/* Metrics Cards */
.metrics-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 1rem;
}

.metric-card {
  padding: 1.25rem;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.metric-title {
  font-size: 1rem;
  font-weight: 600;
  color: #4b5563;
  margin: 0;
}

.metric-content {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.metric-value {
  font-size: 2rem;
  font-weight: 700;
  color: #111827;
}

.metric-description {
  font-size: 0.875rem;
  color: #6b7280;
}

.metric-trend {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
}

.metric-trend.positive {
  color: #10b981;
}

.metric-trend.negative {
  color: #ef4444;
}

.trend-period {
  color: #9ca3af;
  margin-left: 0.25rem;
}

/* Charts */
.charts-container {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(500px, 1fr));
  gap: 1.5rem;
}

.chart-card {
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  padding: 1.25rem;
}

.chart-title {
  font-size: 1rem;
  font-weight: 600;
  color: #4b5563;
  margin: 0 0 1rem;
}

.chart-placeholder {
  height: 300px;
  background-color: #f9fafb;
  border: 1px dashed #d1d5db;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #9ca3af;
  font-style: italic;
}

/* Comparison Table */
.comparison-table-container {
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  padding: 1.25rem;
}

.section-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: #4b5563;
  margin: 0 0 1rem;
}

.comparison-table {
  width: 100%;
  border-collapse: collapse;
}

.comparison-table th,
.comparison-table td {
  padding: 0.75rem 1rem;
  text-align: left;
  border-bottom: 1px solid #e5e7eb;
}

.comparison-table th {
  font-weight: 500;
  color: #6b7280;
  background-color: #f9fafb;
}

.comparison-table tbody tr:hover {
  background-color: #f9fafb;
}

/* Date Range Filters */
.date-range-filters {
  display: flex;
  gap: 1rem;
}

.date-input {
  padding: 0.5rem;
  border: 1px solid #e5e7eb;
  border-radius: 4px;
  min-width: 150px;
  font-size: 0.875rem;
  color: #4b5563;
}
</style>
