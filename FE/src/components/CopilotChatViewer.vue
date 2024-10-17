<template>
  <div>
    <!-- Dashboard Container for Metrics -->
    <v-container fluid class="dashboard-container">
      <v-row justify="center" align="center">
        <!-- Cumulative Number of Turns Card -->
        <MetricCard
          title="Cumulative Number of Turns"
          subtitle="Over the last 28 days"
          :value="cumulativeNumberTurns"
        />

        <!-- Cumulative Number of Acceptances Card -->
        <MetricCard
          title="Cumulative Number of Acceptances"
          subtitle="Over the last 28 days"
          :value="cumulativeNumberAcceptances"
        />
      </v-row>
    </v-container>

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
import { defineComponent, ref, toRef } from 'vue';
import { Metrics } from '../model/Metrics';
import FullWidthChart from './Commons/FullWidthChart.vue'; // Assuming you have this component
import MetricCard from './Commons/MetricCard.vue'; // Assuming you have this component
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
  Legend,
} from 'chart.js';

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
);

export default defineComponent({
  name: 'CopilotChatViewer',
  props: {
    metrics: {
      type: Object,
      required: true,
    },
  },
  components: {
    FullWidthChart,
    MetricCard,
  },
  setup(props) {
    const cumulativeNumberAcceptances = ref(0);
    const cumulativeNumberTurns = ref(0);

    // Total Copilot Chat Active Users Chart Data
    const totalActiveCopilotChatUsersChartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: [],
    });

    const totalActiveChatUsersChartOptions = {
      responsive: true,
      maintainAspectRatio: false,
      scales: {
        y: {
          beginAtZero: true,
          ticks: {
            stepSize: 1,
          },
        },
      },
      layout: {
        padding: {
          left: 50,
          right: 50,
          top: 50,
          bottom: 50,
        },
      },
    };

    const chartOptions = {
      responsive: true,
      maintainAspectRatio: false,
      layout: {
        padding: {
          left: 50,
          right: 50,
          top: 20,
          bottom: 40,
        },
      },
    };

    // Total Number Acceptances and Turns Chart Data
    const totalNumberAcceptancesAndTurnsChartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: [],
    });

    const data = toRef(props, 'metrics').value;

    // Calculate cumulative numbers
    cumulativeNumberTurns.value = 0;
    const cumulativeNumberTurnsData = data.map((m: Metrics) => {
      cumulativeNumberTurns.value += m.total_chat_turns;
      return m.total_chat_turns;
    });

    cumulativeNumberAcceptances.value = 0;
    const cumulativeNumberAcceptancesData = data.map((m: Metrics) => {
      cumulativeNumberAcceptances.value += m.total_chat_acceptances;
      return m.total_chat_acceptances;
    });

    totalNumberAcceptancesAndTurnsChartData.value = {
      labels: data.map((m: Metrics) => m.day),
      datasets: [
        {
          label: 'Total Acceptances',
          data: cumulativeNumberAcceptancesData,
          backgroundColor: 'rgba(75, 192, 192, 0.2)',
          borderColor: 'rgba(75, 192, 192, 1)',
          fill: true,
        },
        {
          label: 'Total Turns',
          data: cumulativeNumberTurnsData,
          backgroundColor: 'rgba(153, 102, 255, 0.2)',
          borderColor: 'rgba(153, 102, 255, 1)',
          fill: true,
        },
      ],
    };

    totalActiveCopilotChatUsersChartData.value = {
      labels: data.map((m: Metrics) => m.day),
      datasets: [
        {
          label: 'Total Active Copilot Chat Users',
          data: data.map((m: Metrics) => m.total_active_chat_users),
          backgroundColor: 'rgba(0, 0, 139, 0.2)', // dark blue with 20% opacity
          borderColor: 'rgba(255, 99, 132, 1)',
          fill: true,
        },
      ],
    };

    return {
      totalActiveCopilotChatUsersChartData,
      totalActiveChatUsersChartOptions,
      cumulativeNumberAcceptances,
      cumulativeNumberTurns,
      totalNumberAcceptancesAndTurnsChartData,
      chartOptions,
    };
  },
});
</script>

<style scoped>
.dashboard-container {
  margin-bottom: 20px;
}

.chart-card {
  margin-bottom: 30px;
  border-radius: 12px;
}

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
