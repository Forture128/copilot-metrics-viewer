<template>
    <v-card class="metric-card">
        <v-card-title class="dora-title" style="text-align: left;">
            Change Failure Rate
            <v-tooltip bottom>
                <template v-slot:activator="{ attrs }">
                    <v-icon small class="ml-2" v-bind="attrs">mdi-information</v-icon>
                </template>
                <span>Change Failure Rate measures the percentage of failed deployments.</span>
            </v-tooltip>
        </v-card-title>
        <v-row>
            <v-col cols="12" md="6">
                <v-row class="cycle-metrics">
                    <v-col cols="12" class="cycle-metric-time" style="text-align: center; padding: 20px;">
                        <span class="main-time" style="font-size: 2.0em; font-weight: bold;">{{ formattedFailureRate
                            }}</span>
                        <span class="time-change negative" style="font-size: 1.5em; color: red; margin-left: 10px;">▲
                            28.9
                            %</span>
                    </v-col>
                    <v-col cols="12">
                        <v-row justify="space-between" class="metric-breakdown">
                            <v-col cols="6" class="text-center">
                                <small>Failed Deployments: <b>{{ failedDeployments }}</b></small>
                            </v-col>
                            <v-col cols="6" class="text-center">
                                <small>Total Deployments: <b>{{ totalDeployments }}</b></small>
                            </v-col>
                        </v-row>
                    </v-col>
                </v-row>
            </v-col>
            <v-col cols="12" md="6">
                <div style="width: 100%; position: relative;">

                    <Bar :data="changeFailureRateData" :options="chartOptions" />
                </div>
            </v-col>
        </v-row>
    </v-card>
</template>

<script lang="ts">
import { defineComponent, computed, ref, PropType } from 'vue';
import { Bar } from 'vue-chartjs';

export default defineComponent({
    name: 'ChangeFailureRateCard',
    components: { Bar },
    props: {
        statuses: {
            type: Array as PropType<any[]>,
            required: true,
        },
    },
    setup(props) {
        // Compute the total and failed deployments based on statuses
        const totalDeployments = computed(() => props.statuses.length);
        
        const failedDeployments = computed(() => {
            return props.statuses?.filter(status => status.state === 'failure' || status.state === 'error').length || 0;
        });

        const formattedFailureRate = computed(() => {
            const rate = (failedDeployments.value / totalDeployments.value) * 100;
            return `${rate.toFixed(2)}%`;
        });

        // Bar chart data for visualizing failed deployments over time
        const changeFailureRateData = ref({
            labels: props.statuses.map(status => new Date(status.created_at).toLocaleDateString()),
            datasets: [
                {
                    label: 'Failed Deployments',
                    data: props.statuses?.map(status => (status.state === 'failure' || status.state === 'error' ? 1 : 0)) || [],
                    backgroundColor: 'rgba(255, 99, 132, 0.2)',
                    borderColor: 'rgba(255, 99, 132, 1)',
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

        return {
            formattedFailureRate,
            failedDeployments,
            totalDeployments,
            changeFailureRateData,
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