<template>
  <v-card class="metric-card">
    <v-card-title class="dora-title" style="text-align: left;">
      Cycle Time
      <v-tooltip bottom>
        <template v-slot:activator="{ attrs }">
          <v-icon small class="ml-2" v-bind="attrs">mdi-information</v-icon>
        </template>
        <span>Cycle time measures the duration from PR creation to merge.</span>
      </v-tooltip>
    </v-card-title>
    <v-row>
      <v-col cols="12" md="6">
        <v-row class="cycle-metrics">
          <v-col cols="12" class="cycle-metric-time" style="text-align: center; padding: 20px;">
            <span class="main-time" style="font-size: 2.5em; font-weight: bold;">{{ formattedCycleTime }}</span>
            <span class="time-change positive" style="font-size: 1.5em; color: green; margin-left: 10px;">▼ 70 %</span>
          </v-col>
          <v-col cols="12">
            <v-row justify="space-between" class="metric-breakdown">
              <v-col cols="3" class="text-center">
                <div class="metric-time" style="color: #f44336">{{ phaseTimes.coding }}</div>
                <div class="rectangle" style="background-color: #f44336; height: 8px; width: 100%;"></div>
                <small>Coding</small>
              </v-col>
              <v-col cols="3" class="text-center">
                <div class="metric-time" style="color: #4caf50">{{ phaseTimes.pickup }}</div>
                <div class="rectangle" style="background-color: #4caf50; height: 8px; width: 100%;"></div>
                <small>Pickup</small>
              </v-col>
              <v-col cols="3" class="text-center">
                <div class="metric-time" style="color: #ffeb3b">{{ phaseTimes.review }}</div>
                <div class="rectangle" style="background-color: #ffeb3b; height: 8px; width: 100%;"></div>
                <small>Review</small>
              </v-col>
              <v-col cols="3" class="text-center">
                <div class="metric-time" style="color: #2196f3">{{ phaseTimes.merge }}</div>
                <div class="rectangle" style="background-color: #2196f3; height: 8px; width: 100%;"></div>
                <small>Merge</small>
              </v-col>
            </v-row>
          </v-col>
        </v-row>
      </v-col>
      <v-col cols="12" md="6">
        <div style="width: 100%; position: relative;">
          <Bar :data="cycleTimeData" :options="chartOptions" />
        </div>
      </v-col>
    </v-row>
  </v-card>
</template>

<script lang="ts">
import { defineComponent, computed, ref, PropType, watch } from 'vue';
import { Bar } from 'vue-chartjs';
import GitHubService from '@/services/DoraService';
import config from '@/config';

function calculateTimeDifferenceInDays(startTime: string, endTime: string): number {
  if (!startTime || !endTime) return 0;
  const start = new Date(startTime).getTime();
  const end = new Date(endTime).getTime();
  const diff = end - start;
  if (diff < 0) return 0;
  return (end - start) / (1000 * 60 * 60 * 24); // Convert milliseconds to days
}

function formatTime(days: number): string {
  const d = Math.floor(days);
  const h = Math.round((days - d) * 24);
  return `${d}d ${h}h`;
}

export default defineComponent({
  name: 'CycleTimeCard',
  components: { Bar },
  props: {
    pullRequests: {
      type: Array as PropType<any[]>,
      required: true,
    },
    selectedRepo: {
      type: String,
      required: true,
    },
  },
  setup(props) {
    const phaseTimes = ref({
      coding: '0d 0h',
      pickup: '0d 0h',
      review: '0d 0h',
      merge: '0d 0h',
    });

    const formattedCycleTime = computed(() => {
      const cycleTimes = props.pullRequests.map(pr => {
        const cycleTime = calculateTimeDifferenceInDays(pr.created_at, pr.merged_at);
        return cycleTime;
      });

      const averageCycleTime = cycleTimes.length > 0 ? (cycleTimes.reduce((a, b) => a + b, 0) / cycleTimes.length) : 0;
      return formatTime(averageCycleTime);
    });

    const updatePhaseTimes = async () => {
      const codingTimes = [];
      const pickupTimes = [];
      const reviewTimes = [];
      const mergeTimes = [];
      const owner = config.github.org;
      const repo = props.selectedRepo;

      for (const pr of props.pullRequests) {
        const reviews = await GitHubService.getTimePullRequestReviews(owner, repo, pr.number);
        if (reviews) {
          codingTimes.push(calculateTimeDifferenceInDays(pr.created_at, reviews.first_review_at));
          pickupTimes.push(calculateTimeDifferenceInDays(reviews.first_review_at, reviews.review_completed_at));
          reviewTimes.push(calculateTimeDifferenceInDays(reviews.review_completed_at, reviews.approved_at));
          mergeTimes.push(calculateTimeDifferenceInDays(reviews.approved_at, pr.merged_at));
          // Investigate review time and merge time
          console.log(`${reviews.review_completed_at} - ${pr.approved_at} = ${calculateTimeDifferenceInDays(reviews.review_completed_at, reviews.approved_at)} `);
        }
      }

      const averageCodingTime = codingTimes.length > 0 ? (codingTimes.reduce((a, b) => a + b, 0) / codingTimes.length) : 0;
      const averagePickupTime = pickupTimes.length > 0 ? (pickupTimes.reduce((a, b) => a + b, 0) / pickupTimes.length) : 0;
      const averageReviewTime = reviewTimes.length > 0 ? (reviewTimes.reduce((a, b) => a + b, 0) / reviewTimes.length) : 0;
      const averageMergeTime = mergeTimes.length > 0 ? (mergeTimes.reduce((a, b) => a + b, 0) / mergeTimes.length) : 0;

      phaseTimes.value = {
        coding: formatTime(averageCodingTime),
        pickup: formatTime(averagePickupTime),
        review: formatTime(averageReviewTime),
        merge: formatTime(averageMergeTime),
      };
    };

    const groupByDate = (pullRequests: any[]) => {
      const grouped: { [key: string]: number[] } = {};
      pullRequests.forEach(pr => {
        const date = pr.created_at.split('T')[0];
        const cycleTime = calculateTimeDifferenceInDays(pr.created_at, pr.merged_at);
        if (!grouped[date]) {
          grouped[date] = [];
        }
        grouped[date].push(cycleTime);
      });
      return grouped;
    };

    const groupedData = computed(() => {
      const grouped = groupByDate(props.pullRequests);
      const labels = Object.keys(grouped).reverse();
      const data = labels.map(date => {
        const cycleTimes = grouped[date];
        const averageCycleTime = cycleTimes.reduce((a, b) => a + b, 0) / cycleTimes.length;
        return averageCycleTime;
      });
      return { labels, data };
    });

    const cycleTimeData = ref({
      labels: groupedData.value.labels,
      datasets: [
        {
          label: 'Cycle Time (days)',
          data: groupedData.value.data,
          backgroundColor: 'rgba(75, 192, 192, 0.2)',
          borderColor: 'rgba(75, 192, 192, 1)',
          borderWidth: 1,
        },
      ],
    });

    const chartOptions = {
      plugins: {
        legend: { display: false },
        title: { display: false },
      },
      scales: {
        x: { title: { display: false } },
        y: { beginAtZero: true, title: { display: false } },
      },
    };


    // Watch for changes in pullRequests prop and update phase times accordingly
    watch(() => props.pullRequests, () => {
      updatePhaseTimes();
      cycleTimeData.value = {
        labels: groupedData.value.labels,
        datasets: [
          {
            label: 'Cycle Time (days)',
            data: groupedData.value.data,
            backgroundColor: 'rgba(75, 192, 192, 0.2)',
            borderColor: 'rgba(75, 192, 192, 1)',
            borderWidth: 1,
          },
        ],
      };
    }, { deep: true });

    return {
      formattedCycleTime,
      phaseTimes,
      cycleTimeData,
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