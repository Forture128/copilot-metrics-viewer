<template>
  <AppLayout>
    <div class="language-analysis-view">
      <div class="view-header">
        <h1 class="view-title">Language Analysis</h1>
        <p class="view-description">
          Analyze Copilot usage patterns across different programming languages.
        </p>
      </div>

      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner" />
        <span>Loading language data...</span>
      </div>

      <div v-else-if="error" class="error-state">
        <div class="error-icon">⚠️</div>
        <div class="error-content">
          <h3 class="error-title">Error Loading Data</h3>
          <p class="error-message">{{ error }}</p>
        </div>
      </div>

      <div v-else-if="!metrics || metrics.length === 0" class="empty-state">
        <p>No language data available. Please check your connection or try again later.</p>
      </div>

      <div v-else>
        <BreakdownComponent
          :metrics="metrics"
          breakdown-key="language"
          class="analysis-component"
        />
      </div>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { computed, onMounted, watch, ref } from 'vue'
import { useStore } from 'vuex'
import { useRouter } from 'vue-router'
import AppLayout from '@/layouts/AppLayout.vue'
import BreakdownComponent from '@/components/analysis/BreakdownComponent.vue'

console.log('[LanguageAnalysisView] Component initializing')

// Store and router
const store = useStore()
const router = useRouter()

// Add a local state to track if we attempted to fetch data
const hasFetchAttempted = ref(false)

// Computed
const metrics = computed(() => {
  const data = store.state.CopilotUsage?.metrics || null
  console.log(
    '[LanguageAnalysisView] Computed metrics:',
    data ? `Found ${Array.isArray(data) ? data.length : 1} metric records` : 'No metrics data'
  )
  return data
})

const isLoading = computed(() => {
  const loading = store.state.CopilotUsage?.metricsLoading || false
  console.log('[LanguageAnalysisView] Computed isLoading:', loading)
  return loading
})

const metricsReady = computed(() => {
  return store.state.CopilotUsage?.metricsReady || false
})

const error = computed(() => {
  const err = store.state.CopilotUsage?.apiError || null
  if (err) console.log('[LanguageAnalysisView] Computed error:', err)
  return err
})

// Add watchers for debugging
watch(metrics, (newVal, oldVal) => {
  console.log(
    '[LanguageAnalysisView] Metrics changed:',
    newVal ? `Now has ${Array.isArray(newVal) ? newVal.length : 1} records` : 'No data',
    oldVal
      ? `Previously had ${Array.isArray(oldVal) ? oldVal.length : 1} records`
      : 'No previous data'
  )
})

watch(isLoading, (newVal, oldVal) => {
  console.log(
    '[LanguageAnalysisView] Loading state changed:',
    newVal ? 'Now loading' : 'Loading complete',
    oldVal ? 'Was previously loading' : 'Was previously not loading'
  )

  // If loading just finished and we don't have metrics, we might need to retry
  if (!newVal && oldVal && !metrics.value && !metricsReady.value && !error.value) {
    console.log('[LanguageAnalysisView] Loading finished but no data, checking auth status')
    checkAuthAndFetch()
  }
})

// Watch for authentication state changes
watch(
  () => store.getters['auth/isAuthenticated'],
  isAuthenticated => {
    console.log(
      '[LanguageAnalysisView] Auth state changed:',
      isAuthenticated ? 'Authenticated' : 'Not authenticated'
    )
    if (isAuthenticated && !metrics.value && !isLoading.value) {
      console.log('[LanguageAnalysisView] Authenticated and no metrics, will fetch data')
      fetchData()
    } else if (!isAuthenticated) {
      console.log('[LanguageAnalysisView] Not authenticated, redirecting to login')
      router.push('/login')
    }
  }
)

// Lifecycle Hooks
onMounted(() => {
  console.log('[LanguageAnalysisView] Component mounted')
  console.log(
    '[LanguageAnalysisView] Initial metrics:',
    metrics.value
      ? `Has ${Array.isArray(metrics.value) ? metrics.value.length : 1} records`
      : 'No data'
  )
  console.log('[LanguageAnalysisView] Initial loading state:', isLoading.value)
  console.log('[LanguageAnalysisView] Initial metrics ready state:', metricsReady.value)

  checkAuthAndFetch()
})

// Methods
const checkAuthAndFetch = () => {
  // Check authentication status first
  if (!store.getters['auth/isAuthenticated']) {
    console.log('[LanguageAnalysisView] Not authenticated, redirecting to login')
    router.push('/login')
    return
  }

  // If authenticated but no data and not already loading, fetch data
  if (!metrics.value && !isLoading.value && !hasFetchAttempted.value) {
    console.log('[LanguageAnalysisView] Authenticated, no metrics and not loading, will fetch data')
    fetchData()
  } else {
    console.log('[LanguageAnalysisView] Either has metrics, already loading, or fetch attempted')
  }
}

const fetchData = async () => {
  console.log('[LanguageAnalysisView] Starting data fetch')
  hasFetchAttempted.value = true

  try {
    console.log('[LanguageAnalysisView] Dispatching CopilotUsage/fetchMetrics action')
    await store.dispatch('CopilotUsage/fetchMetrics')
    console.log('[LanguageAnalysisView] Fetch completed successfully')
  } catch (err: unknown) {
    console.error('[LanguageAnalysisView] Error fetching language metrics:', err)

    // Check if the error is related to authentication
    if (
      err &&
      typeof err === 'object' &&
      'response' in err &&
      err.response &&
      typeof err.response === 'object' &&
      'status' in err.response &&
      err.response.status === 401
    ) {
      console.log('[LanguageAnalysisView] Auth error detected, checking authentication status')
      // This will be handled by the auth module and redirect as needed
    }
  }
}
</script>

<style scoped>
.language-analysis-view {
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
