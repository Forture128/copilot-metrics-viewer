<template>
  <div class="api-response">
    <h2 class="title">API Response Metrics</h2>

    <div v-if="!apiData" class="warning-state">
      <div class="warning-icon">⚠️</div>
      <div class="warning-content">
        <h3 class="warning-title">No Data Available</h3>
        <p class="warning-message">
          API response metrics are currently unavailable. This feature is under development.
        </p>
      </div>
    </div>

    <div v-else class="api-data">
      <!-- Overview Stats -->
      <div class="stats-cards">
        <div class="stat-card">
          <div class="stat-value">{{ (apiData.averageLatency * 1000).toFixed(0) }}ms</div>
          <div class="stat-label">Average Latency</div>
        </div>

        <div class="stat-card">
          <div class="stat-value">{{ (apiData.successRate * 100).toFixed(2) }}%</div>
          <div class="stat-label">Success Rate</div>
        </div>

        <div class="stat-card">
          <div class="stat-value">{{ getTotalErrors().toLocaleString() }}</div>
          <div class="stat-label">Total Errors</div>
        </div>
      </div>

      <!-- Response Time Distribution -->
      <div class="distribution">
        <h3>Response Time Distribution</h3>
        <div class="distribution-chart">
          <div
            v-for="(count, range) in apiData.responseTimeDistribution"
            :key="range"
            class="bar-item"
          >
            <div class="bar-label">{{ formatTimeRange(range) }}</div>
            <div class="bar-container">
              <div
                class="bar"
                :style="{ width: `${getPercentage(count, getTotalResponses())}%` }"
              />
            </div>
            <div class="bar-value">
              {{ count.toLocaleString() }} ({{ getPercentage(count, getTotalResponses()) }}%)
            </div>
          </div>
        </div>
      </div>

      <!-- Error Breakdown -->
      <div class="error-breakdown">
        <h3>Error Breakdown</h3>
        <div v-if="getTotalErrors() === 0" class="empty-chart">No errors recorded.</div>
        <div v-else class="error-chart">
          <div v-for="(count, type) in apiData.errorsByType" :key="type" class="error-item">
            <div class="error-type">{{ formatErrorType(type) }}</div>
            <div class="error-count">{{ count }} errors</div>
            <div class="error-percentage">{{ getPercentage(count, getTotalErrors()) }}%</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ApiResponseData } from '@/types/horus-api.types'

// Props
const props = defineProps<{
  apiData: ApiResponseData | null
}>()

// Methods
const getTotalErrors = () => {
  if (!props.apiData) return 0
  return Object.values(props.apiData.errorsByType).reduce((sum, count) => sum + count, 0)
}

const getTotalResponses = () => {
  if (!props.apiData) return 0
  return Object.values(props.apiData.responseTimeDistribution).reduce(
    (sum, count) => sum + count,
    0
  )
}

const getPercentage = (value: number, total: number) => {
  if (!total) return 0
  return ((value / total) * 100).toFixed(1)
}

const formatTimeRange = (range: string) => {
  switch (range) {
    case 'under100ms':
      return '< 100ms'
    case 'under500ms':
      return '100-500ms'
    case 'under1s':
      return '500ms-1s'
    case 'over1s':
      return '> 1s'
    default:
      return range
  }
}

const formatErrorType = (type: string) => {
  // Capitalize first letter and convert camelCase to Title Case
  return type
    .replace(/([A-Z])/g, ' $1') // Insert a space before capital letters
    .replace(/^./, str => str.toUpperCase()) // Uppercase the first character
}
</script>

<style scoped>
.api-response {
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

.distribution,
.error-breakdown {
  background-color: white;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  padding: 1.5rem;
  margin-bottom: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.distribution h3,
.error-breakdown h3 {
  font-size: 1.25rem;
  margin: 0 0 1.5rem;
  color: #334155;
}

.distribution-chart,
.error-chart {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.bar-item {
  display: grid;
  grid-template-columns: 100px 1fr 100px;
  align-items: center;
  gap: 1rem;
}

.bar-label {
  font-size: 0.875rem;
  color: #334155;
  font-weight: 500;
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

.bar-value {
  font-size: 0.875rem;
  color: #64748b;
  text-align: right;
}

.error-item {
  display: grid;
  grid-template-columns: 1fr 100px 80px;
  align-items: center;
  padding: 0.75rem 1rem;
  border-radius: 0.375rem;
  background-color: #f8fafc;
}

.error-type {
  font-weight: 500;
  color: #334155;
}

.error-count {
  color: #64748b;
  text-align: right;
}

.error-percentage {
  color: #ef4444;
  font-weight: 500;
  text-align: right;
}

.empty-state,
.empty-chart {
  padding: 2rem;
  text-align: center;
  color: #64748b;
  background-color: #f8fafc;
  border-radius: 0.375rem;
}

.warning-state {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  padding: 2rem;
  background-color: #fef3c7;
  border: 1px solid #f59e0b;
  border-radius: 0.5rem;
  margin: 1rem 0;
}

.warning-icon {
  font-size: 1.5rem;
  color: #d97706;
}

.warning-content {
  flex: 1;
}

.warning-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: #92400e;
  margin: 0 0 0.5rem;
}

.warning-message {
  color: #92400e;
  margin: 0;
  line-height: 1.5;
}
</style>
