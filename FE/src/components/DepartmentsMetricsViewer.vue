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
    </v-container>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, toRef } from "vue";
import { TeamMetrics } from "../model/Metrics";
import { Line } from "vue-chartjs";
import { ChartOptions } from "chart.js";
import { calculateTeamAcceptanceRate, calculateTeamCumulativeMetrics } from "@/utils/MetricUtils";
import MetricCard from "./Commons/MetricCard.vue";
export default defineComponent({
  name: "DepartmentsMetricsViewer",
  props: {
    teamMetrics: {
      type: Array as () => TeamMetrics[],
      required: true,
    },
  },
  components: {
    Line,
    MetricCard,
  },
  setup(props) {

    // Chart Data
    const teamAcceptanceRateChartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: [],
    });


    const teamAcceptanceRateChartOptions: ChartOptions<"bar" | "line"> = {
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

    const teamMetricsData = toRef(props, "teamMetrics").value.filter((team) => team.team_tag !== "All Teams");

    // Calculate cumulative metrics
    const { cumulativeNumberAcceptances, cumulativeNumberLOCAccepted, acceptanceRateAverage, cumulativeNumberSuggestions } = calculateTeamCumulativeMetrics(teamMetricsData);

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
      // chartOptions,
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