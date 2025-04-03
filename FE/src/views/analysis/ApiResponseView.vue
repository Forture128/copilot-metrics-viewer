<template>
  <AppLayout>
    <div class="api-response-view">
      <div class="view-header">
        <h1 class="view-title">API Response Analysis</h1>
        <p class="view-description">
          Analyze Copilot API response metrics including latency, accuracy, and success rates.
        </p>
      </div>

      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner" />
        <span>Loading API response data...</span>
      </div>

      <div v-else-if="error" class="error-state">
        <div class="error-icon">⚠️</div>
        <div class="error-content">
          <h3 class="error-title">Error Loading Data</h3>
          <p class="error-message">{{ error }}</p>
        </div>
      </div>

      <div v-else-if="!apiData" class="empty-state">
        <p>No API response data available. Please check your connection or try again later.</p>
      </div>

      <ApiResponse v-else :api-data="apiData" class="analysis-component" />
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useStore } from 'vuex'
import AppLayout from '@/layouts/AppLayout.vue'
import ApiResponse from '@/components/analysis/ApiResponse.vue'

// Store
const store = useStore()

// Computed
const apiData = computed(() => store.state.CopilotUsage?.apiResponseData || null)
const isLoading = computed(() => store.state.CopilotUsage?.apiResponseLoading)
const error = computed(() => store.state.CopilotUsage?.apiResponseError || null)

// Lifecycle Hooks
onMounted(() => {
  if (!apiData.value && !isLoading.value) {
    fetchApiData()
  }
})

// Methods
const fetchApiData = async () => {
  try {
    await store.dispatch('CopilotUsage/fetchApiResponseData')
  } catch (err) {
    console.error('Error fetching API response data:', err)
  }
}
</script>

<style scoped>
.api-response-view {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.view-header {
  margin-bottom: 1rem;
}

.view-title {
  font-size: 1.75rem;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.view-description {
  color: #6b7280;
  margin: 0.5rem 0 0;
  max-width: 800px;
  line-height: 1.5;
}

.loading-state,
.empty-state,
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  background: white;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  text-align: center;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid rgba(79, 70, 229, 0.2);
  border-radius: 50%;
  border-top-color: #4f46e5;
  animation: spin 1s ease-in-out infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.error-state {
  flex-direction: row;
  text-align: left;
  gap: 1rem;
  background-color: #fee2e2;
  color: #b91c1c;
}

.error-icon {
  font-size: 1.5rem;
}

.error-title {
  margin: 0 0 0.25rem;
  font-weight: 600;
}

.error-message {
  margin: 0;
  color: #b91c1c;
}

.analysis-component {
  background: white;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  padding: 1.5rem;
}
</style>
