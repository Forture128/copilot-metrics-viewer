<template>
  <div class="min-h-screen bg-background">
    <!-- Header -->
    <header
      class="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
    >
      <div class="container flex h-14 max-w-screen-2xl items-center">
        <!-- GitHub Button -->
        <Button variant="ghost" size="icon" class="mr-4">
          <Github class="h-5 w-5" />
        </Button>

        <!-- Title -->
        <h1 class="font-semibold text-lg">
          Copilot Metrics Viewer | {{ capitalizedItemName }} : {{ displayedViewName }}
        </h1>

        <!-- Mocked Data Message -->
        <div v-if="mockedDataMessage" class="ml-4 text-destructive text-sm font-medium">
          {{ mockedDataMessage }}
        </div>
      </div>

      <!-- Navigation Tabs -->
      <div class="container max-w-screen-2xl border-t">
        <nav class="flex h-14 items-center space-x-4">
          <Button
            v-for="item in tabItems"
            :key="item"
            variant="ghost"
            :class="[
              'flex items-center gap-2 h-14 px-4 py-2 -mb-px text-sm font-medium transition-colors hover:text-primary',
              tab === item ? 'border-b-2 border-primary text-primary' : 'text-muted-foreground'
            ]"
            @click="tab = item"
          >
            <component :is="getIconComponent(item)" class="h-4 w-4" />
            <span class="capitalize">{{ item }}</span>
          </Button>
        </nav>
      </div>
    </header>

    <!-- Main Content -->
    <main class="container max-w-screen-2xl py-6">
      <!-- Error Message -->
      <Alert v-if="apiError" variant="destructive" class="mb-6">
        <AlertCircle class="h-4 w-4" />
        <AlertTitle>Error</AlertTitle>
        <AlertDescription>{{ apiError }}</AlertDescription>
      </Alert>

      <div v-if="!apiError">
        <!-- Loading Progress -->
        <Progress v-if="!metricsReady" class="w-full" />

        <!-- Content -->
        <div v-if="metricsReady" class="mt-4">
          <MetricsViewer v-if="tab === itemName" :metrics="metrics" :team-metrics="teamMetrics" />
          <DepartmentsMetricsViewer
            v-if="tab === 'departments'"
            :team-metrics="teamMetrics"
            :departments="departments"
          />
          <BreakdownComponent
            v-if="tab === 'languages'"
            :metrics="metrics"
            breakdown-key="language"
          />
          <BreakdownComponent v-if="tab === 'editors'" :metrics="metrics" breakdown-key="editor" />
          <CopilotChatViewer v-if="tab === 'copilot chat'" :metrics="metrics" />
          <SeatsAnalysisViewer v-if="tab === 'seat analysis'" :seats="seats" />
          <ApiResponse v-if="tab === 'api response'" :metrics="metrics" :seats="seats" />
          <TeamMetricsViewer
            v-if="tab === 'team metrics' && teamMetricsReady"
            v-model:team="selectedTeam"
            :teams="teamList"
            :metrics="teamMetrics"
            breakdown-key="team metrics"
          />
          <DoraDashboard v-if="tab === 'dora dashboard'" />
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useStore } from 'vuex'
import config from '../config'

// shadcn-vue components
import { Button } from '@/components/ui/button'
import { Progress } from '@/components/ui/progress'
import { Alert, AlertTitle, AlertDescription } from '@/components/ui/alert'

// Lucide icons
import {
  Github,
  Building,
  Users,
  Code2,
  Edit3,
  MessageSquare,
  FileJson,
  UserCheck,
  LayoutDashboard,
  AlertCircle
} from 'lucide-vue-next'

// Components
import MetricsViewer from './MetricsViewer.vue'
import BreakdownComponent from './BreakdownComponent.vue'
import CopilotChatViewer from './CopilotChatViewer.vue'
import SeatsAnalysisViewer from './SeatsAnalysisViewer.vue'
import ApiResponse from './ApiResponse.vue'
import TeamMetricsViewer from './TeamMetricsViewer.vue'
import DoraDashboard from './DoraDashboard.vue'
import DepartmentsMetricsViewer from './DepartmentsMetricsViewer.vue'

const store = useStore()
const tab = ref('')

// Computed properties
const metricsReady = computed(() => store.state.CopilotUsage.metricsReady)
const metrics = computed(() => store.state.CopilotUsage.metrics)
// const seatsReady = computed(() => store.state.CopilotUsage.seatsReady)
const seats = computed(() => store.state.CopilotUsage.seats)
const teamList = computed(() => store.state.CopilotUsage.teamList)
console.log('Team List', teamList.value)
const teamMetrics = computed(() => store.state.CopilotUsage.teamMetrics)
const teamMetricsReady = computed(() => store.state.CopilotUsage.teamMetricsReady)
const apiError = computed(() => store.state.CopilotUsage.apiError)
const departments = computed(() => store.state.CopilotUsage.departments)
console.log('Departments', departments.value)
const itemName = computed(() => config.scope.type)
const capitalizedItemName = computed(
  () => itemName.value.charAt(0).toUpperCase() + itemName.value.slice(1)
)
const displayedViewName = computed(() => config.scope.name)
const mockedDataMessage = computed(() =>
  config.mockedData ? 'Using mock data - see README if unintended' : ''
)

// Tab items
const tabItems = [
  itemName.value,
  'departments',
  'team metrics'
  // 'languages',
  // 'editors',
  // 'copilot chat',
  // 'api response',
  // 'seat analysis',
  // 'dora dashboard',
]

// Set initial tab
tab.value = tabItems[0]

// Icon mapping
const getIconComponent = (item: string) => {
  const icons: Record<string, any> = {
    organization: Building,
    'team metrics': Users,
    departments: Building,
    languages: Code2,
    editors: Edit3,
    'copilot chat': MessageSquare,
    'api response': FileJson,
    'seat analysis': UserCheck,
    'dora dashboard': LayoutDashboard
  }
  return icons[item] || Building
}

// Fetch data
store.dispatch('CopilotUsage/fetchMetrics')
store.dispatch('CopilotUsage/fetchSeats')
store.dispatch('CopilotUsage/fetchAllTeamMetrics')

const teams = ref([
  'cloud_transfer_reviewers',
  'neo-ap',
  'rci-mfv',
  'attendance_dev_reviewers',
  'mf_connected_db_developers',
  'payroll_dev_reviewers',
  'payroll_kotlin_reviewers',
  'social_insurance_dev_reviewers',
  'tax_adjustment_dev_mfj_reviewers'
])

const selectedTeam = ref(teams.value[0])
</script>

<style>
.container {
  margin-left: auto;
  margin-right: auto;
  padding-left: 1rem;
  padding-right: 1rem;
}
</style>
