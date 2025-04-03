<template>
  <div class="departments-metrics-viewer">
    <h2 class="title">Department Performance Metrics</h2>

    <div
      v-if="
        !departmentData || !departmentData.departments || departmentData.departments.length === 0
      "
      class="empty-state"
    >
      No department data available.
    </div>

    <div v-else class="department-data">
      <!-- Department Comparison Table -->
      <div class="departments-table-container">
        <table class="departments-table">
          <thead>
            <tr>
              <th>Department</th>
              <th>Teams</th>
              <th>Developers</th>
              <th>Suggestions</th>
              <th>Acceptance Rate</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="dept in departmentData.departments" :key="dept.id">
              <td class="department-name">{{ dept.name }}</td>
              <td>{{ dept.teams }}</td>
              <td>{{ dept.developers }}</td>
              <td>{{ dept.totalSuggestions.toLocaleString() }}</td>
              <td>
                <div class="progress-wrapper">
                  <div class="progress-bar" :style="{ width: `${dept.acceptanceRate * 100}%` }" />
                  <span class="progress-value">{{ (dept.acceptanceRate * 100).toFixed(1) }}%</span>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Department Distribution Chart (placeholder) -->
      <div class="chart-section">
        <h3 class="section-title">Department Distribution</h3>
        <div class="charts-row">
          <div class="chart-card">
            <h4 class="chart-title">Developers by Department</h4>
            <div class="chart-placeholder">
              <p>Chart would be displayed here in the final implementation.</p>
              <p>Showing developer distribution across departments.</p>
            </div>
          </div>

          <div class="chart-card">
            <h4 class="chart-title">Suggestions by Department</h4>
            <div class="chart-placeholder">
              <p>Chart would be displayed here in the final implementation.</p>
              <p>Showing suggestion distribution across departments.</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Summary Cards -->
      <div class="summary-cards">
        <div class="summary-card">
          <div class="card-title">Total Departments</div>
          <div class="card-content">
            <div class="summary-value">{{ departmentData.departments.length }}</div>
          </div>
        </div>

        <div class="summary-card">
          <div class="card-title">Total Teams</div>
          <div class="card-content">
            <div class="summary-value">{{ getTotalTeams() }}</div>
          </div>
        </div>

        <div class="summary-card">
          <div class="card-title">Total Developers</div>
          <div class="card-content">
            <div class="summary-value">{{ getTotalDevelopers() }}</div>
          </div>
        </div>

        <div class="summary-card">
          <div class="card-title">Average Acceptance Rate</div>
          <div class="card-content">
            <div class="summary-value">{{ getAverageAcceptanceRate() }}%</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Department {
  id: string
  name: string
  teams: number
  developers: number
  acceptanceRate: number
  totalSuggestions: number
}

interface DepartmentData {
  departments: Department[]
}

// Props
const props = defineProps<{
  departmentData: DepartmentData
}>()

// Methods
const getTotalTeams = (): number => {
  if (!props.departmentData || !props.departmentData.departments) return 0

  return props.departmentData.departments.reduce((total, dept) => total + dept.teams, 0)
}

const getTotalDevelopers = (): number => {
  if (!props.departmentData || !props.departmentData.departments) return 0

  return props.departmentData.departments.reduce((total, dept) => total + dept.developers, 0)
}

const getAverageAcceptanceRate = (): string => {
  if (
    !props.departmentData ||
    !props.departmentData.departments ||
    props.departmentData.departments.length === 0
  ) {
    return '0.0'
  }

  const sum = props.departmentData.departments.reduce(
    (total, dept) => total + dept.acceptanceRate,
    0
  )
  return ((sum / props.departmentData.departments.length) * 100).toFixed(1)
}
</script>

<style scoped>
.departments-metrics-viewer {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.title {
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
  margin: 0 0 1rem;
}

.department-data {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.departments-table-container {
  background-color: white;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.departments-table {
  width: 100%;
  border-collapse: collapse;
}

.departments-table th,
.departments-table td {
  padding: 1rem;
  text-align: left;
  border-bottom: 1px solid #e5e7eb;
}

.departments-table th {
  background-color: #f9fafb;
  font-weight: 600;
  color: #374151;
}

.departments-table tr:last-child td {
  border-bottom: none;
}

.departments-table tbody tr:hover {
  background-color: #f9fafb;
}

.department-name {
  font-weight: 500;
  color: #111827;
}

.progress-wrapper {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.progress-bar {
  flex-grow: 1;
  height: 8px;
  background-color: #4f46e5;
  border-radius: 4px;
}

.progress-value {
  min-width: 48px;
  font-weight: 500;
  color: #4f46e5;
}

.chart-section {
  background-color: white;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  padding: 1.5rem;
}

.section-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: #111827;
  margin: 0 0 1.5rem;
}

.charts-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
}

.chart-card {
  background-color: #f9fafb;
  border-radius: 0.375rem;
  padding: 1rem;
}

.chart-title {
  font-size: 1rem;
  font-weight: 500;
  color: #374151;
  margin: 0 0 1rem;
}

.chart-placeholder {
  padding: 2rem;
  text-align: center;
  color: #6b7280;
  background-color: #f3f4f6;
  border-radius: 0.25rem;
  min-height: 200px;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.summary-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.summary-card {
  background-color: white;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  padding: 1.5rem;
  transition:
    transform 0.2s,
    box-shadow 0.2s;
}

.summary-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
}

.card-title {
  font-size: 0.875rem;
  font-weight: 500;
  color: #6b7280;
  margin-bottom: 0.75rem;
}

.summary-value {
  font-size: 2rem;
  font-weight: 700;
  color: #4f46e5;
}

.empty-state {
  padding: 2rem;
  text-align: center;
  color: #6b7280;
  background-color: #f9fafb;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

@media (max-width: 768px) {
  .charts-row {
    grid-template-columns: 1fr;
  }
}
</style>
