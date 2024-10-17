<template>
  <div>
    <!-- Dashboard Container for Metrics -->
    <v-container fluid class="dashboard-container">
      <v-row justify="center" align="center">
        <!-- Breakdown Count Card -->
        <MetricCard :title="`Number of ${breakdownDisplayNamePlural}`" subtitle="Over the last 28 days"
          :value="numberOfBreakdowns" />
      </v-row>
    </v-container>

    <!-- Combined Charts Section -->
    <v-container fluid class="charts-container">
      <v-row>
        <!-- Top 5 by Accepted Prompts Chart -->
        <ChartCard :title="'Top 5 ' + breakdownDisplayNamePlural + ' by accepted prompts'"
          :data="breakdownsChartDataTop5AcceptedPrompts" :options="chartOptions" :sm="12" :md="6" :chart-type="'pie'" />

        <!-- Top 5 by Acceptance Rate Chart -->
        <ChartCard :title="'Top 5 ' + breakdownDisplayNamePlural + ' by acceptance rate'"
          :data="breakdownsChartDataTop5AcceptanceRate" :options="chartOptions" :sm="12" :md="6" :chart-type="'pie'" />
      </v-row>

      <!-- Breakdown Table -->
      <v-row>
        <v-col cols="12">
          <v-card class="chart-card">
            <v-card-title class="text-center">
              {{ breakdownDisplayNamePlural }} Breakdown
            </v-card-title>
            <v-data-table :headers="headers" :items="Array.from(breakdowns)" class="elevation-2 table-container"
              :items-per-page="10" :footer-props="{
                'items-per-page-options': [5, 10, 20], // Add page size options
              }" :sort-by="['acceptedPrompts']" :sort-desc="[true]">
              <template v-slot:item="{ item }">
                <tr>
                  <td>{{ item[0] }}</td>
                  <td class="text-right">{{ item[1].acceptedPrompts }}</td>
                  <td class="text-right">{{ item[1].acceptedLinesOfCode }}</td>
                  <td class="text-right" v-if="item[1].acceptanceRate !== undefined">
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

<script lang="ts">
import { defineComponent, ref, toRef } from "vue";
import { Metrics } from "../model/Metrics";
import { Breakdown } from "../model/Breakdown";
import ChartCard from "./Commons/ChartCard.vue";
import MetricCard from "./Commons/MetricCard.vue";
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
    ChartCard,
    MetricCard

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
        { title: "Accepted Prompts", key: "acceptedPrompts", align: "end" },
        { title: "Accepted Lines of Code", key: "acceptedLinesOfCode", align: "end" },
        { title: "Acceptance Rate (%)", key: "acceptanceRate", align: "end" },
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

    // //Sort breakdowns map by acceptance rate
    // const sortedBreakdownsByAcceptanceRate = new Map(
    //   Array.from(breakdowns.value.entries()).sort(
    //     (a, b) => b[1].acceptanceRate - a[1].acceptanceRate
    //   )
    // );

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

    // //Sort breakdowns map by accepted prompts
    // const sortedBreakdownsByAcceptedPrompts = new Map(
    //   Array.from(breakdowns.value.entries()).sort(
    //     (a, b) => b[1].acceptedPrompts - a[1].acceptedPrompts
    //   )
    // );

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

<style scoped>
.table-container {
  max-height: 650px;
  overflow-y: auto;
}

.v-data-table {
  font-size: 0.9rem;
  border-radius: 12px;
}

.v-data-table :deep(th) {
  font-weight: bold;
  background-color: #f5f5f5;
  position: sticky;
  top: 0;
  z-index: 2;
  white-space: nowrap;
  font-size: 1.2rem;
  padding: 12px;
}


.v-data-table td {
  padding: 10px;
}

.v-data-table tr {
  transition: background-color 0.3s ease;
}

.v-data-table tr:hover {
  background-color: #f0f8ff;
}

.text-right {
  text-align: right;
}

.elevation-2 {
  .elevation-2 {
    box-shadow: 0px 1px 5px rgba(0, 0, 0, 0.1);
  }
}

.v-data-footer {
  padding-top: 10px;
}
</style>