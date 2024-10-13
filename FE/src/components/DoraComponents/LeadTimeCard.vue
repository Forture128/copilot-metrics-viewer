<template>
  <v-card class="metric-card">
    <v-card-title class="dora-title" style="text-align: left;">
      Lead Time for Changes
      <v-tooltip bottom>
        <template v-slot:activator="{ attrs }">
          <v-icon small class="ml-2" v-bind="attrs">mdi-information</v-icon>
        </template>
        <span>Lead time for changes measures the time it takes from committing code to deploying it to
          production.</span>
      </v-tooltip>
    </v-card-title>
    <v-row>
      <!-- Metric Column -->
      <v-col cols="12" md="6">
        <v-row class="cycle-metrics">
          <v-col cols="12" class="cycle-metric-time" style="text-align: center; padding: 20px;">
            <span class="main-time" style="font-size: 2.0em; font-weight: bold;">{{ formattedLeadTime }}</span>
            <span class="time-change positive" style="font-size: 1.5em; color: green; margin-left: 10px;">
              ▼ 15 %
            </span>
          </v-col>
          <v-col cols="12">
            <v-row justify="space-between" class="metric-breakdown">
              <v-col cols="12" class="text-center">
                <small>Average Lead Time: <b>{{ formattedLeadTime }}</b></small>
              </v-col>
            </v-row>
          </v-col>
        </v-row>
      </v-col>

      <!-- Line Chart Column -->
      <v-col cols="12" md="6">
        <div style="width: 100%; position: relative;">
          <Line :data="leadTimeChartData" :options="chartOptions" />
        </div>
      </v-col>
    </v-row>
  </v-card>
</template>

<script lang="ts">
import { defineComponent, computed, ref, PropType, watch } from 'vue';
import { Line } from 'vue-chartjs';
import { Chart as ChartJS, LineElement, CategoryScale, LinearScale, Title, Tooltip, Legend } from 'chart.js';

// Register necessary components for Chart.js
ChartJS.register(LineElement, CategoryScale, LinearScale, Title, Tooltip, Legend);

// Utility function to format time in days and hours
function formatTime(days: number): string {
  const d = Math.floor(days);
  const h = Math.round((days - d) * 24);
  return `${d}d ${h}h`;
}

export default defineComponent({
  name: 'LeadTimeCard',
  components: { Line },
  props: {
    pullRequests: {
      type: Array as PropType<any[]>,
      required: true,
    },
  },
  setup(props) {
    // Function to calculate the lead time between PR creation and merge
    const calculateLeadTime = (created_at: string, merged_at: string) => {
      const createdDate = new Date(created_at).getTime();
      const mergedDate = new Date(merged_at).getTime();
      return (mergedDate - createdDate) / (1000 * 60 * 60 * 24); // Convert milliseconds to days
    };

    // Computed property for formatted lead time (average)
    const formattedLeadTime = computed(() => {
      const totalLeadTime = props.pullRequests.reduce((sum, pr) => {
        const leadTime = calculateLeadTime(pr.created_at, pr.merged_at);
        return sum + leadTime;
      }, 0);
      const averageLeadTime = props.pullRequests.length > 0
        ? (totalLeadTime / props.pullRequests.length).toFixed(2)
        : 0;
      return formatTime(Number(averageLeadTime));
    });

    // Reactive data for the lead time chart
    const leadTimeChartData = ref({
      labels: props.pullRequests.map(pr => pr.created_at.split('T')[0]),
      datasets: [
        {
          label: 'Lead Time (days)',
          data: props.pullRequests.map(pr => calculateLeadTime(pr.created_at, pr.merged_at)),
          borderColor: 'rgba(54, 162, 235, 1)',
          backgroundColor: 'rgba(54, 162, 235, 0.2)',
        },
      ],
    });

    // Chart options for better visualization
    const chartOptions = {
      responsive: true,
      plugins: {
        legend: { display: false },
        title: { display: false },
      },
      scales: {
        x: { title: { display: false } },
        y: { beginAtZero: true, title: { display: false } },
      },
    };

    // Watch for changes in pullRequests prop and update the chart data
    watch(() => props.pullRequests, (newPullRequests) => {
      leadTimeChartData.value = {
        labels: newPullRequests.map(pr => pr.created_at.split('T')[0]),
        datasets: [
          {
            label: 'Lead Time (days)',
            data: newPullRequests.map(pr => calculateLeadTime(pr.created_at, pr.merged_at)),
            borderColor: 'rgba(54, 162, 235, 1)',
            backgroundColor: 'rgba(54, 162, 235, 0.2)',
          },
        ],
      };
    }, { deep: true });

    return {
      formattedLeadTime,
      leadTimeChartData,
      chartOptions,
    };
  },
});
</script>

<style scoped>
.metric-card {
  padding: 10px;
  text-align: center;
}
</style>