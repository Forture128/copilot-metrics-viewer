<template>
  <div class="auth-layout">
    <div class="auth-container">
      <div class="auth-branding">
        <h1 class="app-name">Horus</h1>
        <p class="app-description">GitHub Copilot Metrics & RBAC Management</p>
      </div>

      <div class="auth-card">
        <slot />
      </div>

      <div class="auth-footer">
        <p>&copy; 2025 Horus | Privacy Policy | Terms of Service</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// No additional script needed for this layout
</script>

<style scoped>
.auth-layout {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background-color: #f5f5f5;
  background-image: linear-gradient(135deg, #f5f7fa 0%, #e4e8ef 100%);
}

.auth-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
  max-width: 480px;
  padding: 2rem;
}

.auth-branding {
  text-align: center;
  margin-bottom: 2rem;
}

.app-name {
  font-size: 2.5rem;
  font-weight: 700;
  margin: 0;
  color: #4f46e5;
}

.app-description {
  color: #6b7280;
  margin: 0.5rem 0 0;
}

.auth-card {
  width: 100%;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.auth-footer {
  margin-top: 2rem;
  text-align: center;
  color: #9ca3af;
  font-size: 0.875rem;
}
</style>
