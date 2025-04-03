<template>
  <AppLayout>
    <div class="department-metrics-view">
      <div class="view-header">
        <h1 class="view-title">Department Metrics</h1>
        <p class="view-description">
          View and analyze department performance metrics and statistics.
        </p>
      </div>

      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner" />
        <span>Loading department metrics...</span>
      </div>

      <div v-else-if="error" class="error-state">
        <div class="error-icon">⚠️</div>
        <div class="error-content">
          <h3 class="error-title">Error Loading Data</h3>
          <p class="error-message">{{ error }}</p>
        </div>
      </div>

      <div v-else class="metrics-content">
        <!-- Department Overview Cards -->
        <div class="metrics-grid">
          <div class="metric-card">
            <h3>Total Departments</h3>
            <div class="metric-value">{{ departments.length }}</div>
          </div>
          <div class="metric-card">
            <h3>Total Users</h3>
            <div class="metric-value">{{ totalUsers }}</div>
          </div>
          <div class="metric-card">
            <h3>Average Users per Department</h3>
            <div class="metric-value">{{ averageUsersPerDepartment }}</div>
          </div>
          <div class="metric-card">
            <h3>Active Departments</h3>
            <div class="metric-value">{{ activeDepartments }}</div>
          </div>
        </div>

        <!-- Department List with Metrics -->
        <div class="departments-section">
          <h2>Department Performance</h2>
          <div class="departments-table">
            <table>
              <thead>
                <tr>
                  <th>Department</th>
                  <th>Users</th>
                  <th>Roles</th>
                  <th>Activity Score</th>
                  <th>Last Updated</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="dept in departments" :key="dept.id">
                  <td>{{ dept.name }}</td>
                  <td>{{ dept.user_count || 0 }}</td>
                  <td>{{ dept.role_count || 0 }}</td>
                  <td>
                    <div class="activity-score">
                      <div
                        class="score-bar"
                        :style="{ width: `${((dept.user_count || 0) / maxUsers) * 100}%` }"
                      />
                      <span>{{ calculateActivityScore(dept) }}%</span>
                    </div>
                  </td>
                  <td>{{ formatDate(dept.updated_at) }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- Department Distribution Chart -->
        <div class="chart-section">
          <h2>Department Size Distribution</h2>
          <div class="chart-container">
            <div v-for="(count, size) in departmentSizeDistribution" :key="size" class="chart-bar">
              <div class="bar-fill" :style="{ height: `${(count / departments.length) * 100}%` }" />
              <div class="bar-label">{{ count }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import AppLayout from '@/layouts/AppLayout.vue'
import horusService from '@/services/HorusService'
import type { DepartmentResponse, DepartmentListResponse } from '@/types/horus-api.types'

// State
const isLoading = ref(false)
const error = ref<string | null>(null)
const departments = ref<DepartmentResponse[]>([])

// Computed properties
const totalUsers = computed(() => {
  return departments.value.reduce((sum, dept) => sum + (dept.user_count || 0), 0)
})

const averageUsersPerDepartment = computed(() => {
  if (departments.value.length === 0) return 0
  return Math.round(totalUsers.value / departments.value.length)
})

const activeDepartments = computed(() => {
  return departments.value.filter(dept => (dept.user_count || 0) > 0).length
})

const maxUsers = computed(() => {
  return Math.max(...departments.value.map(dept => dept.user_count || 0), 1)
})

const departmentSizeDistribution = computed(() => {
  const distribution: Record<string, number> = {
    '1-5': 0,
    '6-10': 0,
    '11-20': 0,
    '21-50': 0,
    '50+': 0
  }

  departments.value.forEach(dept => {
    const userCount = dept.user_count || 0
    if (userCount <= 5) distribution['1-5']++
    else if (userCount <= 10) distribution['6-10']++
    else if (userCount <= 20) distribution['11-20']++
    else if (userCount <= 50) distribution['21-50']++
    else distribution['50+']++
  })

  return distribution
})

// Methods
const fetchDepartments = async () => {
  isLoading.value = true
  error.value = null
  try {
    const response: DepartmentListResponse = await horusService.listDepartments()
    departments.value = response.items
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to fetch departments'
  } finally {
    isLoading.value = false
  }
}

const calculateActivityScore = (dept: DepartmentResponse): number => {
  const userCount = dept.user_count || 0
  return Math.round((userCount / maxUsers.value) * 100)
}

const formatDate = (dateString: string): string => {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

// Lifecycle hooks
onMounted(() => {
  fetchDepartments()
})
</script>

<style scoped>
.department-metrics-view {
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

.metrics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.metric-card {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.metric-card h3 {
  margin: 0 0 0.5rem;
  font-size: 0.875rem;
  color: #6b7280;
}

.metric-value {
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
}

.departments-section {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.departments-section h2 {
  margin: 0 0 1rem;
  font-size: 1.25rem;
  color: #333;
}

.departments-table {
  overflow-x: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th,
td {
  padding: 0.75rem;
  text-align: left;
  border-bottom: 1px solid #e5e7eb;
}

th {
  font-weight: 600;
  color: #4b5563;
  background: #f9fafb;
}

.activity-score {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.score-bar {
  height: 8px;
  background: #4f46e5;
  border-radius: 4px;
  transition: width 0.3s ease;
}

.chart-section {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.chart-section h2 {
  margin: 0 0 1rem;
  font-size: 1.25rem;
  color: #333;
}

.chart-container {
  display: flex;
  align-items: flex-end;
  gap: 1rem;
  height: 200px;
  padding: 1rem 0;
}

.chart-bar {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
}

.bar-fill {
  width: 100%;
  background: #4f46e5;
  border-radius: 4px;
  transition: height 0.3s ease;
}

.bar-label {
  font-size: 0.875rem;
  color: #6b7280;
}

.loading-state,
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
</style>
