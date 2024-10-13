<template>
    <v-card class="metric-card">
        <v-card-title class="dora-title" style="text-align: left;">
            Mean Time to Restore (MTTR)
            <v-tooltip bottom>
                <template v-slot:activator="{ attrs }">
                    <v-icon small class="ml-2" v-bind="attrs">mdi-information</v-icon>
                </template>
                <span>MTTR measures the average time to restore service after a failed deployment.</span>
            </v-tooltip>
        </v-card-title>
        <v-row>
            <v-col cols="12" md="6">
                <v-row class="cycle-metrics">
                    <v-col cols="12" class="cycle-metric-time" style="text-align: center; padding: 20px;">
                        <span class="main-time" style="font-size: 2.0em; font-weight: bold;">{{ formattedMTTR }}</span>
                        <span class="time-change positive" style="font-size: 1.5em; color: green; margin-left: 10px;">▼
                            37
                            %</span>
                    </v-col>
                    <v-col cols="12">
                        <v-row justify="space-between" class="metric-breakdown">
                            <v-col cols="12" class="text-center">
                                <small>Mean Time to Restore: <b>{{ formattedMTTR }}</b></small>
                            </v-col>
                        </v-row>
                    </v-col>
                </v-row>
            </v-col>
            <v-col cols="12" md="6">
                <div style="width: 100%; position: relative;">
                    <Line :data="mttrData" :options="chartOptions" />
                </div>
            </v-col>
        </v-row>
    </v-card>
</template>

<script lang="ts">
import { defineComponent, computed, PropType } from 'vue';
import { Line } from 'vue-chartjs';

export default defineComponent({
    name: 'MTTRCard',
    components: { Line },
    props: {
        statuses: {
            type: Array as PropType<any[]>,
            required: true,
        },
    },
    setup(props) {
        // Function to calculate time difference in hours between two ISO date strings
        const calculateTimeDifferenceInHours = (startTime: string, endTime: string) => {
            const start = new Date(startTime).getTime();
            const end = new Date(endTime).getTime();
            return (end - start) / (1000 * 60 * 60); // Convert milliseconds to hours
        };

        // Computed property to calculate MTTR from deployment statuses
        const formattedMTTR = computed(() => {
            const failedDeployments = props.statuses.filter(status => status.state === 'failure');
            const mttrList: number[] = [];

            failedDeployments.forEach(failure => {
                // Find the next "success" status after the failure
                const recovery = props.statuses.find(
                    status => status.created_at > failure.created_at && status.state === 'success'
                );

                if (recovery) {
                    const timeToRestore = calculateTimeDifferenceInHours(failure.created_at, recovery.created_at);
                    mttrList.push(timeToRestore);
                }
            });

            // Calculate the average MTTR
            const averageMTTR = mttrList.length > 0 ? (mttrList.reduce((a, b) => a + b, 0) / mttrList.length) : 0;

            // Format the MTTR in hours and minutes
            const hours = Math.floor(averageMTTR);
            const minutes = Math.floor((averageMTTR - hours) * 60);
            return `${hours}h ${minutes}m`;
        });

        // Sample MTTR chart data (this can be updated based on actual data)
        const mttrData = computed(() => ({
            labels: props.statuses.map(status => status.created_at.split('T')[0]), // Format the date
            datasets: [
                {
                    data: props.statuses.map(status => (status.state === 'failure' ? 5 : 0)), // Example data
                    borderColor: 'rgba(75, 192, 192, 1)',
                    backgroundColor: 'rgba(75, 192, 192, 0.2)',
                },
            ],
        }));

        // Chart options
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
            formattedMTTR,
            mttrData,
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