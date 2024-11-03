<template>
  <div class="space-y-6">
    <!-- Filters Section -->
    <div class="flex justify-between items-center mb-6">
      <div class="flex-1 max-w-xs">
        <CustomizeSelect
          v-model="selectedDepartment"
          class="w-full"
          @update:model-value="updateDepartmentData"
        >
          <SelectTrigger>
            <SelectValue
              :placeholder="selectedDepartment ? selectedDepartment : 'Select Department'"
            />
          </SelectTrigger>
          <SelectContent>
            <SelectItem v-for="dept in Object.keys(departments)" :key="dept" :value="dept">
              {{ dept }}
            </SelectItem>
          </SelectContent>
        </CustomizeSelect>
      </div>

      <DateRangePeriodSelector
        v-model="dateRange"
        v-model:selected-period="selectedPeriod"
        @date-range-changed="updateDepartmentData(selectedDepartment)"
      />
    </div>

    <!-- Dashboard Container for Metrics -->
    <div class="grid gap-4 grid-cols-12">
      <MetricCard
        title="Acceptance Rate Average"
        subtitle="Over the selected period"
        :value="currentMetrics.acceptanceRateAverage.toFixed(2) + '%'"
        icon="mdi-chart-areaspline"
        :trend="acceptanceRateTrend"
        :description="`${acceptanceRateTrend.toFixed(1)}% from previous period`"
      />
      <MetricCard
        title="Total Suggestions"
        subtitle="Over the selected period"
        :value="currentMetrics.cumulativeNumberSuggestions"
        icon="mdi-lightbulb-outline"
        :trend="suggestionsTrend"
        :description="`${suggestionsTrend.toFixed(1)}% from previous period`"
      />
      <MetricCard
        title="Accepted Prompts"
        subtitle="Over the selected period"
        :value="currentMetrics.cumulativeNumberAcceptances"
        icon="mdi-checkbox-marked-circle-outline"
        :trend="acceptancesTrend"
        :description="`${acceptancesTrend.toFixed(1)}% from previous period`"
      />
      <MetricCard
        title="Lines of Code Accepted"
        subtitle="Over the selected period"
        :value="currentMetrics.cumulativeNumberLOCAccepted"
        icon="mdi-code-tags"
        :trend="locAcceptedTrend"
        :description="`${locAcceptedTrend.toFixed(1)}% from previous period`"
      />
    </div>

    <!-- Charts Section -->
    <div class="space-y-6">
      <FullWidthChart
        title="Team Acceptance Rates"
        description="Tracking the acceptance rates of teams"
        :data="teamAcceptanceRateChartData"
        :options="teamAcceptanceRateChartOptions"
      />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, watch } from 'vue'
import type { TeamMetrics } from '../model/Metrics'
import type { ChartOptions } from 'chart.js'
import { calculateTeamAcceptanceRate, calculateTeamCumulativeMetrics } from '@/utils/MetricUtils'
import MetricCard from './Commons/MetricCard.vue'
import FullWidthChart from './Commons/FullWidthChart.vue'
import DateRangePeriodSelector from './Commons/DateRangePeriodSelector.vue'
import { subMonths, addDays } from 'date-fns'
import {
  Select as CustomizeSelect,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select'

export default defineComponent({
  name: 'DepartmentsMetricsViewer',
  components: {
    MetricCard,
    FullWidthChart,
    DateRangePeriodSelector,
    CustomizeSelect,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue
  },
  props: {
    teamMetrics: {
      type: Array as () => TeamMetrics[],
      required: true
    },
    departments: {
      type: Object as () => { [key: string]: string[] },
      required: true
    }
  },
  setup(props) {
    const selectedDepartment = ref(Object.keys(props.departments)[0])
    const dateRange = ref({
      start: subMonths(new Date(), 1),
      end: new Date()
    })
    const selectedPeriod = ref('monthly')

    // Metrics refs
    const currentMetrics = ref({
      acceptanceRateAverage: 0,
      cumulativeNumberSuggestions: 0,
      cumulativeNumberAcceptances: 0,
      cumulativeNumberLOCAccepted: 0
    })

    // Trend refs
    const acceptanceRateTrend = ref(0)
    const suggestionsTrend = ref(0)
    const acceptancesTrend = ref(0)
    const locAcceptedTrend = ref(0)

    // Chart Data
    const teamAcceptanceRateChartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: []
    })

    const calculateTrend = (currentValue: number, previousValue: number): number => {
      if (isNaN(previousValue)) return 0
      if (previousValue === 0) return 0
      return Number((((currentValue - previousValue) / previousValue) * 100).toFixed(2))
    }

    const updateDepartmentData = (department: string) => {
      // Get teams for selected department
      const departmentTeams = props.departments[department] || []

      // Filter teams by department
      const departmentTeamMetrics = props.teamMetrics.filter(team =>
        departmentTeams.includes(team.team_tag)
      )

      // Filter metrics based on date range
      const filteredTeamMetrics = departmentTeamMetrics.map(team => ({
        ...team,
        metrics: team.metrics.filter(m => {
          const date = new Date(m.day)
          return date >= dateRange.value.start && date <= dateRange.value.end
        })
      }))

      // Calculate previous period metrics
      const periodDays = Math.floor(
        (dateRange.value.end.getTime() - dateRange.value.start.getTime()) / (1000 * 60 * 60 * 24)
      )
      const previousStart = addDays(dateRange.value.start, -periodDays)
      const previousEnd = addDays(dateRange.value.end, -periodDays)

      const previousPeriodMetrics = departmentTeamMetrics.map(team => ({
        ...team,
        metrics: team.metrics.filter(m => {
          const date = new Date(m.day)
          return date >= previousStart && date <= previousEnd
        })
      }))

      // Calculate metrics
      const current = calculateTeamCumulativeMetrics(filteredTeamMetrics)
      const previous = calculateTeamCumulativeMetrics(previousPeriodMetrics)

      // Update current metrics
      currentMetrics.value = current

      // Update trends
      acceptanceRateTrend.value = calculateTrend(
        current.acceptanceRateAverage,
        previous.acceptanceRateAverage
      )
      suggestionsTrend.value = calculateTrend(
        current.cumulativeNumberSuggestions,
        previous.cumulativeNumberSuggestions
      )
      acceptancesTrend.value = calculateTrend(
        current.cumulativeNumberAcceptances,
        previous.cumulativeNumberAcceptances
      )
      locAcceptedTrend.value = calculateTrend(
        current.cumulativeNumberLOCAccepted,
        previous.cumulativeNumberLOCAccepted
      )

      // Update chart data
      updateChartData(filteredTeamMetrics)
    }

    const updateChartData = (teamMetricsData: TeamMetrics[]) => {
      const sortedTeams = teamMetricsData
        .map(team => ({
          team_tag: team.team_tag,
          acceptanceRate: calculateTeamAcceptanceRate(team.metrics),
          totalSuggestion: team.metrics.reduce((sum, m) => sum + m.total_suggestions_count, 0),
          totalAcceptance: team.metrics.reduce((sum, m) => sum + m.total_acceptances_count, 0)
        }))
        .sort((a, b) => b.acceptanceRate - a.acceptanceRate)

      teamAcceptanceRateChartData.value = {
        labels: sortedTeams.map(team => team.team_tag),
        datasets: [
          {
            label: 'Acceptance Rate',
            data: sortedTeams.map(team => team.acceptanceRate),
            backgroundColor: 'rgba(54, 162, 235, 0.6)',
            borderColor: 'rgb(54, 162, 235)',
            fill: false,
            type: 'line',
            yAxisID: 'y1'
          },
          {
            label: 'Total Suggestions',
            data: sortedTeams.map(team => team.totalSuggestion),
            backgroundColor: 'rgba(75, 192, 192, 0.6)',
            borderColor: 'rgb(75, 192, 192)',
            type: 'bar',
            yAxisID: 'y'
          },
          {
            label: 'Total Acceptances',
            data: sortedTeams.map(team => team.totalAcceptance),
            backgroundColor: 'rgba(255, 99, 132, 0.6)',
            borderColor: 'rgb(255, 99, 132)',
            type: 'bar',
            yAxisID: 'y'
          }
        ]
      }
    }

    // Watch for changes in department selection and date range
    watch(
      [selectedDepartment, dateRange, selectedPeriod],
      () => updateDepartmentData(selectedDepartment.value),
      { immediate: true }
    )
    const teamAcceptanceRateChartOptions: ChartOptions<'bar' | 'line'> = {
      responsive: true,
      maintainAspectRatio: true,
      scales: {
        y: {
          beginAtZero: true,
          ticks: {
            stepSize: 5000
          },
          title: {
            display: true,
            text: 'Suggested / Accepted'
          },
          grid: {
            display: false // Remove y-axis grid lines
          }
        },
        y1: {
          beginAtZero: true,
          position: 'right',
          ticks: {
            callback: function (value: string | number) {
              return value + '%'
            }
          },
          title: {
            display: true,
            text: 'Acceptance Rate (%)'
          },
          grid: {
            display: false // Remove y1-axis grid lines
          }
        },
        x: {
          type: 'category',
          ticks: {
            autoSkip: true,
            maxTicksLimit: 10
          }
        }
      },
      layout: {
        padding: {
          left: 50,
          right: 50,
          top: 50,
          bottom: 50
        }
      },
      plugins: {
        tooltip: {
          callbacks: {
            label: function (context) {
              let label = context.dataset.label || ''
              if (label) {
                label += ': '
              }
              if (context.parsed.y !== null) {
                if (context.dataset.yAxisID === 'y1') {
                  label += context.parsed.y.toFixed(2) + '%'
                } else {
                  label += context.parsed.y
                }
              }
              return label
            }
          }
        }
      }
    }

    return {
      selectedDepartment,
      dateRange,
      selectedPeriod,
      currentMetrics,
      acceptanceRateTrend,
      suggestionsTrend,
      acceptancesTrend,
      locAcceptedTrend,
      teamAcceptanceRateChartData,
      teamAcceptanceRateChartOptions,
      updateDepartmentData
    }
  }
})
</script>

<style scoped>
.dashboard-container {
  margin-bottom: 20px;
}

.metric-card {
  height: 100%;
}

.metric-title {
  font-size: 1.2em;
  font-weight: bold;
}

.metric-subtitle {
  font-size: 0.9em;
  color: #666;
}

.metric-value {
  font-size: 2em;
  font-weight: bold;
  margin-top: 10px;
}

.line-chart-container,
.seats-table-container {
  margin-top: 30px;
}

.table-card {
  padding: 10px;
}

.table-container {
  max-height: 500px;
  overflow-y: auto;
}
</style>
