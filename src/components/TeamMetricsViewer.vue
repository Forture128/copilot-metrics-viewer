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
                <div class="metric-title">Total Active Users</div>
                <div class="metric-subtitle">For {{ selectedTeam }}</div>
                <p class="metric-value">{{ teamMetrics.totalActiveUsers }}</p>
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
        <v-col cols="12" md="6">
          <h3 class="text-center">Suggestions and Acceptances Over Time</h3>
          <v-card class="chart-card">
            <v-card-item>
              <Line :data="suggestionsAcceptancesChartData" :options="chartOptions" />
            </v-card-item>
          </v-card>
        </v-col>

        <v-col cols="12" md="6">
          <h3 class="text-center">Lines Suggested vs Accepted</h3>
          <v-card class="chart-card">
            <v-card-item>
              <Bar :data="linesSuggestedAcceptedChartData" :options="chartOptions" />
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>

      <v-row>
        <v-col cols="12">
          <h3 class="text-center">Chat Turns and Acceptances Over Time</h3>
          <v-card class="chart-card">
            <v-card-item>
              <Line :data="chatTurnsAcceptancesChartData" :options="chartOptions" />
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>

<script lang="ts">
import { useToast } from "vue-toastification";
import { defineComponent, ref, watch } from 'vue';
import { Line, Bar } from 'vue-chartjs';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend
} from 'chart.js';
import { TeamMetrics } from '@/model/Metrics';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend
);

export default defineComponent({
  name: 'TeamMetricsViewer',
  components: {
    Line,
    Bar
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
    const selectedTeam = ref('All Teams');
    const teamMetrics = ref({
      totalSuggestionsCount: 0,
      totalAcceptancesCount: 0,
      totalLinesSuggested: 0,
      totalLinesAccepted: 0,
      totalActiveUsers: 0,
      totalChatAcceptances: 0,
      totalChatTurns: 0
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

    const chartOptions = {
      responsive: true,
      maintainAspectRatio: false
    };
    const toast = useToast();

    const updateTeamData = () => {
      console.log("updateTeamData | props.metrics = ", props.metrics);
      const teamData = props.metrics.find((m: TeamMetrics) => m.team_tag === selectedTeam.value);
      const filteredMetrics = teamData ? teamData.metrics : [];

      console.log("selectedTeam = ", selectedTeam.value);
      console.log("filteredMetrics = ", filteredMetrics);

      if (filteredMetrics.length === 0) {
        console.warn(`No metrics found for team: ${selectedTeam.value}`);
        toast.warning(`No metrics found for team-slug: ${selectedTeam.value}`);
        return;
      }

      // Calculate team metrics
      teamMetrics.value = filteredMetrics.reduce((acc: {
        totalSuggestionsCount: number;
        totalAcceptancesCount: number;
        totalLinesSuggested: number;
        totalLinesAccepted: number;
        totalActiveUsers: number;
        totalChatAcceptances: number;
        totalChatTurns: number;
      }, curr: any) => {
        acc.totalSuggestionsCount += curr.total_suggestions_count;
        acc.totalAcceptancesCount += curr.total_acceptances_count;
        acc.totalLinesSuggested += curr.total_lines_suggested;
        acc.totalLinesAccepted += curr.total_lines_accepted;
        acc.totalActiveUsers += curr.total_active_users;
        acc.totalChatAcceptances += curr.total_chat_acceptances;
        acc.totalChatTurns += curr.total_chat_turns;
        return acc;
      }, {
        totalSuggestionsCount: 0,
        totalAcceptancesCount: 0,
        totalLinesSuggested: 0,
        totalLinesAccepted: 0,
        totalActiveUsers: 0,
        totalChatAcceptances: 0,
        totalChatTurns: 0
      });

      // Update charts based on filtered metrics
      updateChartDatasets(filteredMetrics);
    };

    const updateChartDatasets = (filteredMetrics: any[]) => {
      console.log("updateChartDatasets | filteredMetrics ");
      // Update suggestions and acceptances over time
      suggestionsAcceptancesChartData.value = {
        labels: filteredMetrics.map((m: any) => m.day), // Adjust as needed for day labels
        datasets: [
          {
            label: 'Suggestions',
            data: filteredMetrics.map((m: any) => m.total_suggestions_count),
            borderColor: 'rgb(75, 192, 192)',
            tension: 0.1
          },
          {
            label: 'Acceptances',
            data: filteredMetrics.map((m: any) => m.total_acceptances_count),
            borderColor: 'rgb(255, 99, 132)',
            tension: 0.1
          }
        ]
      };

      // Update lines suggested vs accepted
      linesSuggestedAcceptedChartData.value = {
        labels: filteredMetrics.map((m: any) => m.day), // Adjust as needed for day labels
        datasets: [
          {
            label: 'Lines Suggested',
            data: filteredMetrics.map((m: any) => m.total_lines_suggested),
            backgroundColor: 'rgba(75, 192, 192, 0.6)'
          },
          {
            label: 'Lines Accepted',
            data: filteredMetrics.map((m: any) => m.total_lines_accepted),
            backgroundColor: 'rgba(255, 99, 132, 0.6)'
          }
        ]
      };

      // Update chat turns and acceptances over time
      chatTurnsAcceptancesChartData.value = {
        labels: filteredMetrics.map((m: any) => m.day), // Adjust as needed for day labels
        datasets: [
          {
            label: 'Chat Turns',
            data: filteredMetrics.map((m: any) => m.total_chat_turns),
            borderColor: 'rgb(153, 102, 255)',
            tension: 0.1
          },
          {
            label: 'Chat Acceptances',
            data: filteredMetrics.map((m: any) => m.total_chat_acceptances),
            borderColor: 'rgb(255, 159, 64)',
            tension: 0.1
          }
        ]
      };
    };

    watch(selectedTeam, updateTeamData, { immediate: true });

    return {
      selectedTeam,
      teamMetrics,
      chartOptions,
      updateTeamData,
      suggestionsAcceptancesChartData,
      linesSuggestedAcceptedChartData,
      chatTurnsAcceptancesChartData
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
</style>