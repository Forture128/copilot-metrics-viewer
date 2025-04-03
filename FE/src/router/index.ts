import { createRouter, createWebHistory } from 'vue-router'
import LoginView from '../views/auth/LoginView.vue'
import DashboardView from '../views/dashboard/DashboardView.vue'
import DoraMetricsView from '../views/metrics/DoraMetricsView.vue'
import horusService from '../services/HorusService'
import store from '../store'

/**
 * Defines the application routes.
 *
 * @note
 * - The root path ('/') redirects to '/dashboard'.
 * - The '/login' path loads the `LoginView`.
 * - The '/dashboard' path loads the `DashboardView`.
 * - The '/metrics/dora' path loads the `DoraMetricsView`.
 * - Additional routes for organization, teams, departments, analyses, and RBAC.
 */
const routes = [
  {
    path: '/',
    redirect: '/dashboard'
  },
  {
    path: '/login',
    name: 'Login',
    component: LoginView,
    meta: { requiresAuth: false }
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: DashboardView,
    meta: { requiresAuth: true }
  },
  // Metrics routes
  {
    path: '/metrics/dora',
    name: 'DORA Metrics',
    component: DoraMetricsView,
    meta: { requiresAuth: true }
  },
  {
    path: '/metrics/developer',
    name: 'Developer Metrics',
    component: () => import('../views/metrics/DeveloperMetricsView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/metrics/collaboration',
    name: 'Collaboration Quality',
    component: () => import('../views/metrics/CollaborationMetricsView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/metrics/delivery',
    redirect: '/metrics/dora',
    meta: { requiresAuth: true }
  },
  // Organization routes
  {
    path: '/organization',
    name: 'Organization',
    component: () => import('../views/organization/OrganizationView.vue'),
    meta: { requiresAuth: true }
  },
  // Teams route
  {
    path: '/teams',
    name: 'Teams',
    component: () => import('../views/teams/TeamsView.vue'),
    meta: { requiresAuth: true }
  },
  // Departments route
  {
    path: '/departments',
    name: 'Departments',
    component: () => import('../views/departments/DepartmentMetricsView.vue'),
    meta: { requiresAuth: true }
  },
  // Analysis routes
  {
    path: '/analysis/language',
    name: 'Language Analysis',
    component: () => import('../views/analysis/LanguageAnalysisView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/analysis/editor',
    name: 'Editor Analysis',
    component: () => import('../views/analysis/EditorAnalysisView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/analysis/chat',
    name: 'Copilot Chat',
    component: () => import('../views/analysis/ChatAnalysisView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/analysis/seat',
    name: 'Seat Analysis',
    component: () => import('../views/analysis/SeatsAnalysisView.vue'),
    meta: { requiresAuth: true }
  },
  // RBAC routes
  {
    path: '/rbac/users',
    name: 'User Management',
    component: () => import('../views/rbac/UsersView.vue'),
    meta: { requiresAuth: true, requiresAdmin: true }
  },
  {
    path: '/rbac/roles',
    name: 'Role Management',
    component: () => import('../views/rbac/RolesView.vue'),
    meta: { requiresAuth: true, requiresAdmin: true }
  },
  {
    path: '/rbac/departments',
    name: 'Department Management',
    component: () => import('../views/rbac/DepartmentRBACView.vue'),
    meta: { requiresAuth: true, requiresAdmin: true }
  },
  // API Response route
  {
    path: '/api-response',
    name: 'API Response',
    component: () => import('../views/analysis/ApiResponseView.vue'),
    meta: { requiresAuth: true, requiresAdmin: true }
  },
  {
    path: '/metrics/teams',
    name: 'Team Metrics',
    component: () => import('../views/metrics/TeamMetricsView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/metrics/overview',
    name: 'Metrics Overview',
    component: () => import('../views/metrics/MetricsOverviewView.vue'),
    meta: { requiresAuth: true }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

/**
 * Navigation guard to check if the route requires authentication and admin privileges
 */
router.beforeEach((to, from, next) => {
  // Check if the route requires authentication
  const requiresAuth = to.matched.some(record => record.meta.requiresAuth !== false)
  const requiresAdmin = to.matched.some(record => record.meta.requiresAdmin === true)

  // Check authentication first
  if (requiresAuth && !horusService.isAuthenticated()) {
    // Try to load auth from storage first
    if (!horusService.loadAuthFromStorage()) {
      next({ name: 'Login' })
      return
    }
  }

  // Check admin privileges if needed
  if (requiresAdmin) {
    const userRoles = store.getters['auth/userRoles'] || null
    console.log('userRoles', userRoles)
    if (!userRoles.includes('admin')) {
      // Redirect to dashboard if user doesn't have admin role
      next({ name: 'Dashboard' })
      return
    }
  }

  // Otherwise proceed
  next()
})

export default router
