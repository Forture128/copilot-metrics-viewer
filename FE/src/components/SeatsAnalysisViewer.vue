<template>
  <div>
    <!-- Dashboard Container for Metrics -->
    <v-container fluid class="dashboard-container">
      <v-row justify="center" align="center">
        <!-- Total Assigned Card -->
        <v-col cols="12" sm="6" md="4">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Total Assigned</div>
                <div class="metric-subtitle">Currently assigned seats</div>
                <p class="metric-value">{{ totalSeats.length }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>

        <!-- Assigned But Never Used Card -->
        <v-col cols="12" sm="6" md="4">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Assigned But Never Used</div>
                <div class="metric-subtitle">No show seats</div>
                <p class="metric-value">{{ NoshowSeats.length }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>

        <!-- No Activity in the Last 7 days Card -->
        <v-col cols="12" sm="6" md="4">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">No Activity in the Last 7 Days</div>
                <div class="metric-subtitle">No use in the last 7 days</div>
                <p class="metric-value">{{ unusedSeats.length }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>

        <!-- Average Time to First Activity Card -->
        <v-col cols="12" sm="6" md="4">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Avg Time to First Activity</div>
                <div class="metric-subtitle">For seats with activity</div>
                <p class="metric-value">{{ averageTimeToFirstActivity }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>
    </v-container>


    <!-- Activity Recency Distribution and Average Time -->
    <v-container fluid class="line-chart-container">
      <v-row>
        <v-col cols="12" md="6">
          <v-card class="chart-card">
            <v-card-item>
              <Bar :data="recencyChartData" :options="recencyChartOptions" />
            </v-card-item>
          </v-card>
        </v-col>

        <v-col cols="12" md="6">
          <v-card class="chart-card">
            <v-card-item>
              <Bar :data="recencyChartInactiveData" :options="recencyChartInactiveOptions" />
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>
    </v-container>

    <!-- Line Chart for Seat Usage -->
    <v-container fluid class="line-chart-container">
      <v-row justify="center">
        <v-col cols="12" md="8">
          <Line :data="chartData" :options="chartOptions" />
        </v-col>
      </v-row>
    </v-container>
    <!-- Filter Section -->
    <!-- Filter Section -->
    <v-container fluid>
      <v-row>
        <v-col cols="12" md="4">
          <v-select v-model="selectedTeam" :items="teams" label="Filter by Team" />
        </v-col>
        <v-col cols="12" md="4">
          <v-select v-model="activityStatus" :items="activityStatusOptions" label="Filter by Activity Status" />
        </v-col>
        <v-col cols="12" md="4" v-if="activityStatus === 'Active' || activityStatus === 'Inactive'">
          <v-select v-model="recencyFilter" :items="recencyFilterOptions" label="Filter by Recency" />
        </v-col>
      </v-row>
    </v-container>
    <!-- Detailed User Table -->
    <v-container fluid class="seats-table-container">
      <v-row>
        <v-col cols="12">
          <h2 class="text-center">All Assigned Seats</h2>
          <v-card class="table-card">
            <v-data-table :headers="headers" :items="filteredSeats" :items-per-page="10" class="elevation-2">
              <template v-slot:item="{ item }">
                <tr>
                  <td>{{ item.login }}</td>
                  <td>{{ item.id }}</td>
                  <td>{{ item.team }}</td>
                  <td>{{ item.created_at }}</td>
                  <td>{{ item.last_activity_at }}</td>
                  <td>{{ item.timeToFirstActivity ? item.timeToFirstActivity + ' days' : 'N/A' }}</td>
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
import { computed, defineComponent, ref, watchEffect } from 'vue';
import { Bar, Line } from 'vue-chartjs';
import { Seat } from '../model/Seat';
import 'chartjs-adapter-date-fns'; // Required for date/time formatting

import {
  Chart as ChartJS,
  ArcElement,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
  TimeScale,
  TimeSeriesScale
} from 'chart.js';

ChartJS.register(
  ArcElement,
  LinearScale,
  BarElement,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  TimeScale,
  TimeSeriesScale
);

export default defineComponent({
  name: 'SeatsAnalysisViewer',
  components: {
    Line,
    Bar
  },
  props: {
    seats: {
      type: Array as () => Seat[],
      required: true,
      default: () => []
    }
  },
  setup(props) {
    const totalSeats = ref<Seat[]>([]);
    const NoshowSeats = ref<Seat[]>([]);
    const unusedSeats = ref<Seat[]>([]);
    const activeSeats = ref<Record<string, Seat[]>>({
      "> 30 Days": [],
      "Last 30 Days": [],
      "Last 7 Days": [],
      "Today": []
    });
    const inactiveSeats = ref<Record<string, Seat[]>>({
      "> 30 Days": [],
      "Last 30 Days": [],
      "Last 7 Days": [],
      "Today": []
    });
    const activityStatusOptions = ['All', "Unused", 'Active', 'Inactive'];
    const recencyFilterOptions = ['> 30 Days', 'Last 30 Days', 'Last 7 Days', 'Today'];
    const teams = ['MFW-Github-Copilot'];

    const activityStatus = ref('All');
    const recencyFilter = ref(null);
    const selectedTeam = ref(teams[0]); // Default selected team. # FIXME: Update this based on the actual team data.


    const chartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: []
    });

    const chartOptions = {
      responsive: true,
      plugins: {
        legend: {
          position: 'top' as const,
        },
        title: {
          display: true,
          text: 'Seats Usage Over Time',
        },
      },
      scales: {
        x: {
          time: {
            unit: 'day',
            tooltipFormat: 'MMM dd, yyyy',
            displayFormats: {
              day: 'MMM d',
            },
          },
          title: {
            display: true,
            text: 'Date',
          },
        },
        y: {
          beginAtZero: true,
          title: {
            display: true,
            text: 'Seats Count',
          },
        },
      },
    };

    const headers = [
      { title: 'Login', key: 'login' },
      { title: 'GitHub ID', key: 'id' },
      { title: 'Assigning team', key: 'team' },
      { title: 'Assigned time', key: 'created_at' },
      { title: 'Last Activity At', key: 'last_activity_at' },
      { title: 'Last Activity Editor', key: 'last_activity_editor' }
    ];



    const updateChartData = () => {
      const seatCountsByDate: { [key: string]: number } = {};
      const unusedSeatCountsByDate: { [key: string]: number } = {};

      const allDates = totalSeats.value.map(seat => new Date(seat.created_at));
      const minDate = new Date(Math.min(...allDates.map(date => date.getTime())));
      const today = new Date();

      // Calculate seat counts for each day from minDate to today
      for (let d = new Date(minDate); d <= today; d.setDate(d.getDate() + 1)) {
        const currentDate = new Date(d).toISOString().split('T')[0]; // Format date as YYYY-MM-DD
        seatCountsByDate[currentDate] = 0;
        unusedSeatCountsByDate[currentDate] = 0;

        totalSeats.value.forEach(seat => {
          const createdDate = new Date(seat.created_at).toISOString().split('T')[0];
          const lastActivityDate = seat.last_activity_at
            ? new Date(seat.last_activity_at).toISOString().split('T')[0]
            : null;

          // Count total seats created before or on the current date
          if (createdDate <= currentDate) {
            seatCountsByDate[currentDate]++;

            // Count unused seats if inactive for more than 7 days or never used
            if (!lastActivityDate || (new Date(lastActivityDate) < new Date(currentDate) && (new Date(currentDate).getTime() - new Date(lastActivityDate).getTime()) > (7 * 24 * 60 * 60 * 1000))) {
              unusedSeatCountsByDate[currentDate]++;
            }
          }
        });
      }

      const labels = Object.keys(seatCountsByDate);
      const totalSeatData = labels.map(date => seatCountsByDate[date]);
      const unusedSeatData = labels.map(date => unusedSeatCountsByDate[date]);

      // Ensure that Unused Seats stacks on top of Total Seats
      chartData.value = {
        labels,
        datasets: [
          {
            label: 'Total Seats',
            data: totalSeatData,
            backgroundColor: 'rgba(75, 192, 192, 0.6)',  // Semi-transparent fill color for area chart
            borderColor: 'rgb(75, 192, 192)',
            fill: true,  // Enable fill for area chart
          },
          {
            label: 'Unused Seats',
            data: unusedSeatData,
            backgroundColor: 'rgba(192, 75, 192, 0.6)',  // Semi-transparent fill color for unused seats
            borderColor: 'rgb(192, 75, 192)',
            fill: true,  // Enable fill for area chart
          }
        ]
      };
    };

    // START Activity Recency Distribution
    const recencyChartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: []
    });
    const recencyChartInactiveData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: []
    });

    const recencyChartOptions = {
      responsive: true,
      plugins: {
        legend: {
          display: false
        },
        title: {
          display: true,
          text: 'Seats Activity Recency'
        }
      },
      scales: {
        x: {
          title: {
            display: true,
            text: 'Recency'
          }
        },
        y: {
          beginAtZero: true,
          title: {
            display: true,
            text: 'Number of Seats'
          }
        }
      }
    };
    const recencyChartInactiveOptions = {
      responsive: true,
      plugins: {
        legend: {
          display: false
        },
        title: {
          display: true,
          text: 'Seats Inactivity Recency'
        }
      },
      scales: {
        x: {
          title: {
            display: true,
            text: 'Recency'
          }
        },
        y: {
          beginAtZero: true,
          title: {
            display: true,
            text: 'Number of Seats'
          }
        }
      }
    };

    const updateRecencyChartData = () => {
      const today = new Date();
      const activeCounts = { "> 30 Days": 0, "Last 30 Days": 0, "Last 7 Days": 0, "Today": 0 };

      activeSeats.value = { "> 30 Days": [], "Last 30 Days": [], "Last 7 Days": [], "Today": [] };
      inactiveSeats.value = { "> 30 Days": [], "Last 30 Days": [], "Last 7 Days": [], "Today": [] };

      totalSeats.value.forEach(seat => {
        if (!seat.last_activity_at) {
          activeSeats.value["> 30 Days"].push(seat);
          return;
        }

        const lastActivity = new Date(seat.last_activity_at);
        const diffDays = Math.floor((today.getTime() - lastActivity.getTime()) / (1000 * 60 * 60 * 24));

        if (diffDays === 0) {
          activeCounts["Today"]++;
          activeSeats.value["Today"].push(seat);
        } else if (diffDays <= 7) {
          activeCounts["Last 7 Days"]++;
          activeSeats.value["Last 7 Days"].push(seat);
        } else if (diffDays <= 30) {
          activeCounts["Last 30 Days"]++;
          activeSeats.value["Last 30 Days"].push(seat);
        } else {
          activeCounts["> 30 Days"]++;
          activeSeats.value["> 30 Days"].push(seat);
        }
      });

      const totalSeatsCount = totalSeats.value.length;
      const inactiveCounts = {
        "Today": totalSeatsCount - activeCounts["Today"],
        "Last 7 Days": totalSeatsCount - activeCounts["Last 7 Days"],
        "Last 30 Days": totalSeatsCount - activeCounts["Last 30 Days"],
        "> 30 Days": totalSeatsCount - activeCounts["> 30 Days"]
      };

      inactiveSeats.value["Today"] = totalSeats.value.filter(seat => !activeSeats.value["Today"].includes(seat));
      inactiveSeats.value["Last 7 Days"] = totalSeats.value.filter(seat => !activeSeats.value["Last 7 Days"].includes(seat));
      inactiveSeats.value["Last 30 Days"] = totalSeats.value.filter(seat => !activeSeats.value["Last 30 Days"].includes(seat));
      inactiveSeats.value["> 30 Days"] = totalSeats.value.filter(seat => !activeSeats.value["> 30 Days"].includes(seat));

      recencyChartData.value = {
        labels: ["> 30 Days", "Last 30 Days", "Last 7 Days", "Today"],
        datasets: [
          {
            label: 'Active Seats',
            data: [activeCounts["> 30 Days"], activeCounts["Last 30 Days"], activeCounts["Last 7 Days"], activeCounts["Today"]],
            backgroundColor: ['#1E90FF', '#4682B4', '#5F9EA0', '#87CEEB'],
            borderColor: ['#1E90FF', '#4682B4', '#5F9EA0', '#87CEEB'],
            borderWidth: 1
          }
        ]
      };

      recencyChartInactiveData.value = {
        labels: ["> 30 Days", "Last 30 Days", "Last 7 Days", "Today"],
        datasets: [
          {
            label: 'Inactive Seats',
            data: [inactiveCounts["> 30 Days"], inactiveCounts["Last 30 Days"], inactiveCounts["Last 7 Days"], inactiveCounts["Today"]],
            backgroundColor: ['#FF4500', '#FF6347', '#FF8C00', '#FFD700'],
            borderColor: ['#FF4500', '#FF6347', '#FF8C00', '#FFD700'],
            borderWidth: 1
          }
        ]
      };

      console.log(activeSeats.value); // Log the activeSeats object for debugging
      console.log(inactiveSeats.value); // Log the inactiveSeats object for debugging
    };
    // END Activity Recency Distribution

    // START Average Time to First Activity.
    const averageTimeToFirstActivity = ref<string | null>(null);


    const averageTimeChartOptions = {
      responsive: true,
      plugins: {
        legend: {
          position: 'top' as const,
        },
        title: {
          display: true,
          text: 'Average Time to First Activity (Bar Chart)',
        },
        tooltip: {
          callbacks: {
            // Customize the tooltip label
            label: function (tooltipItem: any) {
              const user_id = tooltipItem.label;
              const daysToFirstActivity = tooltipItem.raw;
              return [
                `User Assignment: ${user_id}`,
                `Days to First Activity: ${daysToFirstActivity.toFixed(2)} days`,
              ];
            }
          }
        }
      },
      scales: {
        x: {
          title: {
            display: true,
            text: 'Teams',  // Display teams on the x-axis
          },
        },
        y: {
          beginAtZero: true,
          title: {
            display: true,
            text: 'Days to First Activity',
          },
        },
      }
    };

    const averageTimeChartData = ref<{ labels: string[]; datasets: any[] }>({
      labels: [],
      datasets: [],
    });


    const updateAverageTimeChart = () => {
      const timeToFirstActivityData: number[] = [];

      totalSeats.value.forEach(seat => {
        const createdAt = new Date(seat.created_at);
        const lastActivityAt = seat.last_activity_at ? new Date(seat.last_activity_at) : null;

        if (lastActivityAt && !isNaN(createdAt.getTime()) && !isNaN(lastActivityAt.getTime())) {
          const timeDifference = (lastActivityAt.getTime() - createdAt.getTime()) / (1000 * 60 * 60 * 24);
          if (timeDifference > 0) {
            timeToFirstActivityData.push(timeDifference);
          }
        }
      });
      const avgTimeToActivity = timeToFirstActivityData.reduce((a, b) => a + b, 0) / timeToFirstActivityData.length;
      averageTimeToFirstActivity.value = avgTimeToActivity.toFixed(2);
    };


    watchEffect(() => {
      if (props.seats && Array.isArray(props.seats)) {
        totalSeats.value = props.seats;

        const oneWeekAgo = new Date();
        oneWeekAgo.setDate(oneWeekAgo.getDate() - 7);

        NoshowSeats.value = props.seats.filter(seat => seat.last_activity_at == null);

        unusedSeats.value = totalSeats.value.filter(seat => {
          if (seat.last_activity_at === null) {
            return true;
          }

          const lastActivityDate = new Date(seat.last_activity_at);
          return lastActivityDate < oneWeekAgo;
        });

        updateChartData();
        updateRecencyChartData();
        updateAverageTimeChart();
      }
    });


    // Filtered Seats based on user selection
    const filteredSeats = computed(() => {
      if (activityStatus.value === 'All') {
        return totalSeats.value;
      } else if (activityStatus.value === 'Active') {
        if (recencyFilter.value) {
          return activeSeats.value[recencyFilter.value];
        }
        return totalSeats.value.filter(seat => seat.last_activity_at);
      } else if (activityStatus.value === 'Unused') {
        return totalSeats.value.filter(seat => !seat.last_activity_at);
      } else if (activityStatus.value === 'Inactive') {
        if (recencyFilter.value) {
          return inactiveSeats.value[recencyFilter.value];
        }
        return totalSeats.value.filter(seat => !seat.last_activity_at);
      } else {
        return activeSeats.value[activityStatus.value];
      }
    });

    return {
      totalSeats,
      NoshowSeats,
      unusedSeats,
      selectedTeam,
      activityStatus,
      filteredSeats,
      chartData,
      chartOptions,
      recencyChartData,
      recencyChartInactiveOptions,
      recencyChartInactiveData,
      recencyChartOptions,
      averageTimeChartData,
      averageTimeChartOptions,
      headers,
      averageTimeToFirstActivity,
      activityStatusOptions,
      recencyFilterOptions,
      recencyFilter,
      teams
    };
  }
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