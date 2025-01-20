<template>
  <div class="space-y-6">
    <!-- Team and Date Selection -->
    <div class="flex justify-between items-center mb-6">
      <div class="flex-1 max-w-xs">
        <CustomizeSelect v-model="selectedTeam" class="w-full" @update:model-value="updateTeamData">
          <SelectTrigger>
            <SelectValue :placeholder="selectedTeam ? selectedTeam : 'Select Team'" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem v-for="team in uniqueTeams" :key="team" :value="team">
              {{ team }}
            </SelectItem>
          </SelectContent>
        </CustomizeSelect>
      </div>

      <DateRangePeriodSelector
        v-model="dateRange"
        v-model:selected-period="selectedPeriod"
        @date-range-changed="updateTeamData(selectedTeam)"
      />
    </div>

    <!-- Dashboard Container for Team Metrics -->
    <div class="grid gap-4 grid-cols-12">
      <MetricCard
        title="Total Suggestions"
        :subtitle="`For ${selectedTeam}`"
        :value="teamMetrics.totalSuggestionsCount"
        icon="mdi-lightbulb-outline"
        :trend="teamMetrics.suggestionsTrend"
        :description="`${teamMetrics.suggestionsTrend.toFixed(1)}% from previous period`"
      />
      <MetricCard
        title="Total Acceptances"
        :subtitle="`For ${selectedTeam}`"
        :value="teamMetrics.totalAcceptancesCount"
        icon="mdi-checkbox-marked-circle-outline"
        :trend="teamMetrics.acceptancesTrend"
        :description="`${teamMetrics.acceptancesTrend.toFixed(1)}% from previous period`"
      />
      <MetricCard
        title="Active Users"
        :subtitle="`For ${selectedTeam}`"
        :value="teamMetrics.totalActiveUsers"
        icon="mdi-account-group"
        :trend="teamMetrics.activeUsersTrend"
        :description="`${teamMetrics.activeUsersTrend.toFixed(1)}% from previous period`"
      />
      <!-- <MetricCard
        title="Active Chat Users"
        :subtitle="`For ${selectedTeam}`"
        :value="teamMetrics.totalActiveChatUsers"
        icon="mdi-message-text"
        :trend="6.7"
        description="6.7% increase in chat usage"
      /> -->
      <MetricCard
        title="Chat Users Acceptances"
        :subtitle="`For ${selectedTeam}`"
        :value="teamMetrics.totalChatAcceptances"
        icon="mdi-checkbox-marked-circle-outline"
        :trend="teamMetrics.chatAcceptancesTrend"
        :description="`${teamMetrics.chatAcceptancesTrend.toFixed(1)}% from previous period`"
      />
    </div>

    <!-- Charts and Members Section -->
    <div class="grid gap-6 grid-cols-12">
      <!-- Main charts section -->
      <div class="col-span-8 space-y-6">
        <FullWidthChart
          title="Suggestions & Acceptances"
          :data="suggestionsAcceptancesChartData"
          :options="chartOptions"
        />
        <FullWidthChart
          title="Lines Suggested vs Accepted"
          :data="linesSuggestedAcceptedChartData"
          :options="chartOptions"
        />
      </div>

      <!-- Team members section -->
      <div class="col-span-4 space-y-6">
        <TeamMembersSection :members="teamMembers" />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { useToast } from 'vue-toastification'
import { defineComponent, ref, watch, computed } from 'vue'
import type { ChartOptions, LineControllerChartOptions } from 'chart.js'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
  Filler
} from 'chart.js'
import type { TeamMetrics } from '@/model/Metrics'
// import TeamBreakdownComponent from "./TeamBreakdownComponent.vue";
import MetricCard from './Commons/MetricCard.vue'
import FullWidthChart from './Commons/FullWidthChart.vue'
import {
  Select as CustomizeSelect,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select'
import { cn } from '@/lib/utils'
import { subMonths, addDays, format } from 'date-fns'
import DateRangePeriodSelector from '@/components/Commons/DateRangePeriodSelector.vue'
import TeamMembersSection from './Commons/TeamMembersSection.vue'
import type { Members } from '@/model/Members'

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
  Filler
)

export default defineComponent({
  name: 'TeamMetricsViewer',
  components: {
    // TeamBreakdownComponent,
    MetricCard,
    FullWidthChart,
    CustomizeSelect,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
    DateRangePeriodSelector,
    TeamMembersSection
  },
  props: {
    teams: {
      type: Array as () => string[],
      required: true
    },
    metrics: {
      type: Array as () => TeamMetrics[],
      required: true
    }
  },
  setup(props) {
    const toast = useToast()
    const selectedTeam = ref(props.teams[0])
    const teamMembers = ref<Members[]>([])
    const filteredMetrics = ref<any[]>([])
    const filteredMetricsKey = ref(0)
    const teamMetrics = ref({
      totalSuggestionsCount: 0,
      totalAcceptancesCount: 0,
      totalLinesSuggested: 0,
      totalLinesAccepted: 0,
      totalActiveUsers: 0,
      totalChatAcceptances: 0,
      totalChatTurns: 0,
      totalActiveChatUsers: 0,
      suggestionsTrend: 0,
      acceptancesTrend: 0,
      activeUsersTrend: 0,
      chatAcceptancesTrend: 0
    })
    const suggestionsAcceptancesChartData = ref<{
      labels: string[]
      datasets: any[]
    }>({ labels: [], datasets: [] })

    const linesSuggestedAcceptedChartData = ref<{
      labels: string[]
      datasets: any[]
    }>({ labels: [], datasets: [] })

    const chatTurnsAcceptancesChartData = ref<{
      labels: string[]
      datasets: any[]
    }>({ labels: [], datasets: [] })

    //Chart Options
    const chartOptions: ChartOptions<'line'> & LineControllerChartOptions = {
      responsive: true,
      maintainAspectRatio: false,
      interaction: {
        mode: 'index',
        intersect: false
      },
      plugins: {
        tooltip: {
          enabled: true,
          mode: 'index',
          intersect: false,
          callbacks: {
            label: function (context) {
              const label = context.dataset.label || ''
              const value = context.parsed.y

              if (label) {
                if (label === 'Acceptance Rate') {
                  return `${label}: ${value.toFixed(3)}%`
                }

                return `${label}: ${value.toLocaleString()}`
              }

              return ''
            }
          }
        }
      },
      scales: {
        // Primary Y-axis for Total Lines Suggested and Accepted
        y: {
          beginAtZero: true,
          ticks: {
            stepSize: 5000 // Customize this based on your data range
          },
          title: {
            display: true,
            text: 'Suggested / Accepted'
          }
        },
        // Secondary Y-axis for Acceptance Rate
        y1: {
          beginAtZero: true,
          position: 'right', // Position this axis on the right side
          ticks: {
            callback: function (value: string | number) {
              return value + '%' // Add percentage symbol to the ticks
            }
          },
          title: {
            display: true,
            text: 'Acceptance Rate (%)'
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
          top: 50
        }
      },
      spanGaps: false,
      showLine: true
    }

    const dateRange = ref({
      start: subMonths(new Date(), 1), // Default to last month
      end: new Date()
    })

    const selectedPeriod = ref('monthly')

    const calculateTrend = (currentValue: number, previousValue: number): number => {
      if (previousValue === 0) return 0
      // Round to 2 decimal places
      return Number((((currentValue - previousValue) / previousValue) * 100).toFixed(2))
    }

    const updateDateRange = () => {
      if (!dateRange.value.start || !dateRange.value.end) {
        return
      }

      if (dateRange.value.start > dateRange.value.end) {
        toast.error('Start date cannot be after end date')
        dateRange.value = {
          start: subMonths(new Date(), 1),
          end: new Date()
        }
        return
      }

      updateTeamData(selectedTeam.value)
    }

    const updatePeriodData = (period: string) => {
      selectedPeriod.value = period
      const now = new Date()

      switch (period) {
        case 'weekly':
          dateRange.value = {
            start: addDays(now, -7),
            end: now
          }
          break
        case 'monthly':
          dateRange.value = {
            start: subMonths(now, 1),
            end: now
          }
          break
        case 'yearly':
          dateRange.value = {
            start: new Date(now.getFullYear() - 1, now.getMonth(), now.getDate()),
            end: now
          }
          break
      }

      updateTeamData(selectedTeam.value)
    }

    const updateTeamData = (team: string) => {
      selectedTeam.value = team
      const teamData = props.metrics.find((m: TeamMetrics) => m.team_tag === team)
      teamMembers.value = teamData ? teamData.members : []

      // Filter metrics by date range
      filteredMetrics.value = teamData
        ? teamData.metrics.filter((m: any) => {
            const date = new Date(m.day)
            return date >= dateRange.value.start && date <= dateRange.value.end
          })
        : []

      if (filteredMetrics.value.length === 0) {
        console.warn(`No metrics found for team: ${team} in selected date range`)
        toast.warning(`No metrics found for team-slug: ${team} in selected date range`)
        return
      }

      // Calculate current period metrics
      const currentPeriodMetrics = calculatePeriodMetrics(filteredMetrics.value)

      // Calculate previous period metrics for trend
      const periodDays = Math.floor(
        (dateRange.value.end.getTime() - dateRange.value.start.getTime()) / (1000 * 60 * 60 * 24)
      )
      const previousStart = addDays(dateRange.value.start, -periodDays)
      const previousEnd = addDays(dateRange.value.end, -periodDays)

      const previousPeriodMetrics = teamData
        ? teamData.metrics.filter((m: any) => {
            const date = new Date(m.day)
            return date >= previousStart && date <= previousEnd
          })
        : []

      const previousMetrics = calculatePeriodMetrics(previousPeriodMetrics)

      // Update team metrics with trends
      teamMetrics.value = {
        ...currentPeriodMetrics,
        suggestionsTrend: calculateTrend(
          currentPeriodMetrics.totalSuggestionsCount,
          previousMetrics.totalSuggestionsCount
        ),
        acceptancesTrend: calculateTrend(
          currentPeriodMetrics.totalAcceptancesCount,
          previousMetrics.totalAcceptancesCount
        ),
        activeUsersTrend: calculateTrend(
          currentPeriodMetrics.totalActiveUsers,
          previousMetrics.totalActiveUsers
        ),
        chatAcceptancesTrend: calculateTrend(
          currentPeriodMetrics.totalChatAcceptances,
          previousMetrics.totalChatAcceptances
        )
      }

      filteredMetricsKey.value++
      updateChartDatasets(filteredMetrics.value)
    }

    const calculatePeriodMetrics = (metrics: any[]) => {
      return metrics.reduce(
        (acc: any, curr: any) => ({
          totalSuggestionsCount: acc.totalSuggestionsCount + curr.total_suggestions_count,
          totalAcceptancesCount: acc.totalAcceptancesCount + curr.total_acceptances_count,
          totalLinesSuggested: acc.totalLinesSuggested + curr.total_lines_suggested,
          totalLinesAccepted: acc.totalLinesAccepted + curr.total_lines_accepted,
          totalActiveUsers: Math.max(acc.totalActiveUsers, curr.total_active_users),
          totalChatAcceptances: acc.totalChatAcceptances + curr.total_chat_acceptances,
          totalChatTurns: acc.totalChatTurns + curr.total_chat_turns,
          totalActiveChatUsers: Math.max(acc.totalActiveChatUsers, curr.total_active_chat_users)
        }),
        {
          totalSuggestionsCount: 0,
          totalAcceptancesCount: 0,
          totalLinesSuggested: 0,
          totalLinesAccepted: 0,
          totalActiveUsers: 0,
          totalChatAcceptances: 0,
          totalChatTurns: 0,
          totalActiveChatUsers: 0
        }
      )
    }

    const updateChartDatasets = (filteredMetrics: any[]) => {
      // Calculate acceptance rate
      filteredMetrics.forEach((m: any) => {
        m.acceptance_rate =
          m.total_suggestions_count > 0
            ? (m.total_acceptances_count / m.total_suggestions_count) * 100
            : 0
      })

      // Calculate lines acceptance rate
      filteredMetrics.forEach((m: any) => {
        m.lines_acceptance_rate =
          m.total_lines_suggested > 0 ? (m.total_lines_accepted / m.total_lines_suggested) * 100 : 0
      })

      // Calculate chat acceptance rate
      filteredMetrics.forEach((m: any) => {
        m.chat_acceptance_rate =
          m.total_chat_turns > 0 ? (m.total_chat_acceptances / m.total_chat_turns) * 100 : 0
      })

      // Update suggestions and acceptances over time
      suggestionsAcceptancesChartData.value = {
        labels: filteredMetrics.map((m: any) => m.day),
        datasets: [
          {
            label: 'Total Suggestions',
            data: filteredMetrics.map((m: any) => ({
              x: m.day,
              y: m.total_suggestions_count || 0
            })),
            backgroundColor: 'rgba(75, 192, 192, 0.6)',
            borderColor: 'rgb(75, 192, 192)',
            type: 'bar',
            yAxisID: 'y',
            order: 2,
            barPercentage: 0.8,
            categoryPercentage: 0.9,
            base: 0
          },
          {
            label: 'Acceptances',
            data: filteredMetrics.map((m: any) => ({
              x: m.day,
              y: m.total_acceptances_count || 0
            })),
            backgroundColor: 'rgba(255, 99, 132, 0.6)',
            borderColor: 'rgb(255, 99, 132)',
            type: 'bar',
            yAxisID: 'y',
            order: 1
          },
          {
            label: 'Acceptance Rate',
            data: filteredMetrics.map((m: any) => m.acceptance_rate),
            backgroundColor: 'rgba(54, 162, 235, 0.6)',
            borderColor: 'rgb(54, 162, 235)',
            tension: 0.1,
            pointStyle: 'circle',
            pointHoverRadius: 10,
            type: 'line',
            yAxisID: 'y1'
          }
        ]
      }

      // Update lines suggested vs accepted
      linesSuggestedAcceptedChartData.value = {
        labels: filteredMetrics.map((m: any) => m.day),
        datasets: [
          {
            label: 'Lines Suggested',
            data: filteredMetrics.map((m: any) => m.total_lines_suggested),
            backgroundColor: 'rgba(75, 192, 192, 0.6)',
            borderColor: 'rgb(75, 192, 192)',
            type: 'bar',
            yAxisID: 'y'
          },
          {
            label: 'Lines Accepted',
            data: filteredMetrics.map((m: any) => m.total_lines_accepted),
            backgroundColor: 'rgba(255, 99, 132, 0.6)',
            borderColor: 'rgb(255, 99, 132)',
            type: 'bar',
            yAxisID: 'y'
          },
          {
            label: 'Lines Acceptance Rate',
            data: filteredMetrics.map((m: any) => m.lines_acceptance_rate),
            backgroundColor: 'rgba(54, 162, 235, 0.6)',
            borderColor: 'rgb(54, 162, 235)',
            tension: 0.1,
            pointStyle: 'circle',
            pointHoverRadius: 10,
            type: 'line',
            yAxisID: 'y1'
          }
        ]
      }

      // Update chat turns and acceptances over time
      chatTurnsAcceptancesChartData.value = {
        labels: filteredMetrics.map((m: any) => m.day),
        datasets: [
          {
            label: 'Chat Turns',
            data: filteredMetrics.map((m: any) => m.total_chat_turns),
            backgroundColor: 'rgba(75, 192, 192, 0.6)',
            borderColor: 'rgb(153, 102, 255)',
            type: 'bar',
            yAxisID: 'y'
          },
          {
            label: 'Chat Acceptances',
            data: filteredMetrics.map((m: any) => m.total_chat_acceptances),
            backgroundColor: 'rgba(255, 99, 132, 0.6)',
            borderColor: 'rgb(255, 159, 64)',
            type: 'bar',
            yAxisID: 'y'
          },
          {
            label: 'Chat Acceptance Rate',
            data: filteredMetrics.map((m: any) => m.chat_acceptance_rate),
            backgroundColor: 'rgba(54, 162, 235, 0.6)',
            borderColor: 'rgb(54, 162, 235)',
            tension: 0.1,
            pointStyle: 'circle',
            pointHoverRadius: 10,
            type: 'line',
            yAxisID: 'y1'
          }
        ]
      }
    }

    const uniqueTeams = computed(() => {
      return [...new Set(props.teams)]
    })

    watch(selectedTeam, updateTeamData, { immediate: true })

    return {
      teamMembers,
      selectedTeam,
      selectedPeriod,
      dateRange,
      teamMetrics,
      chartOptions,
      updateTeamData,
      updatePeriodData,
      updateDateRange,
      suggestionsAcceptancesChartData,
      linesSuggestedAcceptedChartData,
      chatTurnsAcceptancesChartData,
      filteredMetrics,
      filteredMetricsKey,
      cn,
      format,
      uniqueTeams
    }
  }
})
</script>
<style scoped>
/* Replace >>> or /deep/ with :deep() */
:deep(.v-card) {
  height: 100%;
}

:deep(.metric-card) {
  height: 100%;
}

.dashboard-container {
  margin-bottom: 30px;
}

.charts-container {
  margin-top: 30px;
}

.metric-card {
  height: 100%;
}

.card-content {
  text-align: center;
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

.chart-card {
  margin-bottom: 20px;
}
</style>
