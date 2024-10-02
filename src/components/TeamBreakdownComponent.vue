<template>
    <div>
        <!-- Total Numbers Summary (including Number of Languages) -->
        <v-sheet v-if="breakdownKey != 'editor'" elevation="3" class="pa-3 total-summary mb-4">
            <v-row>
                <!-- Number of Breakdowns -->
                <v-col cols="3" class="text-center">
                    <div class="metric-box">
                        <v-icon color="blue" size="36px">mdi-format-list-bulleted</v-icon>
                        <div>
                            <strong>Number of {{ breakdownDisplayNamePlural }}:</strong>
                            <p class="metric-value">{{ numberOfBreakdowns }}</p>
                        </div>
                    </div>
                </v-col>

                <!-- Total Accepted Prompts -->
                <v-col cols="3" class="text-center">
                    <div class="metric-box">
                        <v-icon color="green" size="36px">mdi-check-circle</v-icon>
                        <div>
                            <strong>Total Accepted Prompts:</strong>
                            <p class="metric-value">{{ totalAcceptedPrompts }}</p>
                        </div>
                    </div>
                </v-col>

                <!-- Total Accepted Lines of Code -->
                <v-col cols="3" class="text-center">
                    <div class="metric-box">
                        <v-icon color="purple" size="36px">mdi-file-code</v-icon>
                        <div>
                            <strong>Total Accepted Lines of Code:</strong>
                            <p class="metric-value">{{ totalAcceptedLinesOfCode }}</p>
                        </div>
                    </div>
                </v-col>

                <!-- Average Acceptance Rate -->
                <v-col cols="3" class="text-center">
                    <div class="metric-box">
                        <v-icon color="orange" size="36px">mdi-percent</v-icon>
                        <div>
                            <strong>Average Acceptance Rate:</strong>
                            <p class="metric-value">{{ totalAcceptanceRate.toFixed(2) }}%</p>
                        </div>
                    </div>
                </v-col>
            </v-row>
        </v-sheet>

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
            <v-row v-if="breakdownKey != 'editor'" justify="center">
                <v-col cols="12" md="10">
                    <v-card class="chart-card">
                        <v-card-title class="text-center">{{ breakdownDisplayNamePlural }} Breakdown Bar Chart</v-card-title>
                        <v-card-text>
                            <Bar :data="breakdownsChartDataTop20AcceptedData" :options="chartOptions" />
                        </v-card-text>
                    </v-card>
                </v-col>
            </v-row>
            <v-row v-if="breakdownKey != 'editor'" justify="center">
                <v-col cols="12" md="10">
                    <v-card class="chart-card">
                        <v-card-title class="text-center">{{ breakdownDisplayNamePlural }} Breakdown Line Chart AcceptedPrompt Metrics</v-card-title>
                        <v-card-text>
                            <Line :data="breakdownsChartDataTop20AcceptedPromptData" :options="lineBarChartOptions" />
                        </v-card-text>
                    </v-card>
                </v-col>
            </v-row>

            <!-- Languages Breakdown Table -->
            <v-row>
                <v-col cols="12">
                    <h2 class="text-center">{{ breakdownDisplayNamePlural }} Breakdown</h2>
                    <v-card class="chart-card table-card">
                        <v-data-table :headers="headers" :items="Array.from(breakdowns)"
                            class="elevation-2 table-container">
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

<script lang="ts">
import { computed, defineComponent, ref, toRef } from "vue";
import { Metrics } from "../model/Metrics";
import { Breakdown } from "../model/Breakdown";
import { Bar, Line, Pie } from "vue-chartjs";

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
        Line,
        Pie,
        Bar,
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

        // Top 20 by language accepted prompts
        const breakdownsChartDataTop20AcceptedData = ref<{
            labels: string[];
            datasets: any[];
        }>({ labels: [], datasets: [] });
        // Top 20 by language accepted prompts
        const breakdownsChartDataTop20AcceptedPromptData = ref<{
            labels: string[];
            datasets: any[];
        }>({ labels: [], datasets: [] });

        const chartOptions = {
            responsive: true,
            maintainAspectRatio: true,
        };

        const lineBarChartOptions = {
            responsive: true,
            maintainAspectRatio: true,
            scales: {
                x: {
                    display: true,
                    title: {
                        display: true,
                        text: "Breakdown Name",
                    },
                },
                y: {
                    display: true,
                    title: {
                        display: true,
                        text: "Accepted Prompts",
                    },
                },
            },
            elements: {
                line: {
                    tension: 0.4, // Make the line smooth
                },
                point: {
                    radius: 4,
                    backgroundColor: "#fff",
                    borderWidth: 2,
                },
            },
        };

        const pieChartColors = ref([
            // "#FF6B6B", // Soft Red
            "#4ECDC4", // Teal
            "#1A535C", // Deep Sea Green
            "#FFE66D", // Warm Yellow
            "#FF6F61", // Coral
            "#346751", // Deep Forest Green
            "#F4A261", // Soft Orange
            "#2A9D8F", // Soft Turquoise
            "#E76F51", // Earthy Red
            "#264653", // Dark Blue Gray
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
        // Calculate total metrics
        const totalAcceptedPrompts = computed(() => Array.from(breakdowns.value.values()).reduce((acc, item) => acc + item.acceptedPrompts, 0));
        const totalAcceptedLinesOfCode = computed(() => Array.from(breakdowns.value.values()).reduce((acc, item) => acc + item.acceptedLinesOfCode, 0));
        const totalSuggestedLinesOfCode = computed(() => Array.from(breakdowns.value.values()).reduce((acc, item) => acc + item.suggestedLinesOfCode, 0));
        const totalAcceptanceRate = computed(() => {
            return (totalAcceptedLinesOfCode.value / totalSuggestedLinesOfCode.value) * 100;
        });

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
        // Get the top 5 breakdowns by accepted prompts
        const top5BreakdownsAcceptedPrompts = new Map(
            [...breakdowns.value].slice(0, 5)
        );

        // Get the top 20 breakdowns by accepted prompts
        const top20BreakdownsAcceptedPrompts = new Map(
            [...breakdowns.value].slice(0, 20)
        );

        breakdownsChartData.value = {
            labels: Array.from(top20BreakdownsAcceptedPrompts.values()).map(
                (breakdown) => breakdown.name
            ),
            datasets: [
                {
                    data: Array.from(top20BreakdownsAcceptedPrompts.values()).map(
                        (breakdown) => breakdown.acceptedPrompts
                    ),
                    backgroundColor: pieChartColors.value,
                },
            ],
        };
        // Line chart data
        breakdownsChartDataTop20AcceptedData.value = {
            labels: Array.from(top20BreakdownsAcceptedPrompts.values()).map(
                (breakdown) => breakdown.name
            ),
            datasets: [
                {
                    label: "Accepted Lines of Code",
                    data: Array.from(top20BreakdownsAcceptedPrompts.values()).map(
                        (breakdown) => breakdown.acceptedLinesOfCode
                    ),
                    backgroundColor: 'rgba(75, 192, 192, 0.6)',
                    borderColor: 'rgb(75, 192, 192)',
                    type: "bar",
                },
                {
                    label: "Suggestion Lines of Code",
                    data: Array.from(top20BreakdownsAcceptedPrompts.values()).map(
                        (breakdown) => breakdown.suggestedLinesOfCode
                    ),
                    backgroundColor: '#1A535C',
                    borderColor: 'rgb(75, 192, 192)',
                    type: "bar",
                },
            ],
        }
        breakdownsChartDataTop20AcceptedPromptData.value = {
            labels: Array.from(top20BreakdownsAcceptedPrompts.values()).map(
                (breakdown) => breakdown.name
            ),
            datasets: [
                {
                    label: "Accepted Prompts",
                    data: Array.from(top20BreakdownsAcceptedPrompts.values()).map(
                        (breakdown) => breakdown.acceptedPrompts
                    ),
                    backgroundColor: 'rgba(255, 99, 132, 0.6)',
                    borderColor: 'rgb(255, 99, 132)',
                },
            ],
        }

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

        numberOfBreakdowns.value = breakdowns.value.size;

        return {
            chartOptions,
            lineBarChartOptions,
            breakdowns,
            numberOfBreakdowns,
            breakdownsChartData,
            breakdownsChartDataTop20AcceptedData,
            breakdownsChartDataTop20AcceptedPromptData,
            breakdownsChartDataTop5AcceptedPrompts,
            breakdownsChartDataTop5AcceptanceRate,
            totalAcceptedPrompts,
            totalAcceptedLinesOfCode,
            totalAcceptanceRate,
        };
    },
});
</script>
<style scoped>
.dashboard-container {
    margin-bottom: 30px;
}

.table-container {
    max-height: 500px;
    overflow-y: auto;
}

.metric-title {
    font-size: 1.1em;
    font-weight: 600;
}

.metric-value {
    font-size: 1.6em;
    font-weight: bold;
    margin-top: 5px;
}

.total-summary {
    background-color: #f9f9f9;
    border-radius: 10px;
}

.chart-card {
    padding: 16px;
}

.chart-container {
    max-height: 300px;
}

.line-chart {
    max-height: 300px;
    padding: 15px;
}
</style>