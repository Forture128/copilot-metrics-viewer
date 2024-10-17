<template>
    <v-card class="metric-card">
        <v-card-title class="dora-title" style="text-align: left;">
            Deployment Frequency
            <v-tooltip bottom>
                <template v-slot:activator="{ attrs }">
                    <v-icon small class="ml-2" v-bind="attrs">mdi-information</v-icon>
                </template>
                <span>Deployment frequency measures how often code is deployed to production.</span>
            </v-tooltip>
        </v-card-title>
        <v-row>
            <v-col cols="12" md="6">
                <v-row class="cycle-metrics">
                    <v-col cols="12" class="cycle-metric-time" style="text-align: center; padding: 20px;">
                        <span class="main-time" style="font-size: 2.0em; font-weight: bold;">{{ deploymentFrequency
                            }}</span>
                        <span class="time-change positive" style="font-size: 1.5em; color: green; margin-left: 10px;">▲
                            200
                            %</span>
                    </v-col>
                    <v-col cols="12">
                        <v-row justify="space-between" class="metric-breakdown">
                            <v-col cols="12" class="text-center">
                                <small>Total Deployments: <b>{{ totalDeployments }}</b></small>
                            </v-col>
                        </v-row>
                    </v-col>
                </v-row>
            </v-col>
            <v-col cols="12" md="6">
                <div style="width: 100%; position: relative;">
                    <Line :data="deploymentFrequencyData" :options="chartOptions" />
                </div>
            </v-col>
        </v-row>
    </v-card>
</template>

<script lang="ts">
import { defineComponent, computed, PropType } from 'vue';
import { Line } from 'vue-chartjs';

export default defineComponent({
    name: 'DeploymentFrequencyCard',
    components: { Line },
    props: {
        deployments: {
            type: Array as PropType<any[]>,
            required: true,
        },
        deploymentFrequency: {
            type: String,
            required: true,
        },
    },
    setup(props) {
        const totalDeployments = computed(() => props.deployments.length);
        const deploymentFrequencyData = computed(() => {
            const labels = props.deployments.map(deployment => new Date(deployment.created_at).toLocaleDateString());
            const dataset = props.deployments.map(() => 1); // Each deployment counts as 1

            return {
                labels,
                datasets: [
                    {
                        label: 'Deployments',
                        data: dataset,
                        borderColor: 'rgba(54, 162, 235, 1)',
                        backgroundColor: 'rgba(54, 162, 235, 0.2)',
                    },
                ],
            };
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
            totalDeployments,
            deploymentFrequencyData,
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