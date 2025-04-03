<template>
  <AppLayout>
    <div class="teams-view">
      <div class="view-header">
        <h1 class="view-title">Teams</h1>
        <p class="view-description">View and manage teams within your organization.</p>
      </div>

      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner" />
        <span>Loading teams data...</span>
      </div>

      <div v-else-if="error" class="error-state">
        <div class="error-icon">⚠️</div>
        <div class="error-content">
          <h3 class="error-title">Error Loading Data</h3>
          <p class="error-message">{{ error }}</p>
        </div>
      </div>

      <div v-else class="teams-content">
        <div class="placeholder-message">
          Teams management view is under development. Stay tuned for updates.
        </div>
      </div>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import AppLayout from '@/layouts/AppLayout.vue'

// State
const isLoading = ref(false)
const error = ref<string | null>(null)

// You would normally fetch data here
</script>

<style scoped>
.teams-view {
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
.error-state,
.placeholder-message {
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

.placeholder-message {
  font-size: 1.1rem;
  color: #6b7280;
}
</style>
