<template>
  <AppLayout>
    <div class="dora-metrics-view">
      <h1 class="page-title">DORA Metrics Dashboard</h1>
      <p class="page-description">
        Track your development team's performance using DORA metrics: Deployment Frequency, Lead
        Time for Changes, Time to Restore Service, and Change Failure Rate.
      </p>

      <DoraDashboard />
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useStore } from 'vuex'
import AppLayout from '@/layouts/AppLayout.vue'
import DoraDashboard from '@/components/metrics/dora/DoraDashboard.vue'

const store = useStore()

onMounted(() => {
  console.log('DoraMetricsView: Component mounted')
  // Reset and initialize the repositories
  store
    .dispatch('DoraData/resetAndFetchRepositories')
    .then(() => {
      console.log('DoraMetricsView: Repositories fetched')
    })
    .catch(error => {
      console.error('DoraMetricsView: Error fetching repositories:', error)
    })
})
</script>

<style scoped>
.dora-metrics-view {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.page-title {
  font-size: 1.75rem;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.page-description {
  color: #6b7280;
  margin: 0;
  max-width: 800px;
  line-height: 1.5;
}
</style>
