<template>
  <AuthLayout>
    <div class="login-form-container">
      <h2 class="login-title">Login</h2>

      <div v-if="error" class="error-message">
        {{ error }}
      </div>

      <form @submit.prevent="handleLogin" class="login-form">
        <div class="form-group">
          <label for="username">Username</label>
          <input
            id="username"
            v-model="username"
            type="text"
            required
            placeholder="Enter your username"
            :disabled="loading"
            autocomplete="username"
          />
        </div>

        <div class="form-group">
          <label for="password">Password</label>
          <input
            id="password"
            v-model="password"
            type="password"
            required
            placeholder="Enter your password"
            :disabled="loading"
            autocomplete="current-password"
          />
        </div>

        <button type="submit" class="login-button" :disabled="loading || !username || !password">
          <span v-if="loading">Logging in...</span>
          <span v-else>Login</span>
        </button>
      </form>
    </div>
  </AuthLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useStore } from 'vuex'
import { useToast } from 'vue-toastification'
import AuthLayout from '@/layouts/AuthLayout.vue'

// State
const username = ref('')
const password = ref('')
const loading = ref(false)
const error = ref('')

// Hooks
const router = useRouter()
const store = useStore()
const toast = useToast()

// Check if already authenticated
onMounted(() => {
  if (store.getters['auth/isAuthenticated']) {
    router.push('/dashboard')
  }
})

// Methods
const handleLogin = async () => {
  if (!username.value || !password.value) {
    error.value = 'Please enter both username and password'
    return
  }

  error.value = ''
  loading.value = true

  try {
    // The password is only sent over HTTPS and not stored in client memory after login
    const success = await store.dispatch('auth/login', {
      username: username.value,
      password: password.value
    })

    if (success) {
      // Clear sensitive data from memory
      password.value = ''

      toast.success('Login successful')

      // Redirect to dashboard
      router.push('/dashboard')
    } else {
      error.value = 'Invalid response from server'
    }
  } catch (err: any) {
    console.error('Login error:', err)
    error.value = err.response?.data?.message || 'Failed to login. Please try again.'
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-form-container {
  padding: 2rem;
}

.login-title {
  margin-bottom: 1.5rem;
  font-size: 1.5rem;
  font-weight: 600;
  text-align: center;
  color: #333;
}

.login-form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  font-size: 0.875rem;
  font-weight: 500;
  color: #555;
}

.form-group input {
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
  transition: border-color 0.2s;
}

.form-group input:focus {
  outline: none;
  border-color: #4f46e5;
}

.login-button {
  margin-top: 1rem;
  padding: 0.75rem;
  background-color: #4f46e5;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.login-button:hover:not(:disabled) {
  background-color: #4338ca;
}

.login-button:disabled {
  background-color: #a5a5a5;
  cursor: not-allowed;
}

.error-message {
  margin-bottom: 1rem;
  padding: 0.75rem;
  background-color: #fee2e2;
  color: #b91c1c;
  border-radius: 4px;
  font-size: 0.875rem;
}
</style>
