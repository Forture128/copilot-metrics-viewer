<template>
  <div class="metrics-viewer">
    <h2 class="title">Copilot Metrics Overview</h2>

    <div v-if="!metrics" class="empty-state">No metrics data available.</div>

    <div v-else class="metrics-data">
      <!-- Overview Cards -->
      <div class="metrics-cards">
        <div class="metric-card">
          <div class="metric-icon">📊</div>
          <div class="metric-content">
            <div class="metric-value">{{ metrics.totalSuggestions.toLocaleString() }}</div>
            <div class="metric-label">Total Suggestions</div>
          </div>
        </div>

        <div class="metric-card">
          <div class="metric-icon">✅</div>
          <div class="metric-content">
            <div class="metric-value">{{ (metrics.acceptanceRate * 100).toFixed(1) }}%</div>
            <div class="metric-label">Acceptance Rate</div>
          </div>
        </div>

        <div class="metric-card">
          <div class="metric-icon">📏</div>
          <div class="metric-content">
            <div class="metric-value">{{ metrics.averageSuggestionLength.toFixed(1) }}</div>
            <div class="metric-label">Avg. Length</div>
          </div>
        </div>
      </div>

      <!-- Language and Editor Breakdown -->
      <div class="breakdown-sections">
        <!-- Language Breakdown -->
        <div class="breakdown-section">
          <h3 class="section-title">Language Breakdown</h3>
          <div class="language-list">
            <div v-for="(count, language) in topLanguages" :key="language" class="language-item">
              <div class="language-info">
                <span class="language-name">{{ formatLanguageName(language) }}</span>
                <span class="language-count">{{ count.toLocaleString() }}</span>
              </div>
              <div class="bar-container">
                <div
                  class="bar"
                  :style="{ width: `${(count / metrics.totalSuggestions) * 100}%` }"
                />
              </div>
              <div class="language-percentage">
                {{ ((count / metrics.totalSuggestions) * 100).toFixed(1) }}%
              </div>
            </div>
          </div>
        </div>

        <!-- Editor Breakdown -->
        <div class="breakdown-section">
          <h3 class="section-title">Editor Breakdown</h3>
          <div class="editor-list">
            <div v-for="(count, editor) in metrics.byEditor" :key="editor" class="editor-item">
              <div class="editor-info">
                <span class="editor-name">{{ formatEditorName(editor) }}</span>
                <span class="editor-count">{{ count.toLocaleString() }}</span>
              </div>
              <div class="bar-container">
                <div
                  class="bar"
                  :style="{ width: `${(count / metrics.totalSuggestions) * 100}%` }"
                />
              </div>
              <div class="editor-percentage">
                {{ ((count / metrics.totalSuggestions) * 100).toFixed(1) }}%
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Call to Action for Detailed Views -->
      <div class="detailed-view-cta">
        <p>For more detailed analysis, check out:</p>
        <div class="cta-buttons">
          <router-link to="/analysis/language" class="cta-button"> Language Analysis </router-link>
          <router-link to="/analysis/editor" class="cta-button"> Editor Analysis </router-link>
          <router-link to="/metrics/teams" class="cta-button"> Team Metrics </router-link>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

// Define props type
interface Metrics {
  totalSuggestions: number
  acceptanceRate: number
  averageSuggestionLength: number
  byLanguage: Record<string, number>
  byEditor: Record<string, number>
}

// Props
const props = defineProps<{
  metrics: Metrics
}>()

// Computed
const topLanguages = computed(() => {
  if (!props.metrics || !props.metrics.byLanguage) return {}

  // Sort languages by count and get top 5
  const entries = Object.entries(props.metrics.byLanguage)
  entries.sort((a, b) => b[1] - a[1])

  // Convert back to object with top 5 languages
  return entries.slice(0, 5).reduce(
    (obj, [key, value]) => {
      obj[key] = value
      return obj
    },
    {} as Record<string, number>
  )
})

// Methods
const formatLanguageName = (name: string): string => {
  // Format snake_case or camelCase to Title Case
  const formattedName = name
    .replace(/([A-Z])/g, ' $1') // Insert a space before capital letters for camelCase
    .replace(/_/g, ' ') // Replace underscores with spaces for snake_case
    .replace(/^./, str => str.toUpperCase()) // Uppercase the first character

  // Handle special cases
  switch (name.toLowerCase()) {
    case 'js':
    case 'javascript':
      return 'JavaScript'
    case 'ts':
    case 'typescript':
      return 'TypeScript'
    case 'py':
      return 'Python'
    case 'cs':
      return 'C#'
    case 'cpp':
      return 'C++'
    default:
      return formattedName
  }
}

const formatEditorName = (name: string): string => {
  // Format the editor names
  switch (name.toLowerCase()) {
    case 'vscode':
      return 'VS Code'
    case 'intellij':
      return 'IntelliJ IDEA'
    case 'pycharm':
      return 'PyCharm'
    case 'webstorm':
      return 'WebStorm'
    case 'phpstorm':
      return 'PhpStorm'
    case 'vim':
      return 'Vim'
    default:
      return name.replace(/([A-Z])/g, ' $1').replace(/^./, str => str.toUpperCase())
  }
}
</script>

<style scoped>
.metrics-viewer {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.title {
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
  margin: 0 0 1rem;
}

.metrics-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
  margin-bottom: 1rem;
}

.metric-card {
  display: flex;
  align-items: center;
  gap: 1rem;
  background-color: white;
  border-radius: 0.5rem;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition:
    transform 0.2s,
    box-shadow 0.2s;
}

.metric-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
}

.metric-icon {
  font-size: 2rem;
  width: 60px;
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #f3f4f6;
  border-radius: 50%;
}

.metric-content {
  display: flex;
  flex-direction: column;
}

.metric-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: #111827;
}

.metric-label {
  font-size: 0.875rem;
  color: #6b7280;
}

.breakdown-sections {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
}

.breakdown-section {
  background-color: white;
  border-radius: 0.5rem;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.section-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: #111827;
  margin: 0 0 1.5rem;
}

.language-list,
.editor-list {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.language-item,
.editor-item {
  display: grid;
  grid-template-columns: 1fr;
  gap: 0.5rem;
}

.language-info,
.editor-info {
  display: flex;
  justify-content: space-between;
  font-size: 0.875rem;
}

.language-name,
.editor-name {
  font-weight: 500;
  color: #111827;
}

.language-count,
.editor-count {
  color: #6b7280;
}

.bar-container {
  height: 8px;
  background-color: #e5e7eb;
  border-radius: 4px;
  overflow: hidden;
}

.bar {
  height: 100%;
  background-color: #4f46e5;
  border-radius: 4px;
  transition: width 0.5s ease-out;
}

.language-percentage,
.editor-percentage {
  text-align: right;
  font-size: 0.75rem;
  color: #6b7280;
}

.detailed-view-cta {
  background-color: #f3f4f6;
  border-radius: 0.5rem;
  padding: 1.5rem;
  text-align: center;
}

.detailed-view-cta p {
  margin: 0 0 1.25rem;
  color: #4b5563;
}

.cta-buttons {
  display: flex;
  justify-content: center;
  gap: 1rem;
  flex-wrap: wrap;
}

.cta-button {
  display: inline-block;
  padding: 0.75rem 1.5rem;
  background-color: #4f46e5;
  color: white;
  border-radius: 0.375rem;
  font-weight: 500;
  transition: background-color 0.2s;
  text-decoration: none;
}

.cta-button:hover {
  background-color: #4338ca;
}

.empty-state {
  padding: 3rem;
  text-align: center;
  color: #6b7280;
  background-color: #f9fafb;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

@media (max-width: 768px) {
  .breakdown-sections {
    grid-template-columns: 1fr;
  }

  .cta-buttons {
    flex-direction: column;
    align-items: center;
  }

  .cta-button {
    width: 100%;
    max-width: 300px;
  }
}
</style>
