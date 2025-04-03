<template>
  <div class="team-metrics-viewer">
    <h2 class="title">Team Performance Metrics</h2>

    <div v-if="!teamData || !teamData.teams || teamData.teams.length === 0" class="empty-state">
      No team data available.
    </div>

    <div v-else class="team-data">
      <!-- Teams Comparison Table -->
      <div class="teams-table-container">
        <table class="teams-table">
          <thead>
            <tr>
              <th>Team</th>
              <th>Active Developers</th>
              <th>Suggestions</th>
              <th>Acceptance Rate</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="team in teamData.teams" :key="team.id">
              <td class="team-name">{{ team.name }}</td>
              <td>{{ team.activeDevelopers }}</td>
              <td>{{ team.totalSuggestions.toLocaleString() }}</td>
              <td>
                <div class="progress-wrapper">
                  <div class="progress-bar" :style="{ width: `${team.acceptanceRate * 100}%` }" />
                  <span class="progress-value">{{ (team.acceptanceRate * 100).toFixed(1) }}%</span>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Summary Cards -->
      <div class="summary-cards">
        <div class="summary-card">
          <div class="card-title">Top Performing Team</div>
          <div class="card-content">
            <div class="top-team-name">{{ getTopTeam().name }}</div>
            <div class="top-team-stats">
              <div class="stat">
                <span class="stat-label">Acceptance Rate:</span>
                <span class="stat-value"
                  >{{ (getTopTeam().acceptanceRate * 100).toFixed(1) }}%</span
                >
              </div>
              <div class="stat">
                <span class="stat-label">Suggestions:</span>
                <span class="stat-value">{{ getTopTeam().totalSuggestions.toLocaleString() }}</span>
              </div>
            </div>
          </div>
        </div>

        <div class="summary-card">
          <div class="card-title">Total Teams</div>
          <div class="card-content">
            <div class="summary-value">{{ teamData.teams.length }}</div>
          </div>
        </div>

        <div class="summary-card">
          <div class="card-title">Total Developers</div>
          <div class="card-content">
            <div class="summary-value">{{ getTotalDevelopers() }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Team {
  id: string
  name: string
  acceptanceRate: number
  totalSuggestions: number
  activeDevelopers: number
}

interface TeamData {
  teams: Team[]
}

// Props
const props = defineProps<{
  teamData: TeamData
}>()

// Methods
const getTopTeam = (): Team => {
  if (!props.teamData || !props.teamData.teams || props.teamData.teams.length === 0) {
    return {
      id: '',
      name: 'No teams',
      acceptanceRate: 0,
      totalSuggestions: 0,
      activeDevelopers: 0
    }
  }

  // Get team with highest acceptance rate
  return [...props.teamData.teams].sort((a, b) => b.acceptanceRate - a.acceptanceRate)[0]
}

const getTotalDevelopers = (): number => {
  if (!props.teamData || !props.teamData.teams) return 0

  return props.teamData.teams.reduce((total, team) => total + team.activeDevelopers, 0)
}
</script>

<style scoped>
.team-metrics-viewer {
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

.team-data {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.teams-table-container {
  background-color: white;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.teams-table {
  width: 100%;
  border-collapse: collapse;
}

.teams-table th,
.teams-table td {
  padding: 1rem;
  text-align: left;
  border-bottom: 1px solid #e5e7eb;
}

.teams-table th {
  background-color: #f9fafb;
  font-weight: 600;
  color: #374151;
}

.teams-table tr:last-child td {
  border-bottom: none;
}

.teams-table tbody tr:hover {
  background-color: #f9fafb;
}

.team-name {
  font-weight: 500;
  color: #111827;
}

.progress-wrapper {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.progress-bar {
  flex-grow: 1;
  height: 8px;
  background-color: #4f46e5;
  border-radius: 4px;
}

.progress-value {
  min-width: 48px;
  font-weight: 500;
  color: #4f46e5;
}

.summary-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.summary-card {
  background-color: white;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  padding: 1.5rem;
  transition:
    transform 0.2s,
    box-shadow 0.2s;
}

.summary-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
}

.card-title {
  font-size: 0.875rem;
  font-weight: 500;
  color: #6b7280;
  margin-bottom: 0.75rem;
}

.summary-value {
  font-size: 2rem;
  font-weight: 700;
  color: #4f46e5;
}

.top-team-name {
  font-size: 1.25rem;
  font-weight: 600;
  color: #111827;
  margin-bottom: 0.5rem;
}

.top-team-stats {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.stat {
  display: flex;
  justify-content: space-between;
  font-size: 0.875rem;
}

.stat-label {
  color: #6b7280;
}

.stat-value {
  font-weight: 500;
  color: #111827;
}

.empty-state {
  padding: 2rem;
  text-align: center;
  color: #6b7280;
  background-color: #f9fafb;
  border-radius: 0.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}
</style>
