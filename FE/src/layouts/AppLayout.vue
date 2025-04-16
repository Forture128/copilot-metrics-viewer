<template>
  <div class="app-layout">
    <!-- Header -->
    <header class="app-header">
      <div class="header-left">
        <button class="github-button">
          <span class="github-icon">
            <!-- Simple GitHub icon -->
            <svg
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path
                d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54
              6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0
              0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42
              3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"
              />
            </svg>
          </span>
        </button>
        <h1 class="app-title">Horus | {{ currentPage }}</h1>
      </div>
      <div class="header-right">
        <div class="user-info">
          <span v-if="user" class="username">{{ user.username }}</span>
          <button class="logout-button" @click="handleLogout">Logout</button>
        </div>
      </div>
    </header>

    <!-- Tab navigation -->
    <div class="tab-navigation">
      <nav class="tabs">
        <button
          v-for="tab in navTabs"
          :key="tab.name"
          class="tab-button"
          :class="{ active: isActiveTab(tab.path) }"
          @click="navigateTo(tab.path)"
        >
          <span class="tab-icon">{{ tab.icon }}</span>
          <span class="tab-label">{{ tab.name }}</span>
        </button>
      </nav>
    </div>

    <div class="app-content">
      <aside class="app-sidebar">
        <div class="sidebar-header">
          <h2>Navigation</h2>
        </div>

        <div class="sidebar-menu">
          <router-link to="/dashboard" class="menu-item">
            <span class="menu-icon">📊</span>
            <span class="menu-text">Dashboard</span>
          </router-link>

          <!-- Metrics Section -->
          <div class="menu-section">
            <h3 class="section-title">Metrics</h3>

            <router-link to="/metrics/developer" class="menu-item">
              <span class="menu-icon">👨‍💻</span>
              <span class="menu-text">Developer Metrics</span>
            </router-link>

            <router-link to="/metrics/dora" class="menu-item">
              <span class="menu-icon">📈</span>
              <span class="menu-text">DORA Metrics</span>
            </router-link>

            <router-link to="/metrics/collaboration" class="menu-item">
              <span class="menu-icon">🤝</span>
              <span class="menu-text">Collaboration Quality</span>
            </router-link>

            <router-link to="/metrics/delivery" class="menu-item">
              <span class="menu-icon">🚀</span>
              <span class="menu-text">Delivery Insights</span>
            </router-link>

            <router-link to="/metrics/teams" class="menu-item">
              <span class="menu-icon">👥</span>
              <span class="menu-text">Team Metrics</span>
            </router-link>
          </div>

          <!-- Data Analysis Section -->
          <div class="menu-section">
            <h3 class="section-title">Data Analysis</h3>

            <router-link to="/analysis/language" class="menu-item">
              <span class="menu-icon">🔤</span>
              <span class="menu-text">Languages</span>
            </router-link>

            <router-link to="/analysis/editor" class="menu-item">
              <span class="menu-icon">✏️</span>
              <span class="menu-text">Editors</span>
            </router-link>

            <router-link to="/analysis/chat" class="menu-item">
              <span class="menu-icon">💬</span>
              <span class="menu-text">Copilot Chat</span>
            </router-link>

            <router-link to="/analysis/seat" class="menu-item">
              <span class="menu-icon">👤</span>
              <span class="menu-text">Seat Analysis</span>
            </router-link>
          </div>

          <!-- Admin Section - visible only to admins -->
          <div v-if="hasAdminPermission" class="menu-section">
            <h3 class="section-title">Administration</h3>

            <router-link to="/rbac/users" class="menu-item">
              <span class="menu-icon">👥</span>
              <span class="menu-text">User Management</span>
            </router-link>

            <router-link to="/rbac/roles" class="menu-item">
              <span class="menu-icon">🔑</span>
              <span class="menu-text">Role Management</span>
            </router-link>

            <router-link to="/rbac/departments" class="menu-item">
              <span class="menu-icon">🏢</span>
              <span class="menu-text">Department Management</span>
            </router-link>

            <router-link to="/api-response" class="menu-item">
              <span class="menu-icon">🔍</span>
              <span class="menu-text">API Response</span>
            </router-link>
          </div>
        </div>
      </aside>

      <main class="app-main">
        <!-- Error Alert -->
        <div v-if="apiError" class="error-alert">
          <div class="error-icon">⚠️</div>
          <div class="error-content">
            <h3 class="error-title">Error</h3>
            <p class="error-message">{{ apiError }}</p>
          </div>
        </div>

        <!-- Loading Progress -->
        <div v-if="isLoading" class="loading-container">
          <div class="loading-bar" />
          <p class="loading-text">Loading data...</p>
        </div>

        <!-- Content -->
        <slot v-if="!isLoading" />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useStore } from 'vuex'
import { useToast } from 'vue-toastification'
// import config from '@/config'

// Hooks
const router = useRouter()
const route = useRoute()
const store = useStore()
const toast = useToast()

// State
const isLoading = ref(false)

// Computed
const user = computed(() => store.getters['auth/user'])
const apiError = computed(() => store.state.CopilotUsage?.apiError || null)
const hasAdminPermission = computed(() => {
  const userRoles = store.getters['auth/userRoles'] || []
  console.log('userRoles', userRoles)
  return userRoles.includes('admin')
})

const currentPage = computed(() => {
  // Get page name from current route
  const routeName = route.name || 'Dashboard'

  // Format it nicely
  if (typeof routeName === 'string') {
    return routeName.replace(/-/g, ' ')
  }
  return 'Dashboard'
})

// Navigation tabs based on MainComponent
const navTabs = [
  { name: 'Dashboard', path: '/dashboard', icon: '📊' },
  { name: 'Organization', path: '/organization', icon: '🏢' },
  { name: 'Teams', path: '/teams', icon: '👥' },
  { name: 'Departments', path: '/departments', icon: '🏛️' },
  { name: 'DORA Dashboard', path: '/metrics/dora', icon: '📈' }
  // { name: 'Language Analysis', path: '/analysis/language', icon: '🔤' },
  // { name: 'Editor Analysis', path: '/analysis/editor', icon: '✏️' },
  // { name: 'Copilot Chat', path: '/analysis/chat', icon: '💬' },
  // { name: 'Seat Analysis', path: '/analysis/seat', icon: '👤' },
  // { name: 'User Management', path: '/rbac/users', icon: '👥' },
  // { name: 'Role Management', path: '/rbac/roles', icon: '🔑' },
  // { name: 'Department Management', path: '/rbac/departments', icon: '🏢' }
]

// Methods
const handleLogout = () => {
  store.dispatch('auth/logout')
  toast.info('You have been logged out')
  router.push('/login')
}

const navigateTo = (path: string) => {
  router.push(path)
}

const isActiveTab = (path: string) => {
  return route.path === path || (path !== '/dashboard' && route.path.startsWith(path))
}

const hasPermission = (permission: string) => {
  const userRoles = store.getters['auth/userRoles'] || []

  // This is a simplified implementation
  if (permission === 'user_management' && userRoles.includes('admin')) {
    return true
  }

  if (permission === 'role_management' && userRoles.includes('admin')) {
    return true
  }

  if (
    permission === 'department_management' &&
    (userRoles.includes('admin') || userRoles.includes('department_manager'))
  ) {
    return true
  }

  return false
}
</script>

<style scoped>
.app-layout {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  background-color: #f5f7fa;
}

/* Header styles */
.app-header {
  background-color: #ffffff;
  color: #333;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 1.5rem;
  height: 64px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  border-bottom: 1px solid #e5e7eb;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.github-button {
  background: none;
  border: none;
  color: #333;
  cursor: pointer;
  padding: 0.5rem;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.github-button:hover {
  background-color: #f3f4f6;
}

.app-title {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.username {
  font-weight: 500;
}

.logout-button {
  background-color: #4f46e5;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 0.5rem 1rem;
  cursor: pointer;
  font-size: 0.875rem;
  transition: background-color 0.2s;
}

.logout-button:hover {
  background-color: #4338ca;
}

/* Tab navigation */
.tab-navigation {
  background-color: #ffffff;
  border-bottom: 1px solid #e5e7eb;
}

.tabs {
  display: flex;
  overflow-x: auto;
  padding: 0 1.5rem;
}

.tab-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.25rem;
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  font-size: 0.875rem;
  font-weight: 500;
  color: #6b7280;
  cursor: pointer;
  transition:
    color 0.2s,
    border-color 0.2s;
  white-space: nowrap;
}

.tab-button:hover {
  color: #4f46e5;
}

.tab-button.active {
  color: #4f46e5;
  border-bottom-color: #4f46e5;
}

.tab-icon {
  font-size: 1rem;
}

/* Content area */
.app-content {
  display: flex;
  flex: 1;
}

.app-sidebar {
  width: 260px;
  background-color: #ffffff;
  border-right: 1px solid #e5e7eb;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
}

.sidebar-header {
  padding: 1.25rem 1.5rem;
  border-bottom: 1px solid #e5e7eb;
}

.sidebar-header h2 {
  margin: 0;
  font-size: 0.875rem;
  font-weight: 600;
  color: #6b7280;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.sidebar-menu {
  padding: 1rem 0;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.menu-section {
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid #f3f4f6;
}

.section-title {
  padding: 0 1.5rem;
  margin: 0 0 0.5rem;
  font-size: 0.75rem;
  font-weight: 600;
  color: #9ca3af;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.menu-item {
  display: flex;
  align-items: center;
  padding: 0.625rem 1.5rem;
  color: #4b5563;
  text-decoration: none;
  transition:
    background-color 0.2s,
    color 0.2s;
  border-left: 3px solid transparent;
}

.menu-item:hover {
  background-color: #f3f4f6;
  color: #4f46e5;
}

.menu-item.router-link-active {
  background-color: #f3f4f6;
  color: #4f46e5;
  border-left-color: #4f46e5;
}

.menu-icon {
  margin-right: 0.75rem;
  font-size: 1.25rem;
}

.app-main {
  flex: 1;
  padding: 1.5rem;
  overflow-y: auto;
}

/* Error alert */
.error-alert {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  padding: 1rem;
  background-color: #fee2e2;
  border-radius: 4px;
  margin-bottom: 1.5rem;
}

.error-icon {
  font-size: 1.25rem;
}

.error-title {
  margin: 0 0 0.25rem;
  font-size: 1rem;
  font-weight: 600;
  color: #b91c1c;
}

.error-message {
  margin: 0;
  color: #b91c1c;
}

/* Loading indicator */
.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 2rem;
}

.loading-bar {
  width: 100%;
  height: 4px;
  background-color: #e5e7eb;
  border-radius: 2px;
  overflow: hidden;
  position: relative;
}

.loading-bar::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  width: 30%;
  background-color: #4f46e5;
  border-radius: 2px;
  animation: loading 1.5s infinite ease-in-out;
}

.loading-text {
  margin-top: 1rem;
  color: #6b7280;
}

@keyframes loading {
  0% {
    left: -30%;
  }
  100% {
    left: 100%;
  }
}
</style>
