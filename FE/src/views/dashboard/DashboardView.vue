<template>
  <AppLayout>
    <div class="dashboard-content">
      <div class="metrics-overview">
        <h2>Metrics Overview</h2>
        <p>Select a domain to view detailed metrics:</p>

        <div class="metrics-cards">
          <div class="metric-card" @click="navigateTo('metrics/developer')">
            <h3>Developer Metrics</h3>
            <p>View individual developer performance metrics</p>
          </div>

          <div class="metric-card" @click="navigateTo('metrics/collaboration')">
            <h3>Collaboration Quality</h3>
            <p>Analyze team collaboration and code review metrics</p>
          </div>

          <div class="metric-card" @click="navigateTo('metrics/delivery')">
            <h3>Delivery Insights</h3>
            <p>Track deployment frequency and lead time metrics</p>
          </div>
        </div>
      </div>

      <div class="recent-activity">
        <h2>Recent Activity</h2>
        <p class="placeholder-text" v-if="!recentActivity.length">No recent activity to display</p>

        <ul class="activity-list" v-else>
          <li v-for="(activity, index) in recentActivity" :key="index" class="activity-item">
            <span class="activity-date">{{ formatDate(activity.date) }}</span>
            <span class="activity-description">{{ activity.description }}</span>
          </li>
        </ul>
      </div>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import AppLayout from '@/layouts/AppLayout.vue'

// Router
const router = useRouter()

// Data
const recentActivity = ref([
  // This would typically come from an API
  // For demo purposes, we'll use static data
  {
    date: new Date(Date.now() - 2 * 60 * 60 * 1000), // 2 hours ago
    description: 'User report for Q2 2023 was generated'
  },
  {
    date: new Date(Date.now() - 8 * 60 * 60 * 1000), // 8 hours ago
    description: 'New developer metrics were processed'
  },
  {
    date: new Date(Date.now() - 24 * 60 * 60 * 1000), // 1 day ago
    description: 'Team collaboration report was updated'
  }
])

// Methods
const navigateTo = (route: string) => {
  // Special case for delivery insights - redirect to DORA metrics
  if (route === 'metrics/delivery') {
    router.push('/metrics/dora')
  } else {
    router.push(`/${route}`)
  }
}

const formatDate = (date: Date): string => {
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMinutes = Math.floor(diffMs / (1000 * 60))

  if (diffMinutes < 60) {
    return `${diffMinutes} minute${diffMinutes !== 1 ? 's' : ''} ago`
  }

  const diffHours = Math.floor(diffMs / (1000 * 60 * 60))
  if (diffHours < 24) {
    return `${diffHours} hour${diffHours !== 1 ? 's' : ''} ago`
  }

  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24))
  if (diffDays < 7) {
    return `${diffDays} day${diffDays !== 1 ? 's' : ''} ago`
  }

  return date.toLocaleDateString()
}
</script>

<style scoped>
.dashboard-content {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.metrics-overview,
.recent-activity {
  background-color: white;
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.metrics-overview h2,
.recent-activity h2 {
  font-size: 1.25rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
  color: #333;
}

.metrics-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
  margin-top: 1.5rem;
}

.metric-card {
  background-color: #f9fafb;
  border-radius: 8px;
  padding: 1.5rem;
  cursor: pointer;
  transition:
    transform 0.2s,
    box-shadow 0.2s;
  border: 1px solid #e5e7eb;
}

.metric-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.metric-card h3 {
  font-size: 1.125rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
  color: #4f46e5;
}

.metric-card p {
  color: #6b7280;
  font-size: 0.875rem;
  margin: 0;
}

.placeholder-text {
  color: #9ca3af;
  font-style: italic;
}

.activity-list {
  list-style-type: none;
  padding: 0;
  margin: 1rem 0 0;
}

.activity-item {
  display: flex;
  padding: 0.75rem 0;
  border-bottom: 1px solid #f3f4f6;
}

.activity-item:last-child {
  border-bottom: none;
}

.activity-date {
  color: #6b7280;
  font-size: 0.875rem;
  width: 120px;
  flex-shrink: 0;
}

.activity-description {
  flex-grow: 1;
}
</style>
