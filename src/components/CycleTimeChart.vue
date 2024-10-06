<template>
    <Bar :data="cycleTimeChartData" :options="chartOptions" />
</template>

<script>
import { Bar } from 'vue-chartjs';

export default {
    name: 'CycleTimeChart',
    components: {
        Bar,
    },
    props: {
        data: {
            type: Object,
            required: true,
        },
    },
    setup() {
        const chartOptions = {
            responsive: true,
            plugins: {
                legend: {
                    display: false,
                },
                tooltip: {
                    callbacks: {
                        label: function (tooltipItem) {
                            const label = tooltipItem.dataset.label || '';
                            const value = tooltipItem.raw || 0;
                            return `${label}: ${value} hours`;
                        },
                    },
                },
            },
            scales: {
                x: {
                    title: {
                        display: true,
                        text: 'Stages',
                    },
                },
                y: {
                    beginAtZero: true,
                    title: {
                        display: true,
                        text: 'Hours',
                    },
                },
            },
        };

        return {
            chartOptions,
        };
    },
};
</script>