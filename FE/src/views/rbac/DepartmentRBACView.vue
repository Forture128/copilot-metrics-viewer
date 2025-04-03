<template>
  <AppLayout>
    <div class="rbac-departments-view">
      <div class="view-header">
        <h1 class="view-title">Department Management</h1>
        <p class="view-description">Manage department settings and user assignments.</p>
        <button class="create-button" @click="showCreateDepartmentModal = true">
          <span class="button-icon">➕</span>
          Create Department
        </button>
      </div>

      <div v-if="isLoading" class="loading-state">
        <div class="loading-spinner" />
        <span>Loading department management data...</span>
      </div>

      <div v-else-if="error" class="error-state">
        <div class="error-icon">⚠️</div>
        <div class="error-content">
          <h3 class="error-title">Error Loading Data</h3>
          <p class="error-message">{{ error }}</p>
        </div>
      </div>

      <div v-else class="rbac-departments-content">
        <div v-if="departments.length === 0" class="empty-state">
          <p>No departments found. Create your first department to get started.</p>
        </div>

        <div v-else class="departments-grid">
          <div v-for="dept in departments" :key="dept.id" class="department-card">
            <div class="department-header">
              <h3 class="department-name">{{ dept.name }}</h3>
              <div class="department-actions">
                <button class="action-button" @click="editDepartment(dept)">
                  <span class="action-icon">✏️</span>
                </button>
                <button class="action-button" @click="manageDepartmentRoles(dept)">
                  <span class="action-icon">🔑</span>
                </button>
                <button class="action-button delete" @click="confirmDeleteDepartment(dept)">
                  <span class="action-icon">🗑️</span>
                </button>
              </div>
            </div>
            <p class="department-description">
              {{ dept.description || 'No description provided' }}
            </p>
            <div class="department-stats">
              <div class="stat-item">
                <span class="stat-label">Users:</span>
                <span class="stat-value">{{ dept.user_count || 0 }}</span>
              </div>
              <div class="stat-item">
                <span class="stat-label">Roles:</span>
                <span class="stat-value">{{ dept.role_count || 0 }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Create Department Modal -->
    <div
      v-if="showCreateDepartmentModal"
      class="modal-overlay"
      @click="showCreateDepartmentModal = false"
    >
      <div class="modal-content" @click.stop>
        <h2>Create New Department</h2>
        <form @submit.prevent="createDepartment">
          <div class="form-group">
            <label for="department-name">Department Name</label>
            <input
              id="department-name"
              v-model="newDepartment.name"
              type="text"
              required
              placeholder="Enter department name"
            />
          </div>
          <div class="form-group">
            <label for="department-description">Description</label>
            <textarea
              id="department-description"
              v-model="newDepartment.description"
              placeholder="Enter department description"
            />
          </div>
          <div class="modal-actions">
            <button type="button" class="cancel-button" @click="showCreateDepartmentModal = false">
              Cancel
            </button>
            <button type="submit" class="submit-button">Create Department</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Manage Department Roles Modal -->
    <div v-if="showRolesModal" class="modal-overlay" @click="showRolesModal = false">
      <div class="modal-content" @click.stop>
        <h2>Manage Department Roles - {{ selectedDepartment?.name }}</h2>
        <div class="roles-management">
          <div class="roles-list">
            <h3>Assigned Roles</h3>
            <div v-for="role in departmentRoles" :key="role.id" class="role-item">
              <span class="role-name">{{ role.name }}</span>
              <button class="remove-role" @click="removeRole(role.id)">Remove</button>
            </div>
          </div>
          <div class="assign-role">
            <h3>Assign New Role</h3>
            <select v-model="selectedRoleId">
              <option value="">Select a role</option>
              <option v-for="role in availableRoles" :key="role.id" :value="role.id">
                {{ role.name }}
              </option>
            </select>
            <button class="assign-button" :disabled="!selectedRoleId" @click="assignRole">
              Assign Role
            </button>
          </div>
        </div>
        <div class="modal-actions">
          <button class="close-button" @click="showRolesModal = false">Close</button>
        </div>
      </div>
    </div>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import AppLayout from '@/layouts/AppLayout.vue'
import horusService from '@/services/HorusService'
import type {
  DepartmentResponse,
  RoleResponse,
  DepartmentListResponse,
  RoleListResponse,
  DepartmentRoleListResponse,
  AssignDepartmentRoleRequest
} from '@/types/horus-api.types'

// State
const isLoading = ref(false)
const error = ref<string | null>(null)
const departments = ref<DepartmentResponse[]>([])
const roles = ref<RoleResponse[]>([])
const departmentRoles = ref<RoleResponse[]>([])
const availableRoles = ref<RoleResponse[]>([])

// Modal states
const showCreateDepartmentModal = ref(false)
const showRolesModal = ref(false)
const selectedDepartment = ref<DepartmentResponse | null>(null)
const selectedRoleId = ref<number | null>(null)

// Form data
const newDepartment = ref({
  name: '',
  description: ''
})

// Fetch departments
const fetchDepartments = async () => {
  isLoading.value = true
  error.value = null
  try {
    const response: DepartmentListResponse = await horusService.listDepartments()
    departments.value = response.items
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to fetch departments'
  } finally {
    isLoading.value = false
  }
}

// Fetch roles
const fetchRoles = async () => {
  try {
    const response: RoleListResponse = await horusService.listRoles()
    roles.value = response.items
  } catch (err) {
    console.error('Failed to fetch roles:', err)
  }
}

// Create department
const createDepartment = async () => {
  try {
    await horusService.createDepartment(newDepartment.value)
    showCreateDepartmentModal.value = false
    newDepartment.value = { name: '', description: '' }
    await fetchDepartments()
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to create department'
  }
}

// Edit department
const editDepartment = (department: DepartmentResponse) => {
  selectedDepartment.value = department
  // Implement edit functionality
}

// Delete department
const confirmDeleteDepartment = async (department: DepartmentResponse) => {
  if (confirm(`Are you sure you want to delete ${department.name}?`)) {
    try {
      await horusService.deleteDepartment(department.id)
      await fetchDepartments()
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to delete department'
    }
  }
}

// Manage department roles
const manageDepartmentRoles = async (department: DepartmentResponse) => {
  selectedDepartment.value = department
  showRolesModal.value = true
  try {
    const response: DepartmentRoleListResponse = await horusService.getDepartmentRoles(
      department.id
    )
    departmentRoles.value = response.items.map(item => {
      const role = roles.value.find(r => r.id === item.role_id)
      return (
        role || {
          id: item.role_id,
          name: 'Unknown Role',
          description: '',
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString()
        }
      )
    })
    // Filter out already assigned roles
    availableRoles.value = roles.value.filter(
      role => !departmentRoles.value.some(assignedRole => assignedRole.id === role.id)
    )
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to fetch department roles'
  }
}

// Assign role to department
const assignRole = async () => {
  if (!selectedDepartment.value || !selectedRoleId.value) return

  try {
    const roleData: AssignDepartmentRoleRequest = {
      role_id: selectedRoleId.value
    }
    await horusService.assignDepartmentRole(selectedDepartment.value.id, roleData)
    await manageDepartmentRoles(selectedDepartment.value)
    selectedRoleId.value = null
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to assign role'
  }
}

// Remove role from department
const removeRole = async (roleId: number) => {
  if (!selectedDepartment.value) return

  try {
    await horusService.removeDepartmentRole(selectedDepartment.value.id, roleId)
    await manageDepartmentRoles(selectedDepartment.value)
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Failed to remove role'
  }
}

// Initialize
onMounted(async () => {
  await Promise.all([fetchDepartments(), fetchRoles()])
})
</script>

<style scoped>
.rbac-departments-view {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  padding: 1.5rem;
}

.view-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
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

.create-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background-color: #4f46e5;
  color: white;
  border: none;
  border-radius: 6px;
  padding: 0.5rem 1rem;
  cursor: pointer;
  font-size: 0.875rem;
  transition: background-color 0.2s;
}

.create-button:hover {
  background-color: #4338ca;
}

.departments-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

.department-card {
  background: white;
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.department-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.department-name {
  font-size: 1.25rem;
  font-weight: 600;
  color: #333;
  margin: 0;
}

.department-actions {
  display: flex;
  gap: 0.5rem;
}

.action-button {
  background: none;
  border: none;
  padding: 0.25rem;
  cursor: pointer;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.action-button:hover {
  background-color: #f3f4f6;
}

.action-button.delete:hover {
  background-color: #fee2e2;
  color: #dc2626;
}

.department-description {
  color: #6b7280;
  margin: 0 0 1rem;
  font-size: 0.875rem;
}

.department-stats {
  display: flex;
  gap: 1rem;
  font-size: 0.875rem;
}

.stat-item {
  display: flex;
  gap: 0.5rem;
}

.stat-label {
  color: #6b7280;
}

.stat-value {
  font-weight: 500;
  color: #333;
}

/* Modal styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background: white;
  border-radius: 8px;
  padding: 2rem;
  width: 100%;
  max-width: 500px;
  max-height: 90vh;
  overflow-y: auto;
}

.modal-content h2 {
  margin: 0 0 1.5rem;
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #374151;
}

.form-group input,
.form-group textarea {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  font-size: 0.875rem;
}

.form-group textarea {
  min-height: 100px;
  resize: vertical;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 1.5rem;
}

.cancel-button,
.submit-button,
.close-button {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  font-size: 0.875rem;
  cursor: pointer;
  transition: background-color 0.2s;
}

.cancel-button {
  background: none;
  border: 1px solid #d1d5db;
  color: #374151;
}

.submit-button {
  background-color: #4f46e5;
  color: white;
  border: none;
}

.submit-button:hover {
  background-color: #4338ca;
}

/* Roles management styles */
.roles-management {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
}

.roles-list,
.assign-role {
  background: #f9fafb;
  padding: 1rem;
  border-radius: 6px;
}

.role-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem;
  background: white;
  border-radius: 4px;
  margin-bottom: 0.5rem;
}

.remove-role {
  background: none;
  border: none;
  color: #dc2626;
  cursor: pointer;
  font-size: 0.875rem;
}

.assign-role select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  margin-bottom: 1rem;
}

.assign-button {
  width: 100%;
  padding: 0.5rem;
  background-color: #4f46e5;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.assign-button:disabled {
  background-color: #9ca3af;
  cursor: not-allowed;
}

/* Loading and error states */
.loading-state,
.empty-state,
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
</style>
