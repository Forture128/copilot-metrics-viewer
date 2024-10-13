<template>
  <div>
    <!-- Dashboard Container for Metrics -->
    <v-container fluid class="dashboard-container">
      <v-row>
        <!-- Acceptance Rate Average Card -->
        <v-col cols="12" sm="6" md="3">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Acceptance Rate Average</div>
                <div class="metric-subtitle">Over the last 28 days</div>
                <p class="metric-value">
                  {{ acceptanceRateAverage.toFixed(2) }}%
                </p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>
        <!-- Cumulative Number of Suggestions Card -->
        <v-col cols="12" sm="6" md="3">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Cumulative Number of Suggestions</div>
                <div class="metric-subtitle">Over the last 28 days</div>
                <p class="metric-value">{{ cumulativeNumberSuggestions }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>
        <!-- Cumulative Number of Acceptances Card -->
        <v-col cols="12" sm="6" md="3">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">
                  Cumulative Number of Accepted Prompts
                </div>
                <div class="metric-subtitle">Over the last 28 days</div>
                <p class="metric-value">{{ cumulativeNumberAcceptances }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>

        <!-- Cumulative Number of Lines of Code Accepted -->
        <v-col cols="12" sm="6" md="3">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">
                  Cumulative Number of Lines of Code Accepted
                </div>
                <div class="metric-subtitle">Over the last 28 days</div>
                <p class="metric-value">{{ cumulativeNumberLOCAccepted }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>
    </v-container>

    <!-- Combined Charts Section -->
    <v-container fluid class="charts-container">
      <!-- New Line Chart for Acceptance Rates by Teams -->
      <v-row>
        <v-col cols="12">
          <h3 class="text-center">Team Acceptance Rates</h3>
          <v-card class="chart-card">
            <Line :data="teamAcceptanceRateChartData" :options="teamAcceptanceRateChartOptions" />
          </v-card>
        </v-col>
      </v-row>

      <!-- Total Suggestions Count | Total Acceptances Count | Acceptance Rate (%) -->
      <v-row>
        <v-col cols="12">
          <h3 class="text-center">
            Total Suggestions Count | Total Acceptances Count | Acceptance Rate
            (%)
          </h3>
          <v-card class="chart-card">
            <Line :data="totalSuggestionsAndAcceptanceChartData" :options="chartOptions" />
          </v-card>
        </v-col>
      </v-row>

      <!-- Total Lines Suggested | Total Lines Accepted | Acceptance Lines Rate (%) -->
      <v-row>
        <v-col cols="12">
          <h3 class="text-center">
            Total Lines Suggested | Total Lines Accepted | Acceptance Lines Rate
            (%)
          </h3>
          <v-card class="chart-card">
            <Line :data="chartData" :options="chartOptions" />
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, toRef } from "vue";
import { Metrics, TeamMetrics } from "../model/Metrics";
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
} from "chart.js";

import { Line } from "vue-chartjs";
import { ChartOptions } from "chart.js";

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
  name: "MetricsViewer",
  props: {
    metrics: {
      type: Object,
      required: true,
    },
    teamMetrics: {
      type: Array as () => TeamMetrics[],
      required: true,
    },
  },
  components: {
    Line,
  },
  setup(props) {
    console.log("MetricsViewer props", props.teamMetrics);

    // Tiles
    const acceptanceRateAverage = ref(0);
    const cumulativeNumberSuggestions = ref(0);
    const cumulativeNumberAcceptances = ref(0);
    const cumulativeNumberLOCAccepted = ref(0);

    // Chart Data
    const teamAcceptanceRateChartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: [],
    });

    const acceptanceRateChartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: [],
    });

    const totalSuggestionsAndAcceptanceChartData = ref<{
      labels: string[];
      datasets: any[];
    }>({ labels: [], datasets: [] });

    const chartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: [],
    });

    const totalActiveUsersChartData = ref<{
      labels: string[];
      datasets: any[];
    }>({ labels: [], datasets: [] });

    // Chart Options
    const chartOptions: ChartOptions<"bar" | "line"> = {
      responsive: true,
      maintainAspectRatio: true,
      scales: {
        y: {
          beginAtZero: true,
          ticks: {
            stepSize: 5000,
          },
          title: {
            display: true,
            text: "Suggested / Accepted",
          },
          grid: {
            display: false, // Remove y-axis grid lines
          },
        },
        y1: {
          beginAtZero: true,
          position: "right",
          ticks: {
            callback: function (value: string | number) {
              return value + "%";
            },
          },
          title: {
            display: true,
            text: "Acceptance Rate (%)",
          },
          grid: {
            display: false, // Remove y1-axis grid lines
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

    const teamAcceptanceRateChartOptions: ChartOptions<"bar" | "line"> = {
      ...chartOptions,
      plugins: {
        tooltip: {
          callbacks: {
            label: function (context) {
              let label = context.dataset.label || '';
              if (label) {
                label += ': ';
              }
              if (context.parsed.y !== null) {
                if (context.dataset.yAxisID === 'y1') {
                  label += context.parsed.y.toFixed(2) + '%';
                } else {
                  label += context.parsed.y;
                }
              }
              return label;
            }
          }
        }
      }
    };

    const data = toRef(props, "metrics").value;
    const teamMetricsData = toRef(props, "teamMetrics").value.filter((team) => team.team_tag !== "All Teams");

    // Method to calculate acceptance rate for each team
    const calculateTeamAcceptanceRate = (teamMetrics: Metrics[]) => {
      const totalSuggestions = teamMetrics.reduce(
        (sum, m) => sum + m.total_suggestions_count,
        0
      );
      const totalAcceptances = teamMetrics.reduce(
        (sum, m) => sum + m.total_acceptances_count,
        0
      );
      return totalSuggestions > 0
        ? (totalAcceptances / totalSuggestions) * 100
        : 0;
    };

    // Calculate cumulative metrics
    cumulativeNumberSuggestions.value = 0;
    const cumulativeSuggestionsData = data.map((m: Metrics) => {
      cumulativeNumberSuggestions.value += m.total_suggestions_count;
      return m.total_suggestions_count;
    });

    cumulativeNumberAcceptances.value = 0;
    const cumulativeAcceptancesData = data.map((m: Metrics) => {
      cumulativeNumberAcceptances.value += m.total_acceptances_count;
      return m.total_acceptances_count;
    });

    const acceptanceRates = data.map((m: Metrics) => {
      return m.total_suggestions_count !== 0
        ? (m.total_acceptances_count / m.total_suggestions_count) * 100
        : 0;
    });

    totalSuggestionsAndAcceptanceChartData.value = {
      labels: data.map((m: Metrics) => m.day),
      datasets: [
        {
          label: "Total Suggestions",
          data: cumulativeSuggestionsData,
          backgroundColor: 'rgba(75, 192, 192, 0.6)',
          borderColor: 'rgb(75, 192, 192)',
          type: "bar",
          yAxisID: "y",
        },
        {
          label: "Total Acceptance",
          data: cumulativeAcceptancesData,
          backgroundColor: 'rgba(255, 99, 132, 0.6)',
          borderColor: 'rgb(255, 99, 132)',
          type: "bar",
          yAxisID: "y",
        },
        {
          label: "Acceptance Rate",
          data: acceptanceRates,
          type: "line",
          yAxisID: "y1",
          backgroundColor: 'rgba(54, 162, 235, 0.6)',
          borderColor: 'rgb(54, 162, 235)',
        },
      ],
    };

    cumulativeNumberLOCAccepted.value = 0;
    const cumulativeLOCAcceptedData = data.map((m: Metrics) => {
      cumulativeNumberLOCAccepted.value += m.total_lines_accepted;
      return m.total_lines_accepted;
    });

    const acceptanceLinesRates = data.map((m: Metrics) => {
      return m.total_lines_suggested !== 0
        ? (m.total_lines_accepted / m.total_lines_suggested) * 100
        : 0;
    });

    chartData.value = {
      labels: data.map((m: Metrics) => m.day),
      datasets: [
        {
          label: "Total Lines Suggested",
          data: data.map((m: Metrics) => m.total_lines_suggested),
          backgroundColor: 'rgba(75, 192, 192, 0.6)',
          borderColor: 'rgb(75, 192, 192)',
          type: "bar",
          yAxisID: "y",
        },
        {
          label: "Total Lines Accepted",
          data: cumulativeLOCAcceptedData,
          backgroundColor: 'rgba(255, 99, 132, 0.6)',
          borderColor: 'rgb(255, 99, 132)',
          type: "bar",
          yAxisID: "y",
        },
        {
          label: "Acceptance Lines Rate",
          data: acceptanceLinesRates,
          backgroundColor: 'rgba(54, 162, 235, 0.6)',
          borderColor: 'rgb(54, 162, 235)',
          type: "line",
          fill: false,
          yAxisID: "y1",
        },
      ],
    };

    if (cumulativeNumberSuggestions.value === 0) {
      acceptanceRateAverage.value = 0;
    } else {
      acceptanceRateAverage.value =
        (cumulativeNumberAcceptances.value /
          cumulativeNumberSuggestions.value) *
        100;
    }

    totalActiveUsersChartData.value = {
      labels: data.map((m: Metrics) => m.day),
      datasets: [
        {
          label: "Total Active Users",
          data: data.map((m: Metrics) => m.total_active_users),
          backgroundColor: "rgba(0, 0, 139, 0.2)",
          borderColor: "rgba(255, 99, 132, 1)",
        },
      ],
    };

    // Prepare data for the chart
    const sortedTeams = teamMetricsData
      .map((team) => ({
        team_tag: team.team_tag,
        acceptanceRate: calculateTeamAcceptanceRate(team.metrics),
        totalSuggestion: team.metrics.reduce(
          (sum, m) => sum + m.total_suggestions_count,
          0
        ),
        totalAcceptance: team.metrics.reduce(
          (sum, m) => sum + m.total_acceptances_count,
          0
        ),
      }))
      .sort((a, b) => b.acceptanceRate - a.acceptanceRate);

    console.log("Sorted Teams: ", sortedTeams);

    // Update the chart data
    teamAcceptanceRateChartData.value = {
      labels: sortedTeams.map((team) => team.team_tag),
      datasets: [
        {
          label: "Acceptance Rate",
          data: sortedTeams.map((team) => team.acceptanceRate),
          backgroundColor: 'rgba(54, 162, 235, 0.6)',
          borderColor: 'rgb(54, 162, 235)',
          fill: false,
          type: "line",
          yAxisID: "y1",
        },
        {
          label: "Total Suggestions",
          data: sortedTeams.map((team) => team.totalSuggestion),
          backgroundColor: 'rgba(75, 192, 192, 0.6)',
          borderColor: 'rgb(75, 192, 192)',
          type: "bar",
          yAxisID: "y",
        },
        {
          label: "Total Acceptances",
          data: sortedTeams.map((team) => team.totalAcceptance),
          backgroundColor: 'rgba(255, 99, 132, 0.6)',
          borderColor: 'rgb(255, 99, 132)',
          type: "bar",
          yAxisID: "y",
        },
      ],
    };

    return {
      totalSuggestionsAndAcceptanceChartData,
      chartData,
      chartOptions,
      totalActiveUsersChartData,
      acceptanceRateChartData,
      acceptanceRateAverage,
      cumulativeNumberSuggestions,
      cumulativeNumberAcceptances,
      cumulativeNumberLOCAccepted,
      teamAcceptanceRateChartData,
      teamAcceptanceRateChartOptions,
    };
  },
});
</script>
