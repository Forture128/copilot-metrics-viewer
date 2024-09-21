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
      <!-- Total Active Users Chart -->
      <v-row>
        <v-col cols="12">
          <h2 class="text-center">Total Active Users</h2>
          <v-card class="chart-card">
            <Bar :data="totalActiveUsersChartData" :options="totalActiveUsersChartOptions" />
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
import { Metrics } from "../model/Metrics";
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
import { Bar } from "vue-chartjs";
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
  },
  components: {
    Line,
    Bar,
  },
  data() {
    return {
      data: {
        labels: ["VueJs", "EmberJs", "ReactJs", "AngularJs"],
        datasets: [
          {
            backgroundColor: ["#41B883", "#E46651", "#00D8FF", "#DD1B16"],
            data: [40, 20, 80, 10],
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
      },
    };
  },
  setup(props) {
    //Tiles
    const acceptanceRateAverage = ref(0);
    const cumulativeNumberSuggestions = ref(0);
    const cumulativeNumberAcceptances = ref(0);
    const cumulativeNumberLOCAccepted = ref(0);

    //Acceptance Rate
    const acceptanceRateChartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: [],
    });

    //Total Suggestions Count | Total Acceptance Counts
    const totalSuggestionsAndAcceptanceChartData = ref<{
      labels: string[];
      datasets: any[];
    }>({ labels: [], datasets: [] });

    //Total Lines Suggested | Total Lines Accepted
    const chartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: [],
    });

    //Total Active Users
    const totalActiveUsersChartData = ref<{
      labels: string[];
      datasets: any[];
    }>({ labels: [], datasets: [] });

    //Chart Options
    const chartOptions: ChartOptions<"bar" | "line"> = {
      responsive: true,
      maintainAspectRatio: true,
      scales: {
        // Primary Y-axis for Total Lines Suggested and Accepted
        y: {
          beginAtZero: true,
          ticks: {
            stepSize: 5000, // Customize this based on your data range
          },
          title: {
            display: true,
            text: "Lines Suggested / Accepted",
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

    const totalActiveUsersChartOptions = {
      responsive: true,
      maintainAspectRatio: true,
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

    const data = toRef(props, "metrics").value;

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
      const rate =
        m.total_lines_suggested !== 0
          ? (m.total_acceptances_count / m.total_suggestions_count) * 100
          : 0;
      return rate;
    });

    totalSuggestionsAndAcceptanceChartData.value = {
      labels: data.map((m: Metrics) => m.day),
      datasets: [
        {
          label: "Total Suggestions",
          data: cumulativeSuggestionsData,
          backgroundColor: "rgba(75, 192, 192, 0.2)",
          borderColor: "rgba(75, 192, 192, 1)",
          type: "bar",
          yAxisID: "y",
        },
        {
          label: "Total Acceptance",
          data: cumulativeAcceptancesData,
          backgroundColor: "rgba(153, 102, 255, 0.2)",
          borderColor: "rgba(153, 102, 255, 1)",
          type: "bar",
          yAxisID: "y",
        },
        {
          label: "Acceptance Rate",
          data: acceptanceRates,
          type: "line",
          yAxisID: "y1",
          backgroundColor: "rgba(255, 159, 64, 0.2)",
          borderColor: "rgba(255, 159, 64, 1)",
        },
      ],
    };

    cumulativeNumberLOCAccepted.value = 0;
    const cumulativeLOCAcceptedData = data.map((m: Metrics) => {
      const total_lines_accepted = m.total_lines_accepted;
      cumulativeNumberLOCAccepted.value += total_lines_accepted;
      return total_lines_accepted;
    });

    const acceptanceLinesRates = data.map((m: Metrics) => {
      const rate =
        m.total_lines_suggested !== 0
          ? (m.total_lines_accepted / m.total_lines_suggested) * 100
          : 0;
      return rate;
    });

    chartData.value = {
      labels: data.map((m: Metrics) => m.day),
      datasets: [
        {
          label: "Total Lines Suggested",
          data: data.map((m: Metrics) => m.total_lines_suggested),
          backgroundColor: "rgba(75, 192, 192, 0.2)",
          borderColor: "rgba(75, 192, 192, 1)",
          type: "bar", // Bar chart for suggestions
          yAxisID: "y", // Assign to the primary Y-axis
        },
        {
          label: "Total Lines Accepted",
          data: cumulativeLOCAcceptedData,
          backgroundColor: "rgba(153, 102, 255, 0.2)",
          borderColor: "rgba(153, 102, 255, 1)",
          type: "bar", // Bar chart for acceptances
          yAxisID: "y", // Assign to the primary Y-axis
        },
        {
          label: "Acceptance Lines Rate",
          data: acceptanceLinesRates,
          backgroundColor: "rgba(173, 216, 230, 0.2)",
          borderColor: "rgba(173, 216, 230, 1)",
          type: "line", // Line chart for acceptance rate
          fill: false,
          yAxisID: "y1", // Assign to the secondary Y-axis
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
          backgroundColor: "rgba(0, 0, 139, 0.2)", // dark blue with 20% opacity
          borderColor: "rgba(255, 99, 132, 1)",
        },
      ],
    };

    return {
      totalSuggestionsAndAcceptanceChartData,
      chartData,
      chartOptions,
      totalActiveUsersChartData,
      totalActiveUsersChartOptions,
      acceptanceRateChartData,
      acceptanceRateAverage,
      cumulativeNumberSuggestions,
      cumulativeNumberAcceptances,
      cumulativeNumberLOCAccepted,
    };
  },
});
</script>
