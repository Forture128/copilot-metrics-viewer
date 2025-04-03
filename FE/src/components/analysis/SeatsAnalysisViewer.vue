<template>
  <div class="seats-analysis-viewer">
    <h2 class="title">Copilot Seats Analysis</h2>

    <div v-if="!seatsData" class="empty-state">No seats data available.</div>

    <div v-else class="seats-data">
      <!-- Overview Stats -->
      <div class="stats-cards">
        <div class="stat-card">
          <div class="stat-value">{{ seatsData.totalSeats }}</div>
          <div class="stat-label">Total Seats</div>
        </div>

        <div class="stat-card">
          <div class="stat-value">{{ seatsData.activeSeats }}</div>
          <div class="stat-label">Active Seats</div>
        </div>

        <div class="stat-card">
          <div class="stat-value">{{ (seatsData.utilizationRate * 100).toFixed(1) }}%</div>
          <div class="stat-label">Utilization Rate</div>
        </div>
      </div>

      <!-- Department Allocation -->
      <div class="department-allocation">
        <h3>Seats by Department</h3>
        <div class="department-chart">
          <div v-for="(count, dept) in seatsData.byDepartment" :key="dept" class="department-bar">
            <div class="department-info">
              <span class="department-name">{{ formatDepartmentName(dept) }}</span>
              <span class="department-count">{{ count }} seats</span>
            </div>
            <div class="bar-container">
              <div class="bar" :style="{ width: `${(count / seatsData.totalSeats) * 100}%` }" />
            </div>
          </div>
        </div>
      </div>

      <!-- Usage Trends Placeholder -->
      <div class="usage-trends">
        <h3>Usage Trends</h3>
        <div class="chart-placeholder">
          <p>Chart would be displayed here in the final implementation.</p>
          <p>Showing monthly seat allocation and usage over time.</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Props
defineProps({
  seatsData: {
    type: Object,
    required: true
  }
})

// Methods
const formatDepartmentName = (name: string) => {
  // Convert camelCase to Title Case
  return name
    .replace(/([A-Z])/g, ' $1') // Insert a space before capital letters
    .replace(/^./, str => str.toUpperCase()) // Uppercase the first character
}
</script>

<style scoped>
.seats-analysis-viewer {
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

.stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.stat-card {
  background-color: #f8fafc;
  border-radius: 0.5rem;
  padding: 1.5rem;
  text-align: center;
  transition:
    transform 0.2s,
    box-shadow 0.2s;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.stat-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
}

.stat-value {
  font-size: 2rem;
  font-weight: 700;
  color: #4f46e5;
  margin-bottom: 0.5rem;
}

.stat-label {
  color: #64748b;
  font-size: 0.875rem;
}

.department-allocation,
.usage-trends {
  background-color: white;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  padding: 1.5rem;
  margin-bottom: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.department-allocation h3,
.usage-trends h3 {
  font-size: 1.25rem;
  margin: 0 0 1rem;
  color: #334155;
}

.department-chart {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.department-bar {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.department-info {
  display: flex;
  justify-content: space-between;
  font-size: 0.875rem;
}

.department-name {
  font-weight: 500;
  color: #334155;
}

.department-count {
  color: #64748b;
}

.bar-container {
  height: 12px;
  background-color: #e2e8f0;
  border-radius: 6px;
  overflow: hidden;
}

.bar {
  height: 100%;
  background-color: #4f46e5;
  border-radius: 6px;
  transition: width 0.5s ease-out;
}

.chart-placeholder {
  background-color: #f8fafc;
  border-radius: 0.375rem;
  padding: 2rem;
  text-align: center;
  color: #64748b;
}

.empty-state {
  padding: 2rem;
  text-align: center;
  color: #64748b;
  background-color: #f8fafc;
  border-radius: 0.375rem;
}
</style>
