<template>
  <AppLayout>
    <div class="team-metrics-view">
      <div class="view-header">
        <h1 class="view-title">Team Metrics</h1>
        <p class="view-description">
          Analyze Copilot usage and effectiveness across different teams in your organization.
        </p>
      </div>

      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner" />
        <span>Loading team metrics...</span>
      </div>

      <div v-else>
        <!-- Team & department selection controls -->
        <div class="selection-controls">
          <div class="selection-group">
            <label for="department-select" class="select-label">Department</label>
            <select
              id="department-select"
              v-model="selectedDepartment"
              class="select-input"
              :disabled="isLoading"
              @change="handleDepartmentChange"
            >
              <option value="all">All Departments</option>
              <option v-for="(teams, dept) in departments" :key="dept" :value="dept">
                {{ dept }}
              </option>
            </select>
          </div>

          <button class="refresh-button" @click="refreshTeamData" :disabled="isLoading">
            Refresh Data
          </button>
        </div>

        <!-- Replace the error state with a non-blocking notification -->
        <div v-if="error" class="notification-banner warning">
          <div class="notification-icon">⚠️</div>
          <p class="notification-message">{{ error }}</p>
        </div>

        <div v-if="!teamMetrics || teamMetrics.length === 0" class="empty-state">
          <p>No team data available. Please check your connection or try again later.</p>
        </div>

        <div v-if="hasTeamData || error" class="metrics-display">
          <!-- Team metrics summary -->
          <div class="metrics-summary">
            <div class="metric-card">
              <h3 class="metric-title">Total Members</h3>
              <p class="metric-value">{{ totalMembersInDepartment }}</p>
            </div>
            <div class="metric-card">
              <h3 class="metric-title">Teams in Department</h3>
              <p class="metric-value">{{ filteredTeams.length }}</p>
            </div>
            <div class="metric-card">
              <h3 class="metric-title">Selected Department</h3>
              <p class="metric-value">
                {{ selectedDepartment === 'all' ? 'All Departments' : selectedDepartment }}
              </p>
            </div>
          </div>

          <TeamMetricsViewer
            v-if="teamMetrics.length > 0 || error"
            :teams="filteredTeams"
            :metrics="formattedTeamMetrics"
            :available-departments="Object.keys(departments)"
            :selected-department="selectedDepartment"
            class="metrics-component"
          />
        </div>
      </div>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useStore } from 'vuex'
import AppLayout from '@/layouts/AppLayout.vue'
import TeamMetricsViewer from '@/components/TeamMetricsViewer.vue'
import type { TeamMetrics } from '@/model/Metrics'
import type { Members } from '@/model/Members'

// Define a simplified interface that matches the store data structure
interface TeamMetricsItem {
  team_tag: string
  metrics?: any[]
  members?: any[]
}

// Store
const store = useStore()

// Local state
const isLoadingSpecificTeam = ref(false)
const selectedDepartment = ref('all')

// Computed properties
const departments = computed(() => store.state.CopilotUsage?.departments || {})

const filteredTeams = computed(() => {
  return selectedDepartment.value === 'all'
    ? availableTeams.value
    : departments.value[selectedDepartment.value] || []
})

const teamMetrics = computed(() => {
  return (store.state.CopilotUsage?.teamMetrics || []) as TeamMetricsItem[]
})

const isLoading = computed(() => {
  return store.state.CopilotUsage?.teamDataLoading || isLoadingSpecificTeam.value
})

const error = computed(() => {
  return store.state.CopilotUsage?.teamDataError || null
})

const availableTeams = computed(() => {
  return store.state.CopilotUsage?.teamList || []
})

// Format team metrics for display
const formattedTeamMetrics = computed(() => {
  // Filter team metrics based on the selected department
  let filteredMetrics = teamMetrics.value

  if (selectedDepartment.value !== 'all') {
    const departmentTeams = departments.value[selectedDepartment.value] || []
    filteredMetrics = teamMetrics.value.filter((tm: TeamMetricsItem) =>
      departmentTeams.includes(tm.team_tag)
    )
  }

  // Format each team's metrics
  return filteredMetrics.map((tm: TeamMetricsItem) => {
    const formattedMembers = tm.members ? tm.members.map(m => ({ ...m })) : []
    return {
      team_tag: tm.team_tag,
      members: formattedMembers as Members[],
      metrics: tm.metrics || []
    } as TeamMetrics
  })
})

// Determine if we have team data to display
const hasTeamData = computed(() => {
  if (!teamMetrics.value || teamMetrics.value.length === 0) return false

  if (selectedDepartment.value !== 'all') {
    const departmentTeams = departments.value[selectedDepartment.value] || []
    return teamMetrics.value.some((tm: TeamMetricsItem) => departmentTeams.includes(tm.team_tag))
  }

  return teamMetrics.value.length > 0
})

// Calculate total members across all teams in the selected department
const totalMembersInDepartment = computed(() => {
  const teamsToCount = formattedTeamMetrics.value
  let allMembers: any[] = []

  // Collect all members from all teams
  teamsToCount.forEach(team => {
    if (team.members && Array.isArray(team.members)) {
      team.members.forEach(memberObj => {
        if (memberObj.members && Array.isArray(memberObj.members)) {
          allMembers = allMembers.concat(memberObj.members)
        }
      })
    }
  })

  // Filter for unique members by login
  const uniqueLogins = new Set<string>()
  const uniqueMembers = allMembers.filter(member => {
    if (!member.login || uniqueLogins.has(member.login)) {
      return false
    }
    uniqueLogins.add(member.login)
    return true
  })

  return uniqueMembers.length
})

// Methods - moved here before the watcher that uses them
const fetchAllTeamData = async () => {
  try {
    await store.dispatch('CopilotUsage/fetchAllTeamMetrics')
  } catch (err) {
    console.error('[TeamMetricsView] Error fetching all team metrics:', err)
  }
}

const handleDepartmentChange = () => {
  if (selectedDepartment.value !== 'all') {
    // If specific department selected, fetch data for all teams in that department
    refreshDepartmentData()
  } else {
    // If "All Departments" selected, fetch all team data
    fetchAllTeamData()
  }
}

const refreshDepartmentData = async () => {
  if (selectedDepartment.value === 'all') {
    return fetchAllTeamData()
  }

  isLoadingSpecificTeam.value = true

  try {
    // Get teams in the selected department
    const departmentTeams = departments.value[selectedDepartment.value] || []

    // Fetch data for all teams in the department
    const promises = departmentTeams.map((team: string) =>
      store.dispatch('CopilotUsage/fetchTeamMetrics', team)
    )

    await Promise.all(promises)
  } catch (err) {
    console.error(`[TeamMetricsView] Error fetching department metrics:`, err)
  } finally {
    isLoadingSpecificTeam.value = false
  }
}

const refreshTeamData = () => {
  // Always refresh department data first
  store.dispatch('CopilotUsage/fetchDepartmentMetrics')

  if (selectedDepartment.value !== 'all') {
    refreshDepartmentData()
  } else {
    fetchAllTeamData()
  }
}

// Watchers - now after the function declarations
watch(
  departments,
  newDepartments => {
    // Select the first department as default when departments data becomes available
    if (selectedDepartment.value === 'all' && Object.keys(newDepartments).length > 0) {
      selectedDepartment.value = Object.keys(newDepartments)[0]
      handleDepartmentChange()
    }
  },
  { immediate: true }
)

// Lifecycle hooks
onMounted(() => {
  // Fetch department data first
  store.dispatch('CopilotUsage/fetchDepartmentMetrics')

  // Then fetch all team metrics
  fetchAllTeamData()
})
</script>

<style scoped>
.team-metrics-view {
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

.selection-controls {
  display: flex;
  align-items: flex-end;
  margin-bottom: 1.5rem;
  gap: 1rem;
  flex-wrap: wrap;
}

.selection-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  min-width: 200px;
}

.select-label {
  font-weight: 500;
  font-size: 0.875rem;
  color: #4b5563;
}

.select-input {
  padding: 0.5rem;
  border: 1px solid #e5e7eb;
  border-radius: 0.375rem;
  background-color: white;
  min-width: 200px;
  height: 38px;
}

.select-input:disabled {
  background-color: #f3f4f6;
  cursor: not-allowed;
}

.refresh-button {
  padding: 0.5rem 1rem;
  background-color: #4f46e5;
  color: white;
  border-radius: 0.375rem;
  font-weight: 500;
  height: 38px;
  align-self: flex-end;
  cursor: pointer;
  transition: background-color 0.2s;
}

.refresh-button:hover {
  background-color: #4338ca;
}

.metrics-summary {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.metric-card {
  background-color: white;
  padding: 1.25rem;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.metric-title {
  font-size: 0.875rem;
  color: #6b7280;
  margin: 0 0 0.5rem 0;
}

.metric-value {
  font-size: 1.875rem;
  font-weight: 600;
  color: #111827;
  margin: 0;
}

.loading-state,
.empty-state,
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  background: white;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  text-align: center;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid rgba(79, 70, 229, 0.2);
  border-radius: 50%;
  border-top-color: #4f46e5;
  animation: spin 1s ease-in-out infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.error-state {
  flex-direction: row;
  text-align: left;
  gap: 1rem;
  background-color: #fee2e2;
  color: #b91c1c;
}

.error-icon {
  font-size: 1.5rem;
}

.error-title {
  margin: 0 0 0.25rem;
  font-weight: 600;
}

.error-message {
  margin: 0;
  color: #b91c1c;
}

.metrics-component {
  background: white;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  padding: 1.5rem;
}

.notification-banner {
  display: flex;
  align-items: center;
  padding: 0.75rem 1rem;
  border-radius: 0.375rem;
  margin-bottom: 1rem;
  font-size: 0.875rem;
}

.notification-banner.warning {
  background-color: #fffbeb;
  border: 1px solid #fcd34d;
  color: #92400e;
}

.notification-icon {
  margin-right: 0.75rem;
  font-size: 1rem;
}

.notification-message {
  margin: 0;
}

.empty-state {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  background: white;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  text-align: left;
  gap: 1rem;
  margin-bottom: 1rem;
}

.error-content {
  flex: 1;
}

.error-title {
  margin: 0 0 0.25rem;
  font-weight: 600;
  color: #333;
}

.error-message {
  margin: 0;
  color: #6b7280;
}
</style>
