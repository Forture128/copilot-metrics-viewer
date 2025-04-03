import { createApp } from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify'
import Toast from 'vue-toastification'
import './assets/globals.css' // shadcn-vue styles
import 'vue-toastification/dist/index.css'
import './assets/global.css' // your custom styles
import router from './router'
import store from './store'
import axios from 'axios'
import './plugins/chartjs' // Import Chart.js plugin for global registration
import { validateEnv } from './utils/envValidator'

// Validate environment variables before app initialization
try {
  validateEnv()
} catch (error) {
  console.error('Environment validation failed:', error)
  // Show error in the UI
  const errorDiv = document.createElement('div')
  errorDiv.style.cssText = `
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    background: #ff4444;
    color: white;
    padding: 10px;
    text-align: center;
    z-index: 9999;
  `
  errorDiv.textContent = error instanceof Error ? error.message : 'Environment validation failed'
  document.body.appendChild(errorDiv)
  throw error
}

// Set up axios interceptor for token expiration
axios.interceptors.response.use(
  response => response,
  error => {
    // Check if the error is related to token expiration
    if (error.response && error.response.status === 401) {
      const errorMessage = error.response.data?.message || ''
      if (
        errorMessage.includes('ExpiredSignature') ||
        errorMessage.includes('Token validation error')
      ) {
        // Dispatch token expiration handling
        store.dispatch('auth/handleTokenExpired')
      }
    }
    return Promise.reject(error)
  }
)

const app = createApp(App)

app.use(vuetify).use(store).use(Toast).use(router).mount('#app')
