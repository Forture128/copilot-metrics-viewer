<template>
  <AppLayout>
    <div class="organization-view">
      <div class="view-header">
        <h1 class="view-title">Organizations</h1>
        <p class="view-description">View and manage your organizations.</p>
      </div>

      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner" />
        <span>Loading organizations...</span>
      </div>

      <div v-else class="organization-content">
        <!-- Organizations List -->
        <div class="organizations-list">
          <div class="list-header">
            <h2>Organizations</h2>
            <button class="create-button" @click="showCreateModal = true">
              <span class="button-icon">➕</span>
              Create Organization
            </button>
          </div>

          <div v-if="!organizations || organizations.length === 0" class="empty-state">
            No organizations found
          </div>

          <div v-else class="organizations-grid">
            <div
              v-for="org in organizations"
              :key="org.id"
              class="organization-card"
              :class="{ selected: selectedOrg?.id === org.id }"
              @click="selectOrganization(org)"
            >
              <div class="card-header">
                <h3>{{ org.name }}</h3>
                <div class="card-actions">
                  <button class="edit-button" @click.stop="editOrganization(org)">
                    <span class="button-icon">✏️</span>
                  </button>
                  <button class="delete-button" @click.stop="confirmDelete(org)">
                    <span class="button-icon">🗑️</span>
                  </button>
                </div>
              </div>
              <p class="card-description">{{ org.description || 'No description provided' }}</p>
              <div class="card-footer">
                <span class="created-date">Created: {{ formatDate(org.created_at) }}</span>
                <span class="updated-date">Updated: {{ formatDate(org.updated_at) }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Selected Organization Details -->
        <div v-if="selectedOrg" class="organization-details">
          <div class="details-header">
            <h2>Organization Details</h2>
            <button class="close-button" @click="selectedOrg = null">×</button>
          </div>
          <div class="details-grid">
            <div class="detail-item">
              <label>Name</label>
              <span>{{ selectedOrg.name }}</span>
            </div>
            <div class="detail-item">
              <label>ID</label>
              <span>{{ selectedOrg.id }}</span>
            </div>
            <div class="detail-item">
              <label>Created At</label>
              <span>{{ formatDate(selectedOrg.created_at) }}</span>
            </div>
            <div class="detail-item">
              <label>Last Updated</label>
              <span>{{ formatDate(selectedOrg.updated_at) }}</span>
            </div>
          </div>
          <div class="description-section">
            <label>Description</label>
            <p>{{ selectedOrg.description || 'No description provided' }}</p>
          </div>

          <!-- Organization Configuration -->
          <div class="organization-config">
            <div class="config-header">
              <h3>Configuration</h3>
              <button
                v-if="selectedOrg"
                class="create-config-button"
                @click="showCreateConfigModal = true"
              >
                <span class="button-icon">➕</span>
                Add Configuration
              </button>
            </div>
            <div v-if="!configs || configs.length === 0" class="config-empty-state">
              <div class="empty-icon">⚙️</div>
              <h4>No Configuration Found</h4>
              <p>This organization doesn't have any configuration settings yet.</p>
              <button class="create-first-config-button" @click="showCreateConfigModal = true">
                <span class="button-icon">➕</span>
                Create First Configuration
              </button>
            </div>
            <div v-else class="config-grid">
              <div v-for="config in configs" :key="config.id" class="config-item">
                <div class="config-header">
                  <div class="config-key">
                    <h4>{{ config.config_key }}</h4>
                    <span class="config-id">#{{ config.id }}</span>
                  </div>
                  <button class="edit-button" @click="editConfig(config)">
                    <span class="button-icon">✏️</span>
                  </button>
                </div>
                <p class="config-value">{{ config.config_value }}</p>
                <p class="config-date">Updated: {{ formatDate(config.updated_at) }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Create Organization Modal -->
    <div v-if="showCreateModal" class="modal-overlay" @click="showCreateModal = false">
      <div class="modal-content" @click.stop>
        <h2>Create Organization</h2>
        <form @submit.prevent="createOrganization">
          <div class="form-group">
            <label for="org-name">Name</label>
            <input
              id="org-name"
              v-model="newOrg.name"
              type="text"
              required
              placeholder="Enter organization name"
            />
          </div>
          <div class="form-group">
            <label for="org-description">Description</label>
            <textarea
              id="org-description"
              v-model="newOrg.description"
              placeholder="Enter organization description"
            />
          </div>
          <div class="modal-actions">
            <button type="button" class="cancel-button" @click="showCreateModal = false">
              Cancel
            </button>
            <button type="submit" class="submit-button">Create Organization</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Edit Organization Modal -->
    <div v-if="showEditModal && editingOrg" class="modal-overlay" @click="showEditModal = false">
      <div class="modal-content" @click.stop>
        <h2>Edit Organization</h2>
        <form @submit.prevent="updateOrganization">
          <div class="form-group">
            <label for="edit-org-name">Name</label>
            <input
              id="edit-org-name"
              v-model="editingOrg.name"
              type="text"
              required
              placeholder="Enter organization name"
            />
          </div>
          <div class="form-group">
            <label for="edit-org-description">Description</label>
            <textarea
              id="edit-org-description"
              v-model="editingOrg.description"
              placeholder="Enter organization description"
            />
          </div>
          <div class="modal-actions">
            <button type="button" class="cancel-button" @click="showEditModal = false">
              Cancel
            </button>
            <button type="submit" class="submit-button">Save Changes</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Edit Config Modal -->
    <div
      v-if="showConfigModal && editingConfig"
      class="modal-overlay"
      @click="showConfigModal = false"
    >
      <div class="modal-content" @click.stop>
        <h2>Edit Configuration</h2>
        <form @submit.prevent="saveConfig">
          <div class="form-group">
            <label for="config-key">Key</label>
            <input
              id="config-key"
              v-model="editingConfig.config_key"
              type="text"
              required
              :disabled="true"
            />
          </div>
          <div class="form-group">
            <label for="config-value">Value</label>
            <input
              id="config-value"
              v-model="editingConfig.config_value"
              type="text"
              required
              placeholder="Enter configuration value"
            />
          </div>
          <div class="modal-actions">
            <button type="button" class="cancel-button" @click="showConfigModal = false">
              Cancel
            </button>
            <button type="submit" class="submit-button">Save Changes</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Create Config Modal -->
    <div v-if="showCreateConfigModal" class="modal-overlay" @click="showCreateConfigModal = false">
      <div class="modal-content" @click.stop>
        <h2>Create Configuration</h2>
        <form @submit.prevent="createConfig">
          <div class="form-group">
            <label for="config-key">Key</label>
            <input
              id="config-key"
              v-model="newConfig.config_key"
              type="text"
              required
              placeholder="Enter configuration key"
            />
          </div>
          <div class="form-group">
            <label for="new-config-value">Value</label>
            <input
              id="new-config-value"
              v-model="newConfig.config_value"
              type="text"
              required
              placeholder="Enter configuration value"
            />
          </div>
          <div class="modal-actions">
            <button type="button" class="cancel-button" @click="showCreateConfigModal = false">
              Cancel
            </button>
            <button type="submit" class="submit-button">Create Configuration</button>
          </div>
        </form>
      </div>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import AppLayout from '@/layouts/AppLayout.vue'
import horusService from '@/services/HorusService'
import ToastService from '@/services/ToastService'
import type {
  OrganizationResponse,
  OrganizationConfigResponse,
  CreateOrganizationRequest,
  CreateOrganizationConfigRequest,
  UpdateOrganizationConfigRequest
} from '@/types/horus-api.types'

// State
const isLoading = ref(false)
const error = ref<string | null>(null)
const organizations = ref<OrganizationResponse[]>([])
const selectedOrg = ref<OrganizationResponse | null>(null)
const configs = ref<OrganizationConfigResponse[]>([])

// Modal state
const showCreateModal = ref(false)
const showEditModal = ref(false)
const showConfigModal = ref(false)
const showCreateConfigModal = ref(false)
const editingOrg = ref<OrganizationResponse | null>(null)
const editingConfig = ref<OrganizationConfigResponse | null>(null)
const newOrg = ref<CreateOrganizationRequest>({
  name: '',
  description: ''
})
const newConfig = ref<CreateOrganizationConfigRequest>({
  config_key: '',
  config_value: ''
})

// Methods
const fetchOrganizations = async () => {
  isLoading.value = true
  error.value = null
  try {
    const response = await horusService.listOrganizations()
    organizations.value = response.organizations || []
  } catch (err) {
    console.error('Error fetching organizations:', err)
    ToastService.error(err instanceof Error ? err.message : 'Failed to fetch organizations')
  } finally {
    isLoading.value = false
  }
}

const selectOrganization = async (org: OrganizationResponse) => {
  selectedOrg.value = org
  await fetchConfigs(org.id)
}

const fetchConfigs = async (orgId: number) => {
  try {
    const response = await horusService.listOrganizationConfigs(orgId)
    configs.value = response.configs || []
  } catch (err) {
    console.error('Failed to fetch configs:', err)
    configs.value = []
    ToastService.error(err instanceof Error ? err.message : 'Failed to fetch configurations')
  }
}

const createOrganization = async () => {
  try {
    await horusService.createOrganization(newOrg.value)
    showCreateModal.value = false
    newOrg.value = { name: '', description: '' }
    await fetchOrganizations()
    ToastService.success('Organization created successfully')
  } catch (err) {
    ToastService.error(err instanceof Error ? err.message : 'Failed to create organization')
  }
}

const editOrganization = (org: OrganizationResponse) => {
  editingOrg.value = { ...org }
  showEditModal.value = true
}

const updateOrganization = async () => {
  if (!editingOrg.value) return

  try {
    await horusService.updateOrganization(editingOrg.value.id, {
      name: editingOrg.value.name,
      description: editingOrg.value.description
    })
    showEditModal.value = false
    await fetchOrganizations()
    ToastService.success('Organization updated successfully')
  } catch (err) {
    ToastService.error(err instanceof Error ? err.message : 'Failed to update organization')
  }
}

const confirmDelete = async (org: OrganizationResponse) => {
  if (!confirm('Are you sure you want to delete this organization?')) return

  try {
    await horusService.deleteOrganization(org.id)
    await fetchOrganizations()
    ToastService.success('Organization deleted successfully')
  } catch (err) {
    ToastService.error(err instanceof Error ? err.message : 'Failed to delete organization')
  }
}

const editConfig = (config: OrganizationConfigResponse) => {
  editingConfig.value = { ...config }
  showConfigModal.value = true
}

const saveConfig = async () => {
  if (!editingConfig.value || !selectedOrg.value) return

  try {
    await horusService.updateOrganizationConfig(selectedOrg.value.id, editingConfig.value.id, {
      config_value: editingConfig.value.config_value
    })
    showConfigModal.value = false
    await fetchConfigs(selectedOrg.value.id)
    ToastService.success('Configuration updated successfully')
  } catch (err) {
    console.error('Failed to update config:', err)
    ToastService.error(err instanceof Error ? err.message : 'Failed to update configuration')
    // Don't close modal on error so user can try again
  }
}

const createConfig = async () => {
  if (!selectedOrg.value) return

  try {
    await horusService.createOrganizationConfig(selectedOrg.value.id, newConfig.value)
    showCreateConfigModal.value = false
    newConfig.value = { config_key: '', config_value: '' }
    await fetchConfigs(selectedOrg.value.id)
    ToastService.success('Configuration created successfully')
  } catch (err) {
    console.error('Failed to create config:', err)
    ToastService.error(err instanceof Error ? err.message : 'Failed to create configuration')
    // Don't close modal on error so user can try again
  }
}

const formatDate = (dateString: string): string => {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

// Lifecycle hooks
onMounted(() => {
  fetchOrganizations()
})
</script>

<style scoped>
.organization-view {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.view-header {
  margin-bottom: 1rem;
}

.view-title {
  font-size: 1.75rem;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.view-description {
  color: #6b7280;
  margin: 0.5rem 0 0;
  max-width: 800px;
  line-height: 1.5;
}

.organization-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
}

.organizations-list {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.list-header h2 {
  margin: 0;
  font-size: 1.25rem;
  color: #333;
}

.create-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: #4f46e5;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
}

.create-button:hover {
  background: #4338ca;
}

.organizations-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1rem;
}

.organization-card {
  background: #f9fafb;
  padding: 1rem;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.organization-card:hover {
  background: #f3f4f6;
}

.organization-card.selected {
  background: #eef2ff;
  border: 2px solid #4f46e5;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
}

.card-header h3 {
  margin: 0;
  font-size: 1rem;
  color: #333;
}

.card-actions {
  display: flex;
  gap: 0.5rem;
}

.edit-button,
.delete-button {
  background: none;
  border: none;
  padding: 0.25rem;
  cursor: pointer;
}

.edit-button {
  color: #4f46e5;
}

.delete-button {
  color: #ef4444;
}

.card-description {
  margin: 0 0 0.5rem;
  color: #6b7280;
  font-size: 0.875rem;
}

.card-footer {
  display: flex;
  justify-content: space-between;
  font-size: 0.75rem;
  color: #9ca3af;
}

.organization-details {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.details-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.details-header h2 {
  margin: 0;
  font-size: 1.25rem;
  color: #333;
}

.close-button {
  background: none;
  border: none;
  font-size: 1.5rem;
  color: #6b7280;
  cursor: pointer;
  padding: 0.25rem;
}

.details-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1rem;
}

.detail-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.detail-item label {
  font-size: 0.875rem;
  color: #6b7280;
}

.detail-item span {
  font-size: 1rem;
  color: #333;
}

.description-section {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 1.5rem;
}

.description-section label {
  font-size: 0.875rem;
  color: #6b7280;
}

.description-section p {
  margin: 0;
  color: #333;
  line-height: 1.5;
}

.organization-config {
  border-top: 1px solid #e5e7eb;
  padding-top: 1.5rem;
}

.organization-config h3 {
  margin: 0 0 1rem;
  font-size: 1.125rem;
  color: #333;
}

.config-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}

.config-item {
  background: #f9fafb;
  padding: 1rem;
  border-radius: 6px;
}

.config-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.create-config-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: #4f46e5;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
}

.create-config-button:hover {
  background: #4338ca;
}

.create-first-config-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  background: #4f46e5;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
  margin: 1rem auto 0;
}

.create-first-config-button:hover {
  background: #4338ca;
}

.config-key {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.config-id {
  font-size: 0.75rem;
  color: #6b7280;
}

.config-date {
  margin-top: 0.5rem;
  font-size: 0.75rem;
  color: #9ca3af;
}

.config-value {
  margin: 0;
  color: #6b7280;
  font-size: 0.875rem;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  width: 100%;
  max-width: 500px;
}

.modal-content h2 {
  margin: 0 0 1.5rem;
  font-size: 1.25rem;
  color: #333;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  color: #4b5563;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  font-size: 1rem;
}

.form-group textarea {
  min-height: 100px;
  resize: vertical;
}

.form-group input:disabled {
  background: #f3f4f6;
  cursor: not-allowed;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 1.5rem;
}

.cancel-button,
.submit-button {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  font-size: 0.875rem;
  cursor: pointer;
}

.cancel-button {
  background: #f3f4f6;
  border: none;
  color: #4b5563;
}

.submit-button {
  background: #4f46e5;
  border: none;
  color: white;
}

.submit-button:hover {
  background: #4338ca;
}

.loading-state,
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  background: white;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  text-align: center;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 3px solid rgba(79, 70, 229, 0.2);
  border-radius: 50%;
  border-top-color: #4f46e5;
  animation: spin 1s ease-in-out infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.error-state {
  flex-direction: row;
  text-align: left;
  gap: 1rem;
  background-color: #fee2e2;
  color: #b91c1c;
}

.error-icon {
  font-size: 1.5rem;
}

.error-title {
  margin: 0 0 0.25rem;
  font-weight: 600;
}

.error-message {
  margin: 0;
  color: #b91c1c;
}

.empty-state {
  text-align: center;
  color: #6b7280;
  padding: 2rem;
}

.config-empty-state {
  text-align: center;
  padding: 2rem;
  background: #f9fafb;
  border-radius: 8px;
  border: 2px dashed #e5e7eb;
}

.config-empty-state .empty-icon {
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.config-empty-state h4 {
  margin: 0 0 0.5rem;
  color: #374151;
  font-size: 1.125rem;
}

.config-empty-state p {
  margin: 0;
  color: #6b7280;
  font-size: 0.875rem;
  line-height: 1.5;
}

.config-empty-state .hint {
  margin-top: 0.5rem;
  color: #9ca3af;
  font-size: 0.75rem;
}
</style>
