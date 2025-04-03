<template>
  <div class="breakdown-component">
    <h2 class="breakdown-title">{{ breakdownTitle }}</h2>

    <div v-if="!hasMetricsData" class="empty-state">No data available.</div>

    <div v-else class="breakdown-content">
      <!-- Stats Cards -->
      <MetricCard
        :title="`Number of ${breakdownDisplayNamePlural}`"
        subtitle="Over the last 28 days"
        :value="numberOfBreakdowns"
      />

      <!-- Pie Chart -->
      <div class="chart-container">
        <canvas ref="pieChart" />
      </div>

      <!-- Data Table -->
      <div class="breakdown-table">
        <table>
          <thead>
            <tr>
              <th>{{ breakdownName }}</th>
              <th>Suggestions</th>
              <th v-if="hasLegacyData">Accepted Lines</th>
              <th>{{ hasLegacyData ? 'Acceptance Rate' : 'Percentage' }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(item, index) in tableData" :key="index">
              <td>{{ item.name }}</td>
              <td>{{ item.count.toLocaleString() }}</td>
              <td v-if="hasLegacyData && 'acceptedLines' in item">
                {{ item.acceptedLines?.toLocaleString() || '0' }}
              </td>
              <td>{{ item.percentage }}%</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, watch } from 'vue'
import { Breakdown } from '../../model/Breakdown'
import MetricCard from '../Commons/MetricCard.vue'
import Chart from 'chart.js/auto'

// Define types
interface LegacyBreakdownItem {
  name: string
  count: number
  acceptedLines: number
  percentage: string
}

interface SimpleBreakdownItem {
  name: string
  count: number
  percentage: string
}

type BreakdownItem = LegacyBreakdownItem | SimpleBreakdownItem

// Props
const props = defineProps({
  metrics: {
    type: Object,
    required: true
  },
  breakdownKey: {
    type: String,
    default: 'language',
    validator: (value: string) => ['language', 'editor'].includes(value)
  }
})

// Refs
const pieChart = ref<HTMLCanvasElement | null>(null)
let chartInstance: any = null

// Computed
const breakdownTitle = computed(() => {
  return props.breakdownKey === 'language' ? 'Language Breakdown' : 'Editor Breakdown'
})

const breakdownName = computed(() => {
  return props.breakdownKey === 'language' ? 'Language' : 'Editor'
})

const breakdownDisplayNamePlural = computed(() => {
  return props.breakdownKey === 'language' ? 'Languages' : 'Editors'
})

const hasLegacyData = computed(() => {
  return Array.isArray(props.metrics) || (props.metrics && 'breakdown' in props.metrics)
})

const hasSimpleData = computed(() => {
  return (
    props.metrics &&
    ((props.breakdownKey === 'language' && 'byLanguage' in props.metrics) ||
      (props.breakdownKey === 'editor' && 'byEditor' in props.metrics))
  )
})

const hasMetricsData = computed(() => {
  return hasLegacyData.value || hasSimpleData.value
})

// Count of unique breakdowns
const numberOfBreakdowns = computed(() => {
  return tableData.value.length
})

const processedData = computed((): BreakdownItem[] => {
  // For legacy data format
  if (hasLegacyData.value) {
    const breakdownMap = new Map<string, Breakdown>()
    const metrics = Array.isArray(props.metrics) ? props.metrics : [props.metrics]

    metrics.forEach((m: any) => {
      if (!m.breakdown) return

      m.breakdown.forEach((item: any) => {
        const key = item[props.breakdownKey]
        if (!key) return

        let breakdown = breakdownMap.get(key)
        if (!breakdown) {
          // Create a new breakdown if it doesn't exist
          breakdown = new Breakdown({
            name: key,
            acceptedPrompts: item.acceptances_count || 0,
            suggestedLinesOfCode: item.lines_suggested || 0,
            acceptedLinesOfCode: item.lines_accepted || 0,
            acceptanceRate: 0 // Will be calculated below
          })
          breakdownMap.set(key, breakdown)
        } else {
          // Update the existing breakdown
          breakdown.acceptedPrompts += item.acceptances_count || 0
          breakdown.suggestedLinesOfCode += item.lines_suggested || 0
          breakdown.acceptedLinesOfCode += item.lines_accepted || 0
        }

        // Calculate acceptance rate
        if (breakdown.suggestedLinesOfCode > 0) {
          breakdown.acceptanceRate =
            (breakdown.acceptedLinesOfCode / breakdown.suggestedLinesOfCode) * 100
        }
      })
    })

    // Convert to the format needed for charts and table
    return Array.from(breakdownMap.entries())
      .map(([, breakdown]) => {
        return {
          name: formatKey(breakdown.name),
          count: breakdown.acceptedPrompts,
          acceptedLines: breakdown.acceptedLinesOfCode,
          percentage: breakdown.acceptanceRate.toFixed(1)
        } as LegacyBreakdownItem
      })
      .sort((a, b) => b.count - a.count)
  }

  // For simple data format (new)
  if (hasSimpleData.value) {
    const data =
      props.breakdownKey === 'language'
        ? (props.metrics as any).byLanguage || {}
        : (props.metrics as any).byEditor || {}

    const total = Object.values(data).reduce(
      (sum: number, value: any) => sum + (Number(value) || 0),
      0
    )

    return Object.entries(data)
      .map(([key, value]) => {
        const count = Number(value) || 0
        const percentage = total > 0 ? ((count / total) * 100).toFixed(1) : '0.0'

        return {
          name: formatKey(key),
          count,
          percentage
        } as SimpleBreakdownItem
      })
      .sort((a, b) => b.count - a.count)
  }

  return []
})

const tableData = computed(() => {
  return processedData.value
})

const chartData = computed(() => {
  const data = processedData.value
  return {
    labels: data.map(item => item.name),
    values: data.map(item => item.count)
  }
})

// Methods
const formatKey = (key: string) => {
  if (typeof key !== 'string') return 'Unknown'

  // Capitalize first letter and format camelCase to Title Case
  return key
    .replace(/([A-Z])/g, ' $1') // Insert a space before capital letters
    .replace(/^./, str => str.toUpperCase()) // Uppercase the first character
}

const generateRandomColors = (count: number) => {
  const colors = []
  for (let i = 0; i < count; i++) {
    const hue = (i * 137) % 360 // Use golden ratio to space out the hues
    colors.push(`hsl(${hue}, 65%, 65%)`)
  }
  return colors
}

const renderChart = () => {
  if (!pieChart.value || !hasMetricsData.value) return

  // Destroy existing chart if it exists
  if (chartInstance) {
    chartInstance.destroy()
  }

  const { labels, values } = chartData.value
  const colors = generateRandomColors(labels.length)

  const ctx = pieChart.value.getContext('2d')
  if (!ctx) return

  // Create pie chart
  chartInstance = new Chart(ctx, {
    type: 'pie',
    data: {
      labels,
      datasets: [
        {
          data: values,
          backgroundColor: colors,
          borderColor: 'white',
          borderWidth: 1
        }
      ]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        legend: {
          position: 'right'
        },
        tooltip: {
          callbacks: {
            label: context => {
              const value = context.raw as number
              const total = values.reduce((sum, val) => sum + val, 0)
              const percentage = total > 0 ? ((value / total) * 100).toFixed(1) : '0.0'
              return `${context.label}: ${value.toLocaleString()} (${percentage}%)`
            }
          }
        }
      }
    }
  })
}

// Lifecycle hooks
onMounted(() => {
  if (hasMetricsData.value) {
    renderChart()
  }
})

// Watch for changes in metrics or breakdownKey
watch([() => props.metrics, () => props.breakdownKey], () => {
  renderChart()
})
</script>

<style scoped>
.breakdown-component {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.breakdown-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
  margin: 0 0 1rem;
}

.breakdown-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
}

.chart-container {
  height: 300px;
  position: relative;
}

.breakdown-table {
  max-height: 300px;
  overflow-y: auto;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th,
td {
  padding: 0.75rem 1rem;
  text-align: left;
  border-bottom: 1px solid #e2e8f0;
}

th {
  background-color: #f8fafc;
  font-weight: 600;
}

tr:last-child td {
  border-bottom: none;
}

.empty-state {
  padding: 2rem;
  text-align: center;
  color: #64748b;
  background-color: #f8fafc;
  border-radius: 0.375rem;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .breakdown-content {
    grid-template-columns: 1fr;
  }
}
</style>
