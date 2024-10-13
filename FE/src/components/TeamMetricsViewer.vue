<template>
  <div>
    <!-- Team Selection -->
    <v-container fluid class="selection-container">
      <v-row justify="center">
        <v-col cols="12" sm="6" md="4">
          <v-select v-model="selectedTeam" :items="teams" label="Select Team" @change="updateTeamData"></v-select>
        </v-col>
      </v-row>
    </v-container>

    <h1 class="text-center">Overview Team Metrics</h1>
    <!-- Dashboard Container for Team Metrics -->
    <v-container fluid class="dashboard-container">
      <v-row justify="center" align="center">
        <!-- Total Suggestions Count Card -->
        <v-col cols="12" sm="6" md="3">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Total Suggestions Count</div>
                <div class="metric-subtitle">For {{ selectedTeam }}</div>
                <p class="metric-value">{{ teamMetrics.totalSuggestionsCount }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>

        <!-- Total Acceptances Count Card -->
        <v-col cols="12" sm="6" md="3">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Total Acceptances Count</div>
                <div class="metric-subtitle">For {{ selectedTeam }}</div>
                <p class="metric-value">{{ teamMetrics.totalAcceptancesCount }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>

        <!-- Total Active Users Card -->
        <v-col cols="12" sm="6" md="3">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Latest Active Users</div>
                <div class="metric-subtitle">For {{ selectedTeam }}</div>
                <p class="metric-value">{{ teamMetrics.totalActiveUsers }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>

        <!-- Total Active Chat Users Card -->
        <v-col cols="12" sm="6" md="3">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Latest Active Chat Users</div>
                <div class="metric-subtitle">For {{ selectedTeam }}</div>
                <p class="metric-value">{{ teamMetrics.totalActiveChatUsers }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>

        <!-- Total Chat Acceptances Card -->
        <v-col cols="12" sm="6" md="3">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Total Chat Acceptances</div>
                <div class="metric-subtitle">For {{ selectedTeam }}</div>
                <p class="metric-value">{{ teamMetrics.totalChatAcceptances }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>
    </v-container>

    <!-- Charts Section -->
    <v-container fluid class="charts-container">
      <v-row>
        <v-col cols="12">
          <h2 class="text-center">Suggestions and Acceptances Over Time</h2>
          <v-card class="chart-card">
            <v-card-item>
              <Line :data="suggestionsAcceptancesChartData" :options="chartOptions" />
            </v-card-item>
          </v-card>
        </v-col>

      </v-row>
      <v-row>
        <v-col cols="12">
          <h2 class="text-center">Lines Suggested vs Accepted Over Time</h2>
          <v-card class="chart-card">
            <v-card-item>
              <Line :data="linesSuggestedAcceptedChartData" :options="chartOptions" />
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>

      <v-row>
        <v-col cols="12">
          <h2 class="text-center">Chat Turns and Acceptances Over Time</h2>
          <v-card class="chart-card">
            <v-card-item>
              <Line :data="chatTurnsAcceptancesChartData" :options="chartOptions" />
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
    <!-- Breakdown by Language -->
    <h1 class="text-center">Breakdown by Language</h1>
    <v-container fluid class="dashboard-container">
      <!-- Add :key to force re-render when filteredMetrics changes -->
      <TeamBreakdownComponent :metrics="filteredMetrics" :breakdownKey="'language'" :key="filteredMetricsKey" />
    </v-container>

    <!-- Breakdown by Editor -->
    <h1 class="text-center">Breakdown by Editor</h1>
    <v-container fluid class="dashboard-container">
      <!-- Add :key to force re-render when filteredMetrics changes -->
      <TeamBreakdownComponent :metrics="filteredMetrics" :breakdownKey="'editor'" :key="filteredMetricsKey" />
    </v-container>
  </div>
</template>

<script lang="ts">
import { useToast } from "vue-toastification";
import { defineComponent, ref, watch } from 'vue';
import { Line } from 'vue-chartjs';
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
  ChartOptions,
  Filler,
} from 'chart.js';
import { TeamMetrics } from '@/model/Metrics';
import TeamBreakdownComponent from "./TeamBreakdownComponent.vue";

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
);

export default defineComponent({
  name: 'TeamMetricsViewer',
  components: {
    Line,
    TeamBreakdownComponent
  },
  props: {
    teams: {
      type: Array,
      required: true,
    },
    metrics: {
      type: Array as () => TeamMetrics[],
      required: true,
    },
  },
  setup(props) {
    const toast = useToast();
    const selectedTeam = ref('rci-mfv');
    const filteredMetrics = ref<any[]>([]);
    const filteredMetricsKey = ref(0); 
    const teamMetrics = ref({
      totalSuggestionsCount: 0,
      totalAcceptancesCount: 0,
      totalLinesSuggested: 0,
      totalLinesAccepted: 0,
      totalActiveUsers: 0,
      totalChatAcceptances: 0,
      totalChatTurns: 0,
      totalActiveChatUsers: 0,
    });
    const suggestionsAcceptancesChartData = ref<{
      labels: string[];
      datasets: any[];
    }>({ labels: [], datasets: [] });

    const linesSuggestedAcceptedChartData = ref<{
      labels: string[];
      datasets: any[];
    }>({ labels: [], datasets: [] });

    const chatTurnsAcceptancesChartData = ref<{
      labels: string[];
      datasets: any[];
    }>({ labels: [], datasets: [] });

    //Chart Options
    const chartOptions: ChartOptions<"bar" | "line"> = {
      responsive: true,
      maintainAspectRatio: false,
      scales: {
        // Primary Y-axis for Total Lines Suggested and Accepted
        y: {
          beginAtZero: true,
          ticks: {
            stepSize: 5000, // Customize this based on your data range
          },
          title: {
            display: true,
            text: "Suggested / Accepted",
          },
        },
        // Secondary Y-axis for Acceptance Rate
        y1: {
          beginAtZero: true,
          position: "right", // Position this axis on the right side
          ticks: {
            callback: function (value: string | number) {
              return value + "%"; // Add percentage symbol to the ticks
            },
          },
          title: {
            display: true,
            text: "Acceptance Rate (%)",
          },
        },
        x: {
          type: "category",
          ticks: {
            autoSkip: true,
            maxTicksLimit: 10,
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

    const updateTeamData = () => {
      const teamData = props.metrics.find((m: TeamMetrics) => m.team_tag === selectedTeam.value);
      filteredMetrics.value = teamData ? teamData.metrics : [];

      if (filteredMetrics.value.length === 0) {
        console.warn(`No metrics found for team: ${selectedTeam.value}`);
        toast.warning(`No metrics found for team-slug: ${selectedTeam.value}`);
      }

      // Calculate team metrics
      teamMetrics.value = filteredMetrics.value.reduce((acc: {
        totalSuggestionsCount: number;
        totalAcceptancesCount: number;
        totalLinesSuggested: number;
        totalLinesAccepted: number;
        totalActiveUsers: number;
        totalChatAcceptances: number;
        totalChatTurns: number;
        totalActiveChatUsers: number;
      }, curr: any) => {
        acc.totalSuggestionsCount += curr.total_suggestions_count;
        acc.totalAcceptancesCount += curr.total_acceptances_count;
        acc.totalLinesSuggested += curr.total_lines_suggested;
        acc.totalLinesAccepted += curr.total_lines_accepted;
        acc.totalChatAcceptances += curr.total_chat_acceptances;
        acc.totalChatTurns += curr.total_chat_turns;

        // Get last day's active users
        acc.totalActiveUsers = curr.total_active_users;
        acc.totalActiveChatUsers = curr.total_active_chat_users;
        return acc;
      }, {
        totalSuggestionsCount: 0,
        totalAcceptancesCount: 0,
        totalLinesSuggested: 0,
        totalLinesAccepted: 0,
        totalActiveUsers: 0,
        totalChatAcceptances: 0,
        totalChatTurns: 0,
        totalActiveChatUsers: 0
      });
      filteredMetricsKey.value++;

      // Update charts based on filtered metrics
      updateChartDatasets(filteredMetrics.value);
      // updateUserEngagementFunnelData(teamMetrics.value);
    };

    const updateChartDatasets = (filteredMetrics: any[]) => {
      console.log("updateChartDatasets | filteredMetrics ");
      // Calculate acceptance rate
      filteredMetrics.forEach((m: any) => {
        m.acceptance_rate = m.total_suggestions_count > 0 ? (m.total_acceptances_count / m.total_suggestions_count) * 100 : 0;
      });

      // Calculate lines acceptance rate
      filteredMetrics.forEach((m: any) => {
        m.lines_acceptance_rate = m.total_lines_suggested > 0 ? (m.total_lines_accepted / m.total_lines_suggested) * 100 : 0;
      });

      // Calculate chat acceptance rate
      filteredMetrics.forEach((m: any) => {
        m.chat_acceptance_rate = m.total_chat_turns > 0 ? (m.total_chat_acceptances / m.total_chat_turns) * 100 : 0;
      });

      // Update suggestions and acceptances over time
      suggestionsAcceptancesChartData.value = {
        labels: filteredMetrics.map((m: any) => m.day),
        datasets: [
          {
            label: 'Suggestions',
            data: filteredMetrics.map((m: any) => m.total_suggestions_count),
            backgroundColor: 'rgba(75, 192, 192, 0.6)',
            borderColor: 'rgb(75, 192, 192)',
            type: "bar",
            yAxisID: "y",
          },
          {
            label: 'Acceptances',
            data: filteredMetrics.map((m: any) => m.total_acceptances_count),
            backgroundColor: 'rgba(255, 99, 132, 0.6)',
            borderColor: 'rgb(255, 99, 132)',
            type: "bar",
            yAxisID: "y",
          },
          {
            label: 'Acceptance Rate',
            data: filteredMetrics.map((m: any) => m.acceptance_rate),
            backgroundColor: 'rgba(54, 162, 235, 0.6)',
            borderColor: 'rgb(54, 162, 235)',
            tension: 0.1,
            pointStyle: 'circle',
            pointHoverRadius: 10,
            type: "line",
            yAxisID: "y1",
          }
        ]
      };

      // Update lines suggested vs accepted
      linesSuggestedAcceptedChartData.value = {
        labels: filteredMetrics.map((m: any) => m.day),
        datasets: [
          {
            label: 'Lines Suggested',
            data: filteredMetrics.map((m: any) => m.total_lines_suggested),
            backgroundColor: 'rgba(75, 192, 192, 0.6)',
            borderColor: 'rgb(75, 192, 192)',
            type: "bar",
            yAxisID: "y",
          },
          {
            label: 'Lines Accepted',
            data: filteredMetrics.map((m: any) => m.total_lines_accepted),
            backgroundColor: 'rgba(255, 99, 132, 0.6)',
            borderColor: 'rgb(255, 99, 132)',
            type: "bar",
            yAxisID: "y",
          },
          {
            label: 'Lines Acceptance Rate',
            data: filteredMetrics.map((m: any) => m.lines_acceptance_rate),
            backgroundColor: 'rgba(54, 162, 235, 0.6)',
            borderColor: 'rgb(54, 162, 235)',
            tension: 0.1,
            pointStyle: 'circle',
            pointHoverRadius: 10,
            type: "line",
            yAxisID: "y1",
          }
        ]
      };

      // Update chat turns and acceptances over time
      chatTurnsAcceptancesChartData.value = {
        labels: filteredMetrics.map((m: any) => m.day),
        datasets: [
          {
            label: 'Chat Turns',
            data: filteredMetrics.map((m: any) => m.total_chat_turns),
            backgroundColor: 'rgba(75, 192, 192, 0.6)',
            borderColor: 'rgb(153, 102, 255)',
            type: "bar",
            yAxisID: "y",
          },
          {
            label: 'Chat Acceptances',
            data: filteredMetrics.map((m: any) => m.total_chat_acceptances),
            backgroundColor: 'rgba(255, 99, 132, 0.6)',
            borderColor: 'rgb(255, 159, 64)',
            type: "bar",
            yAxisID: "y",
          },
          {
            label: 'Chat Acceptance Rate',
            data: filteredMetrics.map((m: any) => m.chat_acceptance_rate),
            backgroundColor: 'rgba(54, 162, 235, 0.6)',
            borderColor: 'rgb(54, 162, 235)',
            tension: 0.1,
            pointStyle: 'circle',
            pointHoverRadius: 10,
            type: "line",
            yAxisID: "y1",
          }
        ]
      };
    };


    watch(selectedTeam, updateTeamData, { immediate: true });

    return {
      selectedTeam,
      teamMetrics,
      chartOptions,
      // funnelChartOptions,
      updateTeamData,
      suggestionsAcceptancesChartData,
      linesSuggestedAcceptedChartData,
      chatTurnsAcceptancesChartData,
      filteredMetrics,
      filteredMetricsKey,
    };
  }
});
</script>

<style scoped>
.selection-container {
  margin-bottom: 20px;
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