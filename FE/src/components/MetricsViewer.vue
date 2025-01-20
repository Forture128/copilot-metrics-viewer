<template>
  <div class="space-y-6">
    <!-- Date Range Selection -->
    <div class="flex justify-end items-center mb-6">
      <DateRangePeriodSelector
        v-model="dateRange"
        v-model:selected-period="selectedPeriod"
        @date-range-changed="updateMetricsData"
      />
    </div>

    <!-- Dashboard Container for Metrics -->
    <div class="grid gap-4 grid-cols-12">
      <MetricCard
        title="Acceptance Rate"
        subtitle="Over the selected period"
        :value="acceptanceRateAverage.toFixed(2) + '%'"
        icon="mdi-chart-areaspline"
        :trend="acceptanceRateTrend"
        :description="`${acceptanceRateTrend.toFixed(1)}% from previous period`"
      />
      <MetricCard
        title="Total Suggestions"
        subtitle="Over the selected period"
        :value="cumulativeNumberSuggestions"
        icon="mdi-lightbulb-outline"
        :trend="suggestionsTrend"
        :description="`${suggestionsTrend.toFixed(1)}% from previous period`"
      />
      <MetricCard
        title="Accepted Prompts"
        subtitle="Over the selected period"
        :value="cumulativeNumberAcceptances"
        icon="mdi-checkbox-marked-circle-outline"
        :trend="acceptancesTrend"
        :description="`${acceptancesTrend.toFixed(1)}% from previous period`"
      />
      <MetricCard
        title="Lines of Code Accepted"
        subtitle="Over the selected period"
        :value="cumulativeNumberLOCAccepted"
        icon="mdi-code-tags"
        :trend="locAcceptedTrend"
        :description="`${locAcceptedTrend.toFixed(1)}% from previous period`"
      />
    </div>

    <!-- Full-Width Chart Section for Larger, Detailed Charts -->
    <div class="space-y-6">
      <!-- <div class="grid grid-cols-1 md:grid-cols-2 gap-6"> -->
      <FullWidthChart
        title="Total Suggestions Count | Total Acceptances Count | Acceptance Rate (%)"
        description="Tracking suggestions and acceptance rates over time"
        :data="totalSuggestionsAndAcceptanceChartData"
        :options="chartOptions"
      />
      <FullWidthChart
        title="Total Lines Suggested | Total Lines Accepted | Acceptance Lines Rate (%)"
        description="Comparing suggested and accepted lines of code"
        :data="chartData"
        :options="chartOptions"
      />
      <!-- </div> -->
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, watch } from 'vue'
import type { Metrics } from '../model/Metrics'
import { calculateCumulativeMetrics } from '@/utils/MetricUtils'
import FullWidthChart from './Commons/FullWidthChart.vue' // FullWidthChart for larger charts
import MetricCard from './Commons/MetricCard.vue' // MetricCard for mini metrics
import type { ChartOptions, LineControllerChartOptions } from 'chart.js'
import { subMonths, addDays } from 'date-fns'
import DateRangePeriodSelector from './Commons/DateRangePeriodSelector.vue'

export default defineComponent({
  name: 'MetricsViewer',
  components: {
    FullWidthChart,
    MetricCard,
    DateRangePeriodSelector
  },
  props: {
    metrics: {
      type: Object,
      required: true
    }
  },
  setup(props) {
    // Chart Data
    const totalSuggestionsAndAcceptanceChartData = ref<{
      labels: string[]
      datasets: any[]
    }>({
      labels: [],
      datasets: []
    })

    const chartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: []
    })

    const chartOptions: ChartOptions<'line'> & LineControllerChartOptions = {
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
            display: false
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
            display: false
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
          top: 50,
          bottom: 50,
          left: 50,
          right: 50
        }
      },
      spanGaps: false,
      showLine: true
    }

    const data = Array.isArray(props.metrics) ? props.metrics : []
    const {
      cumulativeNumberSuggestions,
      cumulativeNumberAcceptances,
      cumulativeNumberLOCAccepted,
      acceptanceRateAverage
    } = calculateCumulativeMetrics(data)

    const cumulativeSuggestionsData = data.map((m: Metrics) => m.total_suggestions_count)
    const cumulativeAcceptancesData = data.map((m: Metrics) => m.total_acceptances_count)
    const acceptanceRates = data.map((m: Metrics) =>
      m.total_suggestions_count !== 0
        ? (m.total_acceptances_count / m.total_suggestions_count) * 100
        : 0
    )

    totalSuggestionsAndAcceptanceChartData.value = {
      labels: data.map((m: Metrics) => m.day),
      datasets: [
        {
          label: 'Total Suggestions',
          data: cumulativeSuggestionsData,
          backgroundColor: 'rgba(75, 192, 192, 0.6)',
          borderColor: 'rgb(75, 192, 192)',
          type: 'bar'
        },
        {
          label: 'Total Acceptance',
          data: cumulativeAcceptancesData,
          backgroundColor: 'rgba(255, 99, 132, 0.6)',
          borderColor: 'rgb(255, 99, 132)',
          type: 'bar'
        },
        {
          label: 'Acceptance Rate',
          data: acceptanceRates,
          type: 'line',
          yAxisID: 'y1',
          backgroundColor: 'rgba(54, 162, 235, 0.6)',
          borderColor: 'rgb(54, 162, 235)'
        }
      ]
    }

    const cumulativeLOCAcceptedData = data.map((m: Metrics) => m.total_lines_accepted)
    const acceptanceLinesRates = data.map((m: Metrics) =>
      m.total_lines_suggested !== 0 ? (m.total_lines_accepted / m.total_lines_suggested) * 100 : 0
    )

    chartData.value = {
      labels: data.map((m: Metrics) => m.day),
      datasets: [
        {
          label: 'Total Lines Suggested',
          data: data.map((m: Metrics) => m.total_lines_suggested),
          backgroundColor: 'rgba(75, 192, 192, 0.6)',
          borderColor: 'rgb(75, 192, 192)',
          type: 'bar'
        },
        {
          label: 'Total Lines Accepted',
          data: cumulativeLOCAcceptedData,
          backgroundColor: 'rgba(255, 99, 132, 0.6)',
          borderColor: 'rgb(255, 99, 132)',
          type: 'bar'
        },
        {
          label: 'Acceptance Lines Rate',
          data: acceptanceLinesRates,
          backgroundColor: 'rgba(54, 162, 235, 0.6)',
          borderColor: 'rgb(54, 162, 235)',
          type: 'line',
          fill: false,
          yAxisID: 'y1'
        }
      ]
    }

    const dateRange = ref({
      start: subMonths(new Date(), 1),
      end: new Date()
    })
    const selectedPeriod = ref('monthly')

    // Add trend calculations
    const acceptanceRateTrend = ref(0)
    const suggestionsTrend = ref(0)
    const acceptancesTrend = ref(0)
    const locAcceptedTrend = ref(0)

    const calculateTrend = (currentValue: number, previousValue: number): number => {
      if (previousValue === 0) return 0
      return Number((((currentValue - previousValue) / previousValue) * 100).toFixed(2))
    }

    const updateMetricsData = () => {
      // Filter metrics based on date range
      const filteredMetrics = data.filter((m: Metrics) => {
        const date = new Date(m.day)
        return date >= dateRange.value.start && date <= dateRange.value.end
      })

      // Calculate previous period metrics
      const periodDays = Math.floor(
        (dateRange.value.end.getTime() - dateRange.value.start.getTime()) / (1000 * 60 * 60 * 24)
      )
      const previousStart = addDays(dateRange.value.start, -periodDays)
      const previousEnd = addDays(dateRange.value.end, -periodDays)

      const previousPeriodMetrics = data.filter((m: Metrics) => {
        const date = new Date(m.day)
        return date >= previousStart && date <= previousEnd
      })

      // Calculate current period metrics
      const currentMetrics = calculateCumulativeMetrics(filteredMetrics)
      const previousMetrics = calculateCumulativeMetrics(previousPeriodMetrics)

      // Update trends
      acceptanceRateTrend.value = calculateTrend(
        currentMetrics.acceptanceRateAverage,
        previousMetrics.acceptanceRateAverage
      )
      suggestionsTrend.value = calculateTrend(
        currentMetrics.cumulativeNumberSuggestions,
        previousMetrics.cumulativeNumberSuggestions
      )
      acceptancesTrend.value = calculateTrend(
        currentMetrics.cumulativeNumberAcceptances,
        previousMetrics.cumulativeNumberAcceptances
      )
      locAcceptedTrend.value = calculateTrend(
        currentMetrics.cumulativeNumberLOCAccepted,
        previousMetrics.cumulativeNumberLOCAccepted
      )

      // Update chart data
      updateChartData(filteredMetrics)
    }

    const updateChartData = (filteredMetrics: Metrics[]) => {
      const cumulativeSuggestionsData = filteredMetrics.map(m => m.total_suggestions_count)
      const cumulativeAcceptancesData = filteredMetrics.map(m => m.total_acceptances_count)
      const acceptanceRates = filteredMetrics.map(m =>
        m.total_suggestions_count !== 0
          ? (m.total_acceptances_count / m.total_suggestions_count) * 100
          : 0
      )

      totalSuggestionsAndAcceptanceChartData.value = {
        labels: filteredMetrics.map(m => m.day),
        datasets: [
          {
            ...totalSuggestionsAndAcceptanceChartData.value.datasets[0],
            data: cumulativeSuggestionsData
          },
          {
            ...totalSuggestionsAndAcceptanceChartData.value.datasets[1],
            data: cumulativeAcceptancesData
          },
          {
            ...totalSuggestionsAndAcceptanceChartData.value.datasets[2],
            data: acceptanceRates
          }
        ]
      }

      chartData.value = {
        labels: filteredMetrics.map(m => m.day),
        datasets: [
          {
            ...chartData.value.datasets[0],
            data: filteredMetrics.map(m => m.total_lines_suggested)
          },
          {
            ...chartData.value.datasets[1],
            data: filteredMetrics.map(m => m.total_lines_accepted)
          },
          {
            ...chartData.value.datasets[2],
            data: filteredMetrics.map(m =>
              m.total_lines_suggested !== 0
                ? (m.total_lines_accepted / m.total_lines_suggested) * 100
                : 0
            )
          }
        ]
      }
    }

    // Watch for date range changes
    watch([dateRange, selectedPeriod], updateMetricsData, { immediate: true })

    return {
      totalSuggestionsAndAcceptanceChartData,
      chartData,
      chartOptions,
      acceptanceRateAverage,
      cumulativeNumberSuggestions,
      cumulativeNumberAcceptances,
      cumulativeNumberLOCAccepted,
      dateRange,
      selectedPeriod,
      updateMetricsData,
      acceptanceRateTrend,
      suggestionsTrend,
      acceptancesTrend,
      locAcceptedTrend
    }
  }
})
</script>

<style scoped>
.dashboard-container {
  margin-bottom: 20px;
}

.charts-container {
  margin-top: 30px;
}

.full-chart-card {
  height: 100%;
}
</style>
