<template>
  <div>
    <!-- Dashboard Container for Metrics -->
    <v-container fluid class="dashboard-container">
      <v-row justify="center" align="center">
        <!-- Cumulative Number of Turns Card -->
        <v-col cols="12" sm="6" md="3">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Cumulative Number of Turns</div>
                <div class="metric-subtitle">Over the last 28 days</div>
                <p class="metric-value">{{ cumulativeNumberTurns }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>

        <!-- Cumulative Number of Acceptances Card -->
        <v-col cols="12" sm="6" md="3">
          <v-card elevation="4" class="metric-card">
            <v-card-item>
              <div class="card-content">
                <div class="metric-title">Cumulative Number of Acceptances</div>
                <div class="metric-subtitle">Over the last 28 days</div>
                <p class="metric-value">{{ cumulativeNumberAcceptances }}</p>
              </div>
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>
    </v-container>

    <!-- Charts Section -->
    <v-container fluid class="charts-container">
      <v-row>
        <v-col cols="12">
          <h3 class="text-center">Total Acceptances | Total Turns Count</h3>
          <v-card class="chart-card">
            <v-card-item>
              <Line :data="totalNumberAcceptancesAndTurnsChartData" :options="chartOptions" />
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>

      <v-row>
        <v-col cols="12">
          <h3 class="text-center">Total Active Copilot Chat Users</h3>
          <v-card class="chart-card">
            <v-card-item>
              <Bar :data="totalActiveCopilotChatUsersChartData" :options="totalActiveChatUsersChartOptions" />
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </div>
</template>
  
<script lang="ts">
  import { defineComponent, ref, toRef } from 'vue';
  import { Metrics } from '../model/Metrics';
  import { Line, Bar } from 'vue-chartjs'
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
  Legend
} from 'chart.js'

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
)

export default defineComponent({
name: 'CopilotChatViewer',
props: {
        metrics: {
            type: Object,
            required: true
        }
    },
components: {
Bar,
Line
},
setup(props) {

    let cumulativeNumberAcceptances = ref(0);

    let cumulativeNumberTurns = ref(0);

    //Total Copilot Chat Active Users
    const totalActiveCopilotChatUsersChartData = ref<{ labels: string[]; datasets: any[] }>({ labels: [], datasets: [] });  

    const totalActiveChatUsersChartOptions = {
    responsive: true,
    maintainAspectRatio: true,
    scales: {
        y: {
        beginAtZero: true,
        ticks: {
            stepSize: 1
        }
        }
    },
    layout: {
        padding: {
        left: 50,
        right: 50,
        top: 50,
        bottom: 50
        }
    },
    };

    const chartOptions = {
        responsive: true,
        maintainAspectRatio: true,
        height: 300,
        width: 300,
        layout: {
            padding: {
            left: 150,
            right: 150,
            top: 20,
            bottom: 40
            }
        },
    };

    //Total Number Acceptances And Turns
    const totalNumberAcceptancesAndTurnsChartData = ref<{ labels: string[]; datasets: any[] }>({ labels: [], datasets: [] });

    const data = toRef(props, 'metrics').value;

    cumulativeNumberTurns.value = 0;
    const cumulativeNumberTurnsData = data.map((m: Metrics)  => {        
        cumulativeNumberTurns.value += m.total_chat_turns;
        return m.total_chat_turns;
    });

    cumulativeNumberAcceptances.value = 0;
    const cumulativeNumberAcceptancesData = data.map((m: Metrics)  => {        
        cumulativeNumberAcceptances.value += m.total_chat_acceptances;
        return m.total_chat_acceptances;
    });

    totalNumberAcceptancesAndTurnsChartData.value = {
    labels: data.map((m: Metrics)  => m.day),
        datasets: [
        {
            label: 'Total Acceptances',
            data: cumulativeNumberAcceptancesData,
            backgroundColor: 'rgba(75, 192, 192, 0.2)',
            borderColor: 'rgba(75, 192, 192, 1)'

        },
        {
            label: 'Total Turns',
            data: cumulativeNumberTurnsData,
            backgroundColor: 'rgba(153, 102, 255, 0.2)',
            borderColor: 'rgba(153, 102, 255, 1)'
        }]
    };

    totalActiveCopilotChatUsersChartData.value = {
        labels: data.map((m: Metrics) => m.day),
        datasets: [
        {
            label: 'Total Active Copilot Chat Users',
            data: data.map((m: Metrics) => m.total_active_chat_users),
            backgroundColor: 'rgba(0, 0, 139, 0.2)', // dark blue with 20% opacity
            borderColor: 'rgba(255, 99, 132, 1)'
        }]
    };
    
    return {  totalActiveCopilotChatUsersChartData, totalActiveChatUsersChartOptions,cumulativeNumberAcceptances, cumulativeNumberTurns, totalNumberAcceptancesAndTurnsChartData, chartOptions};
}
});

</script>
