<template>
  <div class="copilot-chat-viewer">
    <h2 class="title">Copilot Chat Metrics</h2>

    <div v-if="!chatData" class="empty-state">No chat data available.</div>

    <div v-else class="chat-data">
      <!-- Overview Stats -->
      <div class="stats-cards">
        <div class="stat-card">
          <div class="stat-value">{{ chatData.totalChats.toLocaleString() }}</div>
          <div class="stat-label">Total Chats</div>
        </div>

        <div class="stat-card">
          <div class="stat-value">{{ chatData.averageMessagesPerChat }}</div>
          <div class="stat-label">Avg Messages Per Chat</div>
        </div>

        <div class="stat-card">
          <div class="stat-value">{{ chatData.responseTime.average }}s</div>
          <div class="stat-label">Avg Response Time</div>
        </div>
      </div>

      <!-- Top Questions -->
      <div class="top-questions">
        <h3>Top Questions</h3>
        <ul class="question-list">
          <li v-for="(question, index) in chatData.topQuestions" :key="index">"{{ question }}"</li>
        </ul>
      </div>

      <!-- Response Time Chart (placeholder) -->
      <div class="response-time">
        <h3>Response Time Distribution</h3>
        <div class="chart-placeholder">
          <p>Chart would be displayed here in the final implementation.</p>
          <p>p95 Response Time: {{ chatData.responseTime.percentile95 }}s</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Props
const props = defineProps({
  chatData: {
    type: Object,
    required: true
  }
})
</script>

<style scoped>
.copilot-chat-viewer {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.title {
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
  margin: 0 0 1rem;
}

.stats-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.stat-card {
  background-color: #f8fafc;
  border-radius: 0.5rem;
  padding: 1.5rem;
  text-align: center;
  transition:
    transform 0.2s,
    box-shadow 0.2s;
}

.stat-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
}

.stat-value {
  font-size: 2rem;
  font-weight: 700;
  color: #4f46e5;
  margin-bottom: 0.5rem;
}

.stat-label {
  color: #64748b;
  font-size: 0.875rem;
}

.top-questions,
.response-time {
  background-color: white;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  padding: 1.5rem;
  margin-bottom: 1.5rem;
}

.top-questions h3,
.response-time h3 {
  font-size: 1.25rem;
  margin: 0 0 1rem;
  color: #334155;
}

.question-list {
  padding-left: 1.5rem;
  color: #334155;
}

.question-list li {
  margin-bottom: 0.75rem;
  line-height: 1.5;
}

.chart-placeholder {
  background-color: #f8fafc;
  border-radius: 0.375rem;
  padding: 2rem;
  text-align: center;
  color: #64748b;
}

.empty-state {
  padding: 2rem;
  text-align: center;
  color: #64748b;
  background-color: #f8fafc;
  border-radius: 0.375rem;
}
</style>
