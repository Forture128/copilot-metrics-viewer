<template>
  <v-app>
    <v-main>
      <router-view />
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useStore } from 'vuex'
import { useRouter } from 'vue-router'
import ToastService from './services/ToastService'

const store = useStore()
const router = useRouter()

onMounted(async () => {
  try {
    // Check authentication status on application startup
    const isAuthenticated = await store.dispatch('auth/checkAuth')

    // If not authenticated and not on login page, redirect to login
    if (!isAuthenticated && router.currentRoute.value.path !== '/login') {
      router.push('/login')
    }
  } catch (error: any) {
    console.error('Auth check error:', error)

    // Check if error is related to token expiration
    if (
      error.response?.status === 401 ||
      (error.message && error.message.includes('Token validation error'))
    ) {
      ToastService.error('Your session has expired. Please login again.')
      store.dispatch('auth/handleTokenExpired')
    }
  }
})
</script>

<style>
/* Your global styles can go here if needed */
</style>
