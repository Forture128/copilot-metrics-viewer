<template>
    <v-container fluid class="dora-dashboard">
        <v-row justify="center" align="center">
            <!-- Team Selector & Date Range Picker -->
            <v-col cols="12" md="12">
                <v-row align="center" justify="end">
                    <!-- Team Selector -->
                    <v-col cols="12" md="3">
                        <v-select v-model="selectedRepo" :items="repositoriesNames" label="Select Repository"
                            @change="fetchData" class="team-select"></v-select>
                    </v-col>
                    <v-col cols="12" md="3">
                    <v-select v-model="selectedTimePeriod" :items="timePeriods" label="Select Time Period"
                        class="time-period-select"></v-select>
                </v-col>
                <!-- <v-btn @click="loadMoreRepositories">Load More Repositories</v-btn> -->
                </v-row>

            </v-col>

            <!-- Metric Alerts -->
            <v-col cols="12">
                <v-alert type="info" class="metric-alert">
                    {{ metricAlert }}
                </v-alert>
            </v-col>

            <!-- Metric Cards -->
            <v-col cols="12" md="12">
                <CycleTimeCard :pullRequests="pullRequests" :selectedRepo="selectedRepo" />
            </v-col>
            <v-col cols="12" md="6">
                <DeploymentFrequencyCard :deployments="deployments" :deploymentFrequency="deploymentFrequency" />
            </v-col>
            <v-col cols="12" md="6">
                <ChangeFailureRateCard :statuses="statuses" />
            </v-col>
            <v-col cols="12" md="6">
                <MTTRCard :statuses="statuses" />
            </v-col>
            <v-col cols="12" md="6">
                <LeadTimeCard :pullRequests="pullRequests" />
            </v-col>
        </v-row>
    </v-container>
</template>

<script lang="ts">
import { defineComponent, ref, computed, watch, onMounted } from 'vue';
import DeploymentFrequencyCard from './DoraComponents/DeploymentFrequencyCard.vue';
import ChangeFailureRateCard from './DoraComponents/ChangeFailureRateCard.vue';
import MTTRCard from './DoraComponents/MTTRCard.vue';
import CycleTimeCard from './DoraComponents/CycleTimeCard.vue';
import LeadTimeCard from './DoraComponents/LeadTimeCard.vue';
import { useStore } from 'vuex';
import config from '@/config';

export default defineComponent({
    name: 'DoraDashboard',
    components: {
        CycleTimeCard,
        DeploymentFrequencyCard,
        ChangeFailureRateCard,
        MTTRCard,
        LeadTimeCard,
    },
    setup() {
        const store = useStore();
        const repositories = computed(() => store.state.DoraData.repositories);
        const repositoriesNames = computed(() => repositories.value.map((repo: Repository) => repo.name));
        const selectedTimePeriod = ref<string>('Weekly');
        const timePeriods = ref<string[]>(['Weekly', 'Monthly', 'Yearly']);

        interface Repository {
            name: string;
            // Add other properties if needed
        }
        const selectedRepo = ref<string>('eslint-config-moneyforward');
        const menu = ref(false);
        const date = ref({ start: new Date(), end: new Date() });
        const startDate = computed(() => date.value.start);
        const endDate = computed(() => date.value.end);

        const metricAlert = ref(
            'Cycle Time needs focus. Deployment PRs are good. Work on reducing time to restore system functionality after incidents.'
        );

        const updateDates = (newDate: any) => {
            date.value.start = newDate.start;
            date.value.end = newDate.end;
        };

        watch([selectedRepo, startDate, endDate], () => {
            fetchData();
        });
        const fetchData = () => {
            const owner = config.github.org;
            const repo = selectedRepo.value ? selectedRepo.value : 'eslint-config-moneyforward';
            // Fetch deployments and their statuses
            store.dispatch('DoraData/fetchDeploymentsAndStatuses', { owner, repo });
            store.dispatch('DoraData/fetchPullRequests', { owner, repo });
        };

        onMounted(() => {
            store.dispatch('DoraData/fetchRepositories').then(() => {
                selectedRepo.value = repositories.value[0].name;
                fetchData();
            });
        });
        const loadMoreRepositories = () => {
            store.dispatch('DoraData/loadMoreRepositories');
        };

        const deployments = computed(() => store.state.DoraData.deployments || []);
        const deploymentFrequency = computed(() => {
            if (!deployments.value.length) return '0/week';
            const totalDeployments = deployments.value.length;
            const weeks = Math.max(1, (endDate.value.getTime() - startDate.value.getTime()) / (1000 * 60 * 60 * 24 * 7));
            return `${(totalDeployments / weeks).toFixed(2)}/week`;
        });
        const pullRequests = computed(() => store.state.DoraData.pullRequests || []);
        const statuses = computed(() => store.state.DoraData.statuses || []);
        

        return {
            selectedTimePeriod, 
            timePeriods,
            repositoriesNames,
            selectedRepo,
            menu,
            date,
            startDate,
            endDate,
            metricAlert,
            updateDates,
            deployments,
            deploymentFrequency,
            pullRequests,
            statuses,
            fetchData,
            loadMoreRepositories
        };
    },
});
</script>

<style scoped>
.dora-dashboard {
    margin-bottom: 20px;
}

.metric-alert {
    display: flex;
    align-items: center;
    margin-bottom: 20px;
}

.team-select {
    max-width: 250px;
}
</style>