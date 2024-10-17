<template>
  <div>
    <!-- Dashboard Container for Metrics -->
    <v-container fluid class="dashboard-container">
      <v-row>
        <MetricCard title="Acceptance Rate Average" subtitle="Over the last 28 days"
          :value="acceptanceRateAverage.toFixed(2) + '%'" icon="mdi-chart-areaspline" />
        <MetricCard title="Cumulative Number of Suggestions" subtitle="Over the last 28 days"
          :value="cumulativeNumberSuggestions" icon="mdi-lightbulb-outline" />
        <MetricCard title="Cumulative Number of Accepted Prompts" subtitle="Over the last 28 days"
          :value="cumulativeNumberAcceptances" icon="mdi-checkbox-marked-circle-outline" />
        <MetricCard title="Cumulative Number of Lines of Code Accepted" subtitle="Over the last 28 days"
          :value="cumulativeNumberLOCAccepted" icon="mdi-code-tags" />
      </v-row>
    </v-container>

    <!-- Full-Width Chart Section for Larger, Detailed Charts -->
    <v-container fluid class="charts-container">
      <v-row>
        <FullWidthChart title="Total Suggestions Count | Total Acceptances Count | Acceptance Rate (%)"
          :data="totalSuggestionsAndAcceptanceChartData" :options="chartOptions" />
      </v-row>

      <v-row>
        <FullWidthChart title="Total Lines Suggested | Total Lines Accepted | Acceptance Lines Rate (%)"
          :data="chartData" :options="chartOptions" />
      </v-row>
    </v-container>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { Metrics } from "../model/Metrics";
import { calculateCumulativeMetrics } from "@/utils/MetricUtils";
import FullWidthChart from "./Commons/FullWidthChart.vue";  // FullWidthChart for larger charts
import MetricCard from "./Commons/MetricCard.vue";  // MetricCard for mini metrics
import { ChartOptions, LineControllerChartOptions } from 'chart.js';
export default defineComponent({
  name: "MetricsViewer",
  components: {
    FullWidthChart,
    MetricCard,
  },
  props: {
    metrics: {
      type: Object,
      required: true,
    },
  },
  setup(props) {
    // Chart Data
    const totalSuggestionsAndAcceptanceChartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: [],
    });

    const chartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: [],
    });



    const chartOptions: ChartOptions<"line"> & LineControllerChartOptions = {
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
            display: false,
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
            display: false,
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
      spanGaps: false,
      showLine: true,
    };

    const data = Array.isArray(props.metrics) ? props.metrics : [];
    const { cumulativeNumberSuggestions, cumulativeNumberAcceptances, cumulativeNumberLOCAccepted, acceptanceRateAverage } = calculateCumulativeMetrics(data);

    const cumulativeSuggestionsData = data.map((m: Metrics) => m.total_suggestions_count);
    const cumulativeAcceptancesData = data.map((m: Metrics) => m.total_acceptances_count);
    const acceptanceRates = data.map((m: Metrics) =>
      m.total_suggestions_count !== 0
        ? (m.total_acceptances_count / m.total_suggestions_count) * 100
        : 0
    );

    totalSuggestionsAndAcceptanceChartData.value = {
      labels: data.map((m: Metrics) => m.day),
      datasets: [
        {
          label: "Total Suggestions",
          data: cumulativeSuggestionsData,
          backgroundColor: 'rgba(75, 192, 192, 0.6)',
          borderColor: 'rgb(75, 192, 192)',
          type: "bar",
        },
        {
          label: "Total Acceptance",
          data: cumulativeAcceptancesData,
          backgroundColor: 'rgba(255, 99, 132, 0.6)',
          borderColor: 'rgb(255, 99, 132)',
          type: "bar",
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

    const cumulativeLOCAcceptedData = data.map((m: Metrics) => m.total_lines_accepted);
    const acceptanceLinesRates = data.map((m: Metrics) =>
      m.total_lines_suggested !== 0
        ? (m.total_lines_accepted / m.total_lines_suggested) * 100
        : 0
    );

    chartData.value = {
      labels: data.map((m: Metrics) => m.day),
      datasets: [
        {
          label: "Total Lines Suggested",
          data: data.map((m: Metrics) => m.total_lines_suggested),
          backgroundColor: 'rgba(75, 192, 192, 0.6)',
          borderColor: 'rgb(75, 192, 192)',
          type: "bar",
        },
        {
          label: "Total Lines Accepted",
          data: cumulativeLOCAcceptedData,
          backgroundColor: 'rgba(255, 99, 132, 0.6)',
          borderColor: 'rgb(255, 99, 132)',
          type: "bar",
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

    return {
      totalSuggestionsAndAcceptanceChartData,
      chartData,
      chartOptions,
      acceptanceRateAverage,
      cumulativeNumberSuggestions,
      cumulativeNumberAcceptances,
      cumulativeNumberLOCAccepted,
    };
  },
});
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
