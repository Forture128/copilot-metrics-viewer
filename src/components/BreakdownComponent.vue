<template>
  <div>
    <!-- Dashboard Container for Metrics -->
    <v-container fluid class="dashboard-container">
      <v-row justify="center" align="center">
        <!-- Breakdown Count Card -->
        <v-col cols="12" sm="6" md="3">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">
                  Number of {{ breakdownDisplayNamePlural }}
                </div>
                <div class="metric-subtitle">Over the last 28 days</div>
                <p class="metric-value">{{ numberOfBreakdowns }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>
    </v-container>

    <!-- Combined Charts Section -->
    <v-container fluid class="charts-container">
      <v-row>
        <!-- Top 5 by Accepted Prompts Chart -->
        <v-col cols="12" md="6">
          <v-card class="chart-card">
            <v-card-item class="d-flex justify-center align-center">
              <div class="text-h6 mb-1">
                Top 5 {{ breakdownDisplayNamePlural }} by accepted prompts
              </div>
              <div class="chart-container">
                <Pie :data="breakdownsChartDataTop5AcceptedPrompts" :options="chartOptions" />
              </div>
            </v-card-item>
          </v-card>
        </v-col>

        <!-- Top 5 by Acceptance Rate Chart -->
        <v-col cols="12" md="6">
          <v-card class="chart-card">
            <v-card-item class="d-flex justify-center align-center">
              <div class="text-h6 mb-1">
                Top 5 {{ breakdownDisplayNamePlural }} by acceptance rate
              </div>
              <div class="chart-container">
                <Pie :data="breakdownsChartDataTop5AcceptanceRate" :options="chartOptions" />
              </div>
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>

      <!-- <v-row>
        <v-col cols="12">
          <h3 class="text-center">
            Total Suggestions Count | Total Acceptances Count | Acceptance Rate
            (%)
          </h3>
          <v-card class="chart-card">
            <PolarArea
              :data="totalBreakdownsChartData"
              :options="chartOptions"
            />
          </v-card>
        </v-col>
      </v-row> -->

      <!-- Breakdown Table -->
      <v-row>
        <v-col cols="12">
          <h2 class="text-center">
            {{ breakdownDisplayNamePlural }} Breakdown
          </h2>
          <v-card class="chart-card">
            <v-data-table :headers="headers" :items="Array.from(breakdowns)" class="elevation-2 table-container">
              <template v-slot:item="{ item }">
                <tr>
                  <td>{{ item[0] }}</td>
                  <td>{{ item[1].acceptedPrompts }}</td>
                  <td>{{ item[1].acceptedLinesOfCode }}</td>
                  <td v-if="item[1].acceptanceRate !== undefined">
                    {{ item[1].acceptanceRate.toFixed(2) }}%
                  </td>
                </tr>
              </template>
            </v-data-table>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>
>

<script lang="ts">
import { defineComponent, ref, toRef } from "vue";
import { Metrics } from "../model/Metrics";
import { Breakdown } from "../model/Breakdown";
import { Pie } from "vue-chartjs";

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
  RadialLinearScale,
} from "chart.js";

ChartJS.register(
  ArcElement,
  CategoryScale,
  LinearScale,
  BarElement,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  RadialLinearScale // This is the missing scale
);

export default defineComponent({
  name: "BreakdownComponent",
  props: {
    metrics: {
      type: Object,
      required: true,
    },
    breakdownKey: {
      type: String,
      required: true,
    },
  },
  components: {
    Pie,
  },
  computed: {
    breakdownDisplayName() {
      return (
        this.breakdownKey.charAt(0).toUpperCase() + this.breakdownKey.slice(1)
      );
    },
    breakdownDisplayNamePlural() {
      return `${this.breakdownDisplayName}s`;
    },
    headers() {
      return [
        { title: `${this.breakdownDisplayName} Name`, key: "breakdownName" },
        { title: "Accepted Prompts", key: "acceptedPrompts" },
        { title: "Accepted Lines of Code", key: "acceptedLinesOfCode" },
        { title: "Acceptance Rate (%)", key: "acceptanceRate" },
      ];
    },
  },
  setup(props) {
    // Create an empty map to store the breakdowns.
    const breakdowns = ref(new Map<string, Breakdown>());

    // Number of breakdowns
    const numberOfBreakdowns = ref(0);

    // Breakdowns Chart Data for breakdowns breakdown Pie Chart
    const breakdownsChartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: [],
    });


    //Top 5 by accepted prompts
    const breakdownsChartDataTop5AcceptedPrompts = ref<{
      labels: string[];
      datasets: any[];
    }>({ labels: [], datasets: [] });

    //Top 5 by acceptance rate
    const breakdownsChartDataTop5AcceptanceRate = ref<{
      labels: string[];
      datasets: any[];
    }>({ labels: [], datasets: [] });

    const chartOptions = {
      responsive: true,
      maintainAspectRatio: true,
    };

    const pieChartColors = ref([
      "#4B0082",
      "#41B883",
      "#483D8B",
      "#87CEFA",
      "#32CD32",
    ]);

    const data = toRef(props, "metrics").value;

    // Process the breakdown separately
    data.forEach((m: Metrics) =>
      m.breakdown.forEach((breakdownData) => {
        const breakdownName = breakdownData[
          props.breakdownKey as keyof typeof breakdownData
        ] as string;
        let breakdown = breakdowns.value.get(breakdownName);

        if (!breakdown) {
          // Create a new breakdown object if it does not exist
          breakdown = new Breakdown({
            name: breakdownName,
            acceptedPrompts: breakdownData.acceptances_count,
            suggestedLinesOfCode: breakdownData.lines_suggested,
            acceptedLinesOfCode: breakdownData.lines_accepted,
          });
          breakdowns.value.set(breakdownName, breakdown);
        } else {
          // Update the existing breakdown object
          breakdown.acceptedPrompts += breakdownData.acceptances_count;
          breakdown.suggestedLinesOfCode += breakdownData.lines_suggested;
          breakdown.acceptedLinesOfCode += breakdownData.lines_accepted;
        }
        // Recalculate the acceptance rate
        breakdown.acceptanceRate =
          breakdown.suggestedLinesOfCode !== 0
            ? (breakdown.acceptedLinesOfCode / breakdown.suggestedLinesOfCode) *
            100
            : 0;
      })
    );

    //Sort breakdowns map by acceptance rate
    breakdowns.value[Symbol.iterator] = function* () {
      yield* [...this.entries()].sort(
        (a, b) => b[1].acceptanceRate - a[1].acceptanceRate
      );
    };

    // Get the top 5 breakdowns by acceptance rate
    const top5BreakdownsAcceptanceRate = new Map(
      [...breakdowns.value].slice(0, 5)
    );

    breakdownsChartDataTop5AcceptanceRate.value = {
      labels: Array.from(top5BreakdownsAcceptanceRate.values()).map(
        (breakdown) => breakdown.name
      ),
      datasets: [
        {
          data: Array.from(top5BreakdownsAcceptanceRate.values()).map(
            (breakdown) => breakdown.acceptanceRate.toFixed(2)
          ),
          backgroundColor: pieChartColors.value,
        },
      ],
    };

    //Sort breakdowns map by accepted prompts
    breakdowns.value[Symbol.iterator] = function* () {
      yield* [...this.entries()].sort(
        (a, b) => b[1].acceptedPrompts - a[1].acceptedPrompts
      );
    };

    breakdownsChartData.value = {
      labels: Array.from(breakdowns.value.values()).map(
        (breakdown) => breakdown.name
      ),
      datasets: [
        {
          data: Array.from(breakdowns.value.values()).map(
            (breakdown) => breakdown.acceptedPrompts
          ),
          backgroundColor: pieChartColors.value,
        },
      ],
    };

    // Get the top 5 breakdowns by accepted prompts
    const top5BreakdownsAcceptedPrompts = new Map(
      [...breakdowns.value].slice(0, 5)
    );

    breakdownsChartDataTop5AcceptedPrompts.value = {
      labels: Array.from(top5BreakdownsAcceptedPrompts.values()).map(
        (breakdown) => breakdown.name
      ),
      datasets: [
        {
          data: Array.from(top5BreakdownsAcceptedPrompts.values()).map(
            (breakdown) => breakdown.acceptedPrompts
          ),
          backgroundColor: pieChartColors.value,
        },
      ],
    };
    // const top50BreakdownsAcceptedPrompts = new Map(
    //   [...breakdowns.value].slice(0, 50)
    // );
    // // Add Polar Area Chart for Total Breakdowns
    // totalBreakdownsChartData.value = {
    //   labels: Array.from(top50BreakdownsAcceptedPrompts.values()).map(
    //     (breakdown) => breakdown.name
    //   ),
    //   datasets: [
    //     {
    //       data: Array.from(top50BreakdownsAcceptedPrompts.values()).map(
    //         (breakdown) => breakdown.acceptedPrompts
    //       ),
    //     },
    //   ],
    // };

    numberOfBreakdowns.value = breakdowns.value.size;

    return {
      chartOptions,
      breakdowns,
      numberOfBreakdowns,
      breakdownsChartData,
      // totalBreakdownsChartData,
      breakdownsChartDataTop5AcceptedPrompts,
      breakdownsChartDataTop5AcceptanceRate,
    };
  },
});
</script>
