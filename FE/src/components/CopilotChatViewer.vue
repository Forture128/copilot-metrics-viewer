<template>
  <div class="space-y-6">
    <!-- Dashboard Container for Metrics -->
    <div class="grid gap-4 grid-cols-12">
      <MetricCard
        title="Cumulative Number of Turns"
        subtitle="Over the last 28 days"
        :value="cumulativeNumberTurns"
        icon="mdi-message-text"
        :trend="4.2"
        description="4.2% increase from last month"
      />

      <!-- Cumulative Number of Acceptances Card -->

      <MetricCard
        title="Cumulative Number of Acceptances"
        subtitle="Over the last 28 days"
        :value="cumulativeNumberAcceptances"
        icon="mdi-checkbox-marked-circle-outline"
        :trend="3.8"
        description="3.8% increase from last month"
      />
    </div>

    <!-- Charts Section -->
    <v-container fluid class="charts-container">
      <!-- Total Acceptances | Total Turns Count Chart -->
      <FullWidthChart
        title="Total Acceptances | Total Turns Count"
        :data="totalNumberAcceptancesAndTurnsChartData"
        :options="chartOptions"
      />

      <!-- Total Active Copilot Chat Users Chart -->
      <FullWidthChart
        title="Total Active Copilot Chat Users"
        :data="totalActiveCopilotChatUsersChartData"
        :options="totalActiveChatUsersChartOptions"
      />
    </v-container>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, toRef } from 'vue'
import type { Metrics } from '../model/Metrics'
import FullWidthChart from './Commons/FullWidthChart.vue' // Assuming you have this component
import MetricCard from './Commons/MetricCard.vue'
import {
  Chart as ChartJS,
  ArcElement,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend
} from 'chart.js'

ChartJS.register(
  ArcElement,
  CategoryScale,
  LinearScale,
  BarElement,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend
)

export default defineComponent({
  name: 'CopilotChatViewer',
  components: {
    FullWidthChart,
    MetricCard
  },
  props: {
    metrics: {
      type: Object,
      required: true
    }
  },
  setup(props) {
    const cumulativeNumberAcceptances = ref(0)
    const cumulativeNumberTurns = ref(0)

    // Total Copilot Chat Active Users Chart Data
    const totalActiveCopilotChatUsersChartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: []
    })

    const totalActiveChatUsersChartOptions = {
      responsive: true,
      maintainAspectRatio: false,
      scales: {
        y: {
          beginAtZero: true,
          ticks: {
            stepSize: 1
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
      }
    }

    const chartOptions = {
      responsive: true,
      maintainAspectRatio: false,
      layout: {
        padding: {
          left: 50,
          right: 50,
          top: 20,
          bottom: 40
        }
      }
    }

    // Total Number Acceptances and Turns Chart Data
    const totalNumberAcceptancesAndTurnsChartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: []
    })

    const data = toRef(props, 'metrics').value

    // Calculate cumulative numbers
    cumulativeNumberTurns.value = 0
    const cumulativeNumberTurnsData = data.map((m: Metrics) => {
      cumulativeNumberTurns.value += m.total_chat_turns
      return m.total_chat_turns
    })

    cumulativeNumberAcceptances.value = 0
    const cumulativeNumberAcceptancesData = data.map((m: Metrics) => {
      cumulativeNumberAcceptances.value += m.total_chat_acceptances
      return m.total_chat_acceptances
    })

    totalNumberAcceptancesAndTurnsChartData.value = {
      labels: data.map((m: Metrics) => m.day),
      datasets: [
        {
          label: 'Total Acceptances',
          data: cumulativeNumberAcceptancesData,
          backgroundColor: 'rgba(75, 192, 192, 0.2)',
          borderColor: 'rgba(75, 192, 192, 1)',
          fill: true
        },
        {
          label: 'Total Turns',
          data: cumulativeNumberTurnsData,
          backgroundColor: 'rgba(153, 102, 255, 0.2)',
          borderColor: 'rgba(153, 102, 255, 1)',
          fill: true
        }
      ]
    }

    totalActiveCopilotChatUsersChartData.value = {
      labels: data.map((m: Metrics) => m.day),
      datasets: [
        {
          label: 'Total Active Copilot Chat Users',
          data: data.map((m: Metrics) => m.total_active_chat_users),
          backgroundColor: 'rgba(0, 0, 139, 0.2)', // dark blue with 20% opacity
          borderColor: 'rgba(255, 99, 132, 1)',
          fill: true
        }
      ]
    }

    return {
      totalActiveCopilotChatUsersChartData,
      totalActiveChatUsersChartOptions,
      cumulativeNumberAcceptances,
      cumulativeNumberTurns,
      totalNumberAcceptancesAndTurnsChartData,
      chartOptions
    }
  }
})
</script>

<style scoped>
.chart-container {
  padding: 20px;
}

.v-card-title {
  font-size: 1.2em;
  font-weight: bold;
  margin-bottom: 10px;
  text-align: center;
}
</style>
